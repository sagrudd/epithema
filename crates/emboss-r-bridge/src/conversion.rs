//! Conversion helpers from internal workspace types to bridge-safe summaries.

use emboss_core::{Alignment, Feature, PLATFORM_IDENTITY, SequenceRecord};
use emboss_diagnostics::{Diagnostic, PlatformError};
use emboss_plot_contract::PlotSpec;
use emboss_service::{ArtifactReference, EmbossService, MethodResult, ResultPayload, TableReport};
use emboss_tools::ToolDescriptor;

use crate::error::BridgeErrorSummary;
use crate::health::BridgeHealth;
use crate::types::{
    BridgeAlignmentSummary, BridgeArtifactSummary, BridgeDiagnosticSummary, BridgeFeatureSummary,
    BridgeOperationStatus, BridgePlotContract, BridgePlotSummary, BridgeProvenanceSummary,
    BridgeResultSummary, BridgeSequenceSummary, BridgeTableSummary, BridgeToolSummary,
};
use crate::version::BridgeVersion;

impl From<ToolDescriptor> for BridgeToolSummary {
    fn from(value: ToolDescriptor) -> Self {
        Self {
            name: value.name.to_owned(),
            summary: value.summary.to_owned(),
        }
    }
}

impl From<&ToolDescriptor> for BridgeToolSummary {
    fn from(value: &ToolDescriptor) -> Self {
        Self::from(*value)
    }
}

impl From<&Diagnostic> for BridgeDiagnosticSummary {
    fn from(value: &Diagnostic) -> Self {
        Self {
            severity: value.severity.to_string(),
            code: value.code().map(ToOwned::to_owned),
            message: value.message().to_owned(),
            context: value.context().map(ToOwned::to_owned),
            location: value.location().map(|location| location.scope().to_owned()),
        }
    }
}

impl From<&emboss_diagnostics::ArtifactProvenance> for BridgeProvenanceSummary {
    fn from(value: &emboss_diagnostics::ArtifactProvenance) -> Self {
        Self {
            origin_kind: format!("{:?}", value.origin_kind).to_ascii_lowercase(),
            locator: value.locator().to_owned(),
            provider: value.provider().map(ToOwned::to_owned),
            description: value.description().map(ToOwned::to_owned),
        }
    }
}

impl From<&ArtifactReference> for BridgeArtifactSummary {
    fn from(value: &ArtifactReference) -> Self {
        Self {
            id: value.id.clone(),
            kind: format!("{:?}", value.kind).to_ascii_lowercase(),
            label: value.label.clone(),
            local_path: value
                .local_path
                .as_ref()
                .map(|path| path.display().to_string()),
            provenance: value.provenance.as_ref().map(BridgeProvenanceSummary::from),
        }
    }
}

impl From<&SequenceRecord> for BridgeSequenceSummary {
    fn from(value: &SequenceRecord) -> Self {
        Self {
            identifier: value.identifier().accession().to_owned(),
            display_name: value.identifier().display_name().map(ToOwned::to_owned),
            molecule: value.molecule().as_str().to_owned(),
            alphabet: value.alphabet().to_string(),
            length: value.len(),
            description: value.metadata().description.clone(),
            feature_count: value.features().len(),
        }
    }
}

impl From<&Feature> for BridgeFeatureSummary {
    fn from(value: &Feature) -> Self {
        let bounds = value.location.bounds();
        Self {
            kind: format!("{:?}", value.kind).to_ascii_lowercase(),
            name: value.name.clone(),
            start: bounds.start(),
            end: bounds.end(),
            strand: value.location.strand().map(|strand| strand.to_string()),
            span_count: value.location.spans().len(),
            qualifier_count: value.qualifiers.len(),
        }
    }
}

impl From<&Alignment> for BridgeAlignmentSummary {
    fn from(value: &Alignment) -> Self {
        Self {
            identifier: value.identifier().map(ToOwned::to_owned),
            row_count: value.row_count(),
            column_count: value.column_count(),
            pairwise: value.is_pairwise(),
            multiple: value.is_multiple(),
            row_identifiers: value
                .rows()
                .iter()
                .map(|row| row.identifier().accession().to_owned())
                .collect(),
        }
    }
}

impl From<&MethodResult> for BridgeResultSummary {
    fn from(value: &MethodResult) -> Self {
        Self {
            tool: value.tool.as_str().to_owned(),
            payload_kind: value.payload.kind_label().to_owned(),
            title: value.summary.title.clone(),
            lines: value.summary.lines.clone(),
            artifact_count: value.artifacts.len(),
            diagnostic_count: value.report.diagnostics().len(),
            plot_available: value.plot.is_some(),
        }
    }
}

impl From<&TableReport> for BridgeTableSummary {
    fn from(value: &TableReport) -> Self {
        Self {
            title: None,
            columns: value.columns.clone(),
            rows: value.rows.clone(),
            row_count: value.row_count(),
        }
    }
}

impl From<&PlotSpec> for BridgePlotSummary {
    fn from(value: &PlotSpec) -> Self {
        Self {
            id: value.metadata.id.clone(),
            title: value.metadata.title.clone(),
            kind: value.kind.as_str().to_owned(),
            series_count: value.series.len(),
        }
    }
}

/// Serializes a typed plot contract into a bridge-safe JSON handoff payload.
pub fn project_plot_contract(spec: &PlotSpec) -> Result<BridgePlotContract, PlatformError> {
    let json = spec.to_json_pretty().map_err(|error| {
        PlatformError::new(
            emboss_diagnostics::ErrorCategory::Validation,
            error.to_string(),
        )
        .with_code("bridge.plot_contract.invalid")
    })?;

    Ok(BridgePlotContract {
        summary: BridgePlotSummary::from(spec),
        json,
    })
}

/// Projects a compact table summary when the method result payload is tabular.
#[must_use]
pub fn project_table_summary(result: &MethodResult) -> Option<BridgeTableSummary> {
    match &result.payload {
        ResultPayload::TableReport(table) => Some(BridgeTableSummary::from(table)),
        _ => None,
    }
}

impl From<&PlatformError> for BridgeErrorSummary {
    fn from(value: &PlatformError) -> Self {
        Self {
            category: value.category().to_string(),
            code: value.code().map(ToOwned::to_owned),
            message: value.message().to_owned(),
            detail: value.detail().map(ToOwned::to_owned),
        }
    }
}

/// Projects stable version metadata for the bridge surface.
#[must_use]
pub fn project_version() -> BridgeVersion {
    BridgeVersion {
        package_version: env!("CARGO_PKG_VERSION").to_owned(),
        binary_name: PLATFORM_IDENTITY.binary_name.to_owned(),
        sister_package: PLATFORM_IDENTITY.sister_project.to_owned(),
        plot_backend: PLATFORM_IDENTITY.plot_backend.to_owned(),
    }
}

/// Projects bridge-facing health information from a shared service instance.
#[must_use]
pub fn project_health(service: &EmbossService) -> BridgeHealth {
    BridgeHealth {
        sister_package: PLATFORM_IDENTITY.sister_project.to_owned(),
        plot_backend: PLATFORM_IDENTITY.plot_backend.to_owned(),
        tools_registered: service.descriptors().len(),
        providers_configured: service.providers().len(),
        service_status: service.status_line(),
        operation_status: BridgeOperationStatus {
            ok: true,
            message: "Rust bridge scaffold is ready for future emboss-r bindings".to_owned(),
        },
    }
}

/// Projects a sequence summary into a bridge-safe owned form.
#[must_use]
pub fn project_sequence_summary(record: &SequenceRecord) -> BridgeSequenceSummary {
    BridgeSequenceSummary::from(record)
}

/// Projects feature summaries into bridge-safe owned forms.
#[must_use]
pub fn project_feature_summaries(features: &[Feature]) -> Vec<BridgeFeatureSummary> {
    features.iter().map(BridgeFeatureSummary::from).collect()
}

/// Projects an alignment summary into a bridge-safe owned form.
#[must_use]
pub fn project_alignment_summary(alignment: &Alignment) -> BridgeAlignmentSummary {
    BridgeAlignmentSummary::from(alignment)
}

#[cfg(test)]
mod tests {
    use emboss_core::{
        Alignment, AlignmentRow, Feature, FeatureKind, FeatureLocation, Interval, MoleculeKind,
        SequenceIdentifier, SequenceRecord, Strand,
    };
    use emboss_diagnostics::{
        Diagnostic, DiagnosticLocation, ErrorCategory, ExecutionContext, ExecutionOutcome,
        ExecutionReport, InvocationOrigin, OutcomeStatus, PlatformError, Severity,
    };
    use emboss_plot_contract::{
        AxisScaleHint, DataVector, GeometryHint, PlotAxis, PlotKind, PlotMetadata, PlotSeries,
        PlotSpec, SeriesStyle,
    };
    use emboss_service::{
        ArtifactKind, ArtifactReference, EmbossService, MethodResult, ResultPayload, ResultSummary,
        TableReport, ToolName,
    };
    use emboss_tools::ToolDescriptor;

    use super::{
        project_alignment_summary, project_feature_summaries, project_health,
        project_plot_contract, project_table_summary, project_version,
    };
    use crate::types::{
        BridgeDiagnosticSummary, BridgeFeatureSummary, BridgePlotSummary, BridgeResultSummary,
        BridgeSequenceSummary, BridgeToolSummary,
    };

    #[test]
    fn projects_version_metadata() {
        let version = project_version();
        assert_eq!(version.binary_name, "emboss-rs");
        assert_eq!(version.sister_package, "emboss-r");
        assert_eq!(version.plot_backend, "R");
    }

    #[test]
    fn converts_tool_descriptor_to_summary() {
        let summary = BridgeToolSummary::from(ToolDescriptor::new("needle", "global alignment"));
        assert_eq!(summary.name, "needle");
        assert_eq!(summary.summary, "global alignment");
    }

    #[test]
    fn converts_diagnostic_to_bridge_summary() {
        let diagnostic = Diagnostic::new(Severity::Warning, "missing provenance note")
            .with_code("bridge.provenance.missing")
            .with_context("autodoc import")
            .with_location(DiagnosticLocation::new("docs/source"));

        let summary = BridgeDiagnosticSummary::from(&diagnostic);
        assert_eq!(summary.severity, "warning");
        assert_eq!(summary.code.as_deref(), Some("bridge.provenance.missing"));
        assert_eq!(summary.location.as_deref(), Some("docs/source"));
    }

    #[test]
    fn converts_platform_error_to_bridge_summary() {
        let error = PlatformError::new(
            ErrorCategory::NotImplemented,
            "bridge method not implemented",
        )
        .with_code("bridge.method.not_implemented")
        .with_detail("tool dispatch projection is deferred");

        let summary = crate::error::BridgeErrorSummary::from(&error);
        assert_eq!(summary.category, "not-implemented");
        assert_eq!(
            summary.code.as_deref(),
            Some("bridge.method.not_implemented")
        );
    }

    #[test]
    fn projects_health_from_service() {
        let service = EmbossService::empty();
        let health = project_health(&service);
        assert_eq!(health.sister_package, "emboss-r");
        assert_eq!(health.providers_configured, 3);
        assert!(health.operation_status.ok);
    }

    #[test]
    fn projects_shared_method_result_summary() {
        let context = ExecutionContext::for_origin(InvocationOrigin::Cli);
        let report = ExecutionReport::from_context(
            &context,
            "emboss-rs",
            "0.1.0",
            ExecutionOutcome::new(OutcomeStatus::Succeeded).with_summary("ok"),
        );
        let result = MethodResult::new(
            ToolName::new("seqret").expect("tool name should build"),
            ResultPayload::Empty,
            ResultSummary::new("Sequence result").with_line("Length: 4"),
            report,
        );

        let summary = BridgeResultSummary::from(&result);
        assert_eq!(summary.tool, "seqret");
        assert_eq!(summary.payload_kind, "empty");
        assert_eq!(summary.title, "Sequence result");
        assert_eq!(summary.lines, vec!["Length: 4"]);
        assert!(!summary.plot_available);
    }

    #[test]
    fn projects_sequence_summary() {
        let sequence = SequenceRecord::new(
            SequenceIdentifier::new("seq1").expect("valid identifier"),
            MoleculeKind::Dna,
            "ACGT",
        )
        .expect("sequence should build");

        let summary = BridgeSequenceSummary::from(&sequence);
        assert_eq!(summary.identifier, "seq1");
        assert_eq!(summary.molecule, "dna");
        assert_eq!(summary.length, 4);
    }

    #[test]
    fn projects_feature_summaries() {
        let feature = Feature::new(
            FeatureKind::Gene,
            FeatureLocation::new(
                Interval::new(2, 6).expect("valid interval"),
                Strand::Forward,
            ),
        )
        .with_name("geneA")
        .with_qualifier("product", "enzyme");

        let summaries = project_feature_summaries(&[feature]);
        assert_eq!(summaries.len(), 1);
        let summary: &BridgeFeatureSummary = &summaries[0];
        assert_eq!(summary.name.as_deref(), Some("geneA"));
        assert_eq!(summary.start, 2);
        assert_eq!(summary.end, 6);
    }

    #[test]
    fn projects_alignment_summary() {
        let alignment = Alignment::with_identifier(
            Some("pairwise"),
            vec![
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
            ],
        )
        .expect("alignment should build");

        let summary = project_alignment_summary(&alignment);
        assert_eq!(summary.identifier.as_deref(), Some("pairwise"));
        assert!(summary.pairwise);
        assert_eq!(summary.row_identifiers, vec!["seq1", "seq2"]);
    }

    #[test]
    fn projects_table_summary_from_method_result() {
        let context = ExecutionContext::for_origin(InvocationOrigin::Cli);
        let report = ExecutionReport::from_context(
            &context,
            "emboss-rs",
            "0.1.0",
            ExecutionOutcome::new(OutcomeStatus::Succeeded).with_summary("ok"),
        );
        let table = TableReport::new(
            vec!["name".to_owned(), "length".to_owned()],
            vec![vec!["seq1".to_owned(), "4".to_owned()]],
        );
        let result = MethodResult::new(
            ToolName::new("infoseq").expect("tool name should build"),
            ResultPayload::TableReport(table),
            ResultSummary::new("Sequence table").with_line("Rows: 1"),
            report,
        )
        .with_artifact(
            ArtifactReference::new("summary-table", ArtifactKind::Table)
                .with_label("Summary table"),
        );

        let summary = project_table_summary(&result).expect("table summary should project");
        let result_summary = BridgeResultSummary::from(&result);
        assert_eq!(summary.row_count, 1);
        assert_eq!(summary.columns, vec!["name", "length"]);
        assert_eq!(result_summary.artifact_count, 1);
    }

    #[test]
    fn projects_plot_contract_into_bridge_json() {
        let plot = PlotSpec::new(
            PlotKind::Line,
            PlotMetadata::new("gc_profile", "GC profile"),
            PlotAxis::new("Window").with_scale_hint(AxisScaleHint::Linear),
            PlotAxis::new("GC").with_scale_hint(AxisScaleHint::Linear),
            vec![
                PlotSeries::new(
                    "sample",
                    "Sample",
                    DataVector::Numeric(vec![1.0, 2.0, 3.0]),
                    vec![0.4, 0.5, 0.45],
                )
                .with_style(SeriesStyle::empty().with_geometry_hint(GeometryHint::Line)),
            ],
        );

        let handoff = project_plot_contract(&plot).expect("plot contract should serialize");
        assert_eq!(
            handoff.summary,
            BridgePlotSummary {
                id: "gc_profile".to_owned(),
                title: "GC profile".to_owned(),
                kind: "line".to_owned(),
                series_count: 1,
            }
        );
        assert!(handoff.json.contains("\"kind\": \"line\""));
    }
}
