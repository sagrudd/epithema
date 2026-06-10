//! Minimal Rust-callable bridge surface for future R wrappers.

use epithema_core::{Alignment, Feature, SequenceRecord};
use epithema_plot_contract::PlotPayload;
use epithema_service::{EpithemaService, MethodResult};

use crate::conversion::{
    project_alignment_summary, project_feature_summaries, project_health, project_plot_contract,
    project_sequence_summary, project_table_summary, project_version,
};
use crate::health::BridgeHealth;
use crate::types::{
    BridgeAlignmentSummary, BridgeFeatureSummary, BridgePlotContract, BridgePlotSummary,
    BridgeSequenceSummary, BridgeTableSummary, BridgeToolSummary,
};
use crate::version::BridgeVersion;

/// Returns stable version and platform metadata for the bridge surface.
#[must_use]
pub fn bridge_version() -> BridgeVersion {
    project_version()
}

/// Returns a bridge-facing health summary using the supplied service instance.
#[must_use]
pub fn health_check_with_service(service: &EpithemaService) -> BridgeHealth {
    project_health(service)
}

/// Returns a bridge-facing health summary using a default empty service runtime.
#[must_use]
pub fn health_check() -> BridgeHealth {
    health_check_with_service(&EpithemaService::empty())
}

/// Lists bridge-safe tool summaries from the supplied service instance.
#[must_use]
pub fn list_tools(service: &EpithemaService) -> Vec<BridgeToolSummary> {
    service
        .descriptors()
        .iter()
        .map(BridgeToolSummary::from)
        .collect()
}

/// Confirms that the bridge accepts Rust-side plot payload contracts for
/// handoff to the R-owned rendering layer.
#[must_use]
pub fn supports_plot_payload(_payload: &PlotPayload) -> bool {
    true
}

/// Projects a plot contract into bridge-safe JSON for the R plotting backend.
pub fn serialize_plot_contract(
    payload: &PlotPayload,
) -> Result<BridgePlotContract, epithema_diagnostics::PlatformError> {
    project_plot_contract(payload)
}

/// Projects a small summary of a plot contract.
#[must_use]
pub fn summarize_plot_contract(payload: &PlotPayload) -> BridgePlotSummary {
    BridgePlotSummary::from(payload)
}

/// Projects a sequence record into a bridge-safe summary.
#[must_use]
pub fn summarize_sequence(record: &SequenceRecord) -> BridgeSequenceSummary {
    project_sequence_summary(record)
}

/// Projects feature collections into bridge-safe summaries.
#[must_use]
pub fn summarize_features(features: &[Feature]) -> Vec<BridgeFeatureSummary> {
    project_feature_summaries(features)
}

/// Projects an alignment into a bridge-safe summary.
#[must_use]
pub fn summarize_alignment(alignment: &Alignment) -> BridgeAlignmentSummary {
    project_alignment_summary(alignment)
}

/// Projects a tabular method result into a bridge-safe table summary when applicable.
#[must_use]
pub fn summarize_table_result(result: &MethodResult) -> Option<BridgeTableSummary> {
    project_table_summary(result)
}

#[cfg(test)]
mod tests {
    use epithema_core::{
        Alignment, AlignmentRow, Feature, FeatureKind, FeatureLocation, Interval, MoleculeKind,
        SequenceIdentifier, SequenceRecord, Strand,
    };
    use epithema_plot_contract::PlotPayload;
    use epithema_service::{
        EpithemaService, ExecutionContext, ExecutionOutcome, ExecutionReport, InvocationOrigin,
        MethodResult, OutcomeStatus, ResultPayload, ResultSummary, ServiceRegistry, TableReport,
        ToolName,
    };
    use epithema_tools::ToolDescriptor;

    use super::{
        bridge_version, health_check, list_tools, summarize_alignment, summarize_features,
        summarize_plot_contract, summarize_sequence, summarize_table_result, supports_plot_payload,
    };

    #[test]
    fn exposes_bridge_version() {
        let version = bridge_version();
        assert_eq!(version.sister_package, "epithemaR");
    }

    #[test]
    fn reports_default_health() {
        let health = health_check();
        assert!(health.operation_status.ok);
        assert_eq!(health.tools_registered, 0);
    }

    #[test]
    fn lists_projected_tools() {
        let mut registry = ServiceRegistry::new();
        registry
            .register(ToolDescriptor::new("seqret", "sequence conversion"))
            .expect("tool registration should succeed");
        let service = EpithemaService::new(registry);

        let tools = list_tools(&service);
        assert_eq!(tools.len(), 1);
        assert_eq!(tools[0].name, "seqret");
    }

    #[test]
    fn accepts_plot_payload_contracts() {
        let payload = PlotPayload::empty("example");
        assert!(supports_plot_payload(&payload));
    }

    #[test]
    fn summarizes_plot_payload_contracts() {
        let payload = PlotPayload::empty("example");
        let summary = summarize_plot_contract(&payload);
        assert_eq!(summary.title, "example");
        assert_eq!(summary.kind, "line");
    }

    #[test]
    fn summarizes_sequence_record() {
        let record = SequenceRecord::new(
            SequenceIdentifier::new("seq1").expect("valid identifier"),
            MoleculeKind::Dna,
            "ACGT",
        )
        .expect("sequence should build");

        let summary = summarize_sequence(&record);
        assert_eq!(summary.identifier, "seq1");
        assert_eq!(summary.length, 4);
    }

    #[test]
    fn summarizes_features() {
        let feature = Feature::new(
            FeatureKind::Gene,
            FeatureLocation::new(
                Interval::new(1, 4).expect("valid interval"),
                Strand::Forward,
            ),
        )
        .with_name("geneA");

        let summaries = summarize_features(&[feature]);
        assert_eq!(summaries.len(), 1);
        assert_eq!(summaries[0].name.as_deref(), Some("geneA"));
    }

    #[test]
    fn summarizes_alignment() {
        let alignment = Alignment::new(vec![
            AlignmentRow::new(
                SequenceIdentifier::new("seq1").expect("valid identifier"),
                MoleculeKind::Dna,
                "AC-GT",
            )
            .expect("row should build"),
            AlignmentRow::new(
                SequenceIdentifier::new("seq2").expect("valid identifier"),
                MoleculeKind::Dna,
                "ACTGT",
            )
            .expect("row should build"),
        ])
        .expect("alignment should build");

        let summary = summarize_alignment(&alignment);
        assert!(summary.pairwise);
        assert_eq!(summary.column_count, 5);
    }

    #[test]
    fn summarizes_table_result_when_present() {
        let context = ExecutionContext::for_origin(InvocationOrigin::Cli);
        let report = ExecutionReport::from_context(
            &context,
            "epithema",
            env!("CARGO_PKG_VERSION"),
            ExecutionOutcome::new(OutcomeStatus::Succeeded).with_summary("ok"),
        );
        let result = MethodResult::new(
            ToolName::new("infoseq").expect("tool name should build"),
            ResultPayload::TableReport(TableReport::new(
                vec!["name".to_owned()],
                vec![vec!["seq1".to_owned()]],
            )),
            ResultSummary::new("Sequence table"),
            report,
        );

        let table = summarize_table_result(&result).expect("table summary should exist");
        assert_eq!(table.row_count, 1);
        assert_eq!(table.columns, vec!["name"]);
    }
}
