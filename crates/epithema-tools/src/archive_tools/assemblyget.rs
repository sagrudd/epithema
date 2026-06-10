//! `assemblyget` analytical core.

use epithema_diagnostics::{ErrorCategory, PlatformError};

/// Shared execution error for archive tools.
pub type ToolExecutionError = PlatformError;

/// Stable table columns for bounded `assemblyget` manifest/routing reports.
pub const ASSEMBLYGET_REPORT_COLUMNS: [&str; 10] = [
    "provider",
    "requested_accession",
    "object_class",
    "assembly_accession",
    "run_accession",
    "route_endpoint",
    "manifest_mode",
    "file_count",
    "total_size_bytes",
    "materialization_status",
];

/// Explicit materialization state for the bounded `assemblyget` slice.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AssemblygetMaterializationStatus {
    /// The method reports assembly manifest/routing intent only.
    NotMaterialized,
}

impl AssemblygetMaterializationStatus {
    /// Returns the stable report label for the materialization state.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::NotMaterialized => "not_materialized",
        }
    }
}

/// Typed parameters for the bounded `assemblyget` analytical core.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AssemblygetParams {
    /// Provider-qualified archive or assembly accession.
    pub query: String,
    /// Stable archive object-class label.
    pub object_class: String,
    /// Canonical assembly, study, or project identifier selected for reporting.
    pub assembly_accession: String,
    /// Linked run accession when available.
    pub run_accession: Option<String>,
    /// Stable provider-local route label used for the manifest lookup.
    pub route_endpoint: String,
    /// Stable manifest mode label.
    pub manifest_mode: String,
    /// Count of normalized manifest rows exposed by the provider route.
    pub file_count: usize,
    /// Aggregate byte size across manifest rows when known.
    pub total_size_bytes: Option<u64>,
}

/// Structured `assemblyget` analytical outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AssemblygetOutcome {
    /// Provider used for manifest/routing lookup.
    pub provider: String,
    /// Requested accession or provider-local locator.
    pub accession: String,
    /// Stable archive object-class label.
    pub object_class: String,
    /// Canonical assembly, study, or project identifier selected for reporting.
    pub assembly_accession: String,
    /// Linked run accession when available.
    pub run_accession: Option<String>,
    /// Stable provider-local route label used for the manifest lookup.
    pub route_endpoint: String,
    /// Stable manifest mode label.
    pub manifest_mode: String,
    /// Count of normalized manifest rows exposed by the provider route.
    pub file_count: usize,
    /// Aggregate byte size across manifest rows when known.
    pub total_size_bytes: Option<u64>,
    /// Explicit no-download/no-write materialization state.
    pub materialization_status: AssemblygetMaterializationStatus,
}

impl AssemblygetOutcome {
    /// Returns stable report columns for table-first `assemblyget` rendering.
    #[must_use]
    pub fn report_columns(&self) -> Vec<String> {
        ASSEMBLYGET_REPORT_COLUMNS
            .iter()
            .map(|column| (*column).to_owned())
            .collect()
    }

    /// Projects the deterministic outcome into one stable table row.
    #[must_use]
    pub fn report_row(&self) -> Vec<String> {
        vec![
            self.provider.clone(),
            self.accession.clone(),
            self.object_class.clone(),
            self.assembly_accession.clone(),
            self.run_accession.clone().unwrap_or_else(|| "-".to_owned()),
            self.route_endpoint.clone(),
            self.manifest_mode.clone(),
            self.file_count.to_string(),
            self.total_size_bytes
                .map(|size| size.to_string())
                .unwrap_or_else(|| "-".to_owned()),
            self.materialization_status.as_str().to_owned(),
        ]
    }

    /// Projects the deterministic outcome into stable table-first report rows.
    #[must_use]
    pub fn report_rows(&self) -> Vec<Vec<String>> {
        vec![self.report_row()]
    }

    /// Renders a stable tab-separated manifest/routing report.
    #[must_use]
    pub fn render_tsv_report(&self) -> String {
        let mut rendered = self.report_columns().join("\t");
        for row in self.report_rows() {
            rendered.push('\n');
            rendered.push_str(&row.join("\t"));
        }
        rendered
    }
}

/// Returns the bounded `assemblyget` help text.
#[must_use]
pub fn assemblyget_help() -> &'static str {
    "Usage: epithema assemblyget <provider-qualified-archive-accession>\n\nReport deterministic assembly-level manifest intent for one provider-qualified ENA or SRA archive accession. The v1 seam resolves archive metadata through the governed provider route, emits a stable table report, and does not download, stage, unpack, index, or write archive files."
}

/// Executes the bounded `assemblyget` analytical core.
pub fn run_assemblyget(
    params: AssemblygetParams,
) -> Result<AssemblygetOutcome, ToolExecutionError> {
    let (provider, accession) = parse_provider_qualified_query(&params.query)?;
    ensure_supported_provider(&provider)?;
    let object_class = normalize_object_class(&params.object_class)?;
    let assembly_accession = require_non_empty(
        &params.assembly_accession,
        "assemblyget requires a selected assembly, study, or project accession",
        "tools.assemblyget.assembly_accession.empty",
    )?;
    let route_endpoint = require_non_empty(
        &params.route_endpoint,
        "assemblyget requires a non-empty route endpoint label",
        "tools.assemblyget.route.empty",
    )?;
    let manifest_mode = require_non_empty(
        &params.manifest_mode,
        "assemblyget requires a non-empty manifest mode label",
        "tools.assemblyget.manifest_mode.empty",
    )?;

    Ok(AssemblygetOutcome {
        provider,
        accession,
        object_class,
        assembly_accession,
        run_accession: params.run_accession,
        route_endpoint,
        manifest_mode,
        file_count: params.file_count,
        total_size_bytes: params.total_size_bytes,
        materialization_status: AssemblygetMaterializationStatus::NotMaterialized,
    })
}

fn parse_provider_qualified_query(query: &str) -> Result<(String, String), ToolExecutionError> {
    let query = query.trim();
    if query.is_empty() {
        return Err(validation_error(
            "assemblyget requires one provider-qualified archive or assembly accession",
            "tools.assemblyget.query.empty",
        ));
    }
    if query.starts_with('>') || query.contains('\n') || query.contains('\r') {
        return Err(validation_error(
            "assemblyget does not accept inline sequence or payload literals",
            "tools.assemblyget.query.inline_payload",
        ));
    }
    if query.contains(',') || query.split_whitespace().count() > 1 {
        return Err(validation_error(
            "assemblyget accepts exactly one provider-qualified accession",
            "tools.assemblyget.query.multiple",
        ));
    }
    if is_local_file_reference(query) {
        return Err(validation_error(
            "assemblyget does not accept local file references",
            "tools.assemblyget.query.local_file",
        ));
    }

    let Some((provider, accession)) = query.split_once(':') else {
        return Err(validation_error(
            "assemblyget requires a provider-qualified query such as ena:ERR123456",
            "tools.assemblyget.query.unqualified",
        ));
    };
    let provider = provider.trim().to_ascii_lowercase();
    let accession = accession.trim();
    if provider.is_empty() || accession.is_empty() {
        return Err(validation_error(
            "assemblyget requires non-empty provider and accession fields",
            "tools.assemblyget.query.malformed",
        ));
    }
    if is_local_file_reference(accession) {
        return Err(validation_error(
            "assemblyget does not accept provider-local file references",
            "tools.assemblyget.query.local_file",
        ));
    }

    Ok((provider, accession.to_owned()))
}

fn ensure_supported_provider(provider: &str) -> Result<(), ToolExecutionError> {
    match provider {
        "ena" | "sra" => Ok(()),
        _ => Err(validation_error(
            "assemblyget supports only bounded ENA and SRA manifest/routing reports",
            "tools.assemblyget.provider.unsupported",
        )
        .with_detail(format!("received '{provider}'"))),
    }
}

fn normalize_object_class(value: &str) -> Result<String, ToolExecutionError> {
    let object_class = require_non_empty(
        value,
        "assemblyget requires a non-empty archive object class",
        "tools.assemblyget.object_class.empty",
    )?;
    let normalized = object_class.to_ascii_lowercase();

    match normalized.as_str() {
        "assembly" | "project" | "run" | "study" => Ok(normalized),
        _ => Err(validation_error(
            "assemblyget supports only assembly, project, study, and run object classes",
            "tools.assemblyget.object_class.unsupported",
        )
        .with_detail(format!("received '{object_class}'"))),
    }
}

fn require_non_empty(
    value: &str,
    message: &'static str,
    code: &'static str,
) -> Result<String, ToolExecutionError> {
    let value = value.trim();
    if value.is_empty() {
        Err(validation_error(message, code))
    } else {
        Ok(value.to_owned())
    }
}

fn is_local_file_reference(value: &str) -> bool {
    value.starts_with('/')
        || value.starts_with("./")
        || value.starts_with("../")
        || value.contains('/')
        || value.contains('\\')
}

fn validation_error(message: &'static str, code: &'static str) -> ToolExecutionError {
    PlatformError::new(ErrorCategory::Validation, message).with_code(code)
}

#[cfg(test)]
mod tests {
    use super::{
        ASSEMBLYGET_REPORT_COLUMNS, AssemblygetMaterializationStatus, AssemblygetParams,
        run_assemblyget,
    };

    fn base_params() -> AssemblygetParams {
        AssemblygetParams {
            query: "ena:ERR123456".to_owned(),
            object_class: "run".to_owned(),
            assembly_accession: "ERP000001".to_owned(),
            run_accession: Some("ERR123456".to_owned()),
            route_endpoint: "ena.portal.filereport".to_owned(),
            manifest_mode: "manifest_intent_only".to_owned(),
            file_count: 2,
            total_size_bytes: Some(22),
        }
    }

    #[test]
    fn builds_bounded_manifest_routing_outcome() {
        let outcome = run_assemblyget(base_params()).expect("assemblyget core should succeed");

        assert_eq!(outcome.provider, "ena");
        assert_eq!(outcome.accession, "ERR123456");
        assert_eq!(outcome.object_class, "run");
        assert_eq!(outcome.assembly_accession, "ERP000001");
        assert_eq!(outcome.route_endpoint, "ena.portal.filereport");
        assert_eq!(outcome.manifest_mode, "manifest_intent_only");
        assert_eq!(outcome.file_count, 2);
        assert_eq!(outcome.total_size_bytes, Some(22));
        assert_eq!(
            outcome.materialization_status,
            AssemblygetMaterializationStatus::NotMaterialized
        );
        assert_eq!(outcome.materialization_status.as_str(), "not_materialized");
    }

    #[test]
    fn exposes_stable_report_columns_and_row() {
        let outcome = run_assemblyget(base_params()).expect("assemblyget core should succeed");

        assert_eq!(
            outcome.report_columns(),
            ASSEMBLYGET_REPORT_COLUMNS
                .iter()
                .map(|column| (*column).to_owned())
                .collect::<Vec<_>>()
        );
        assert_eq!(
            outcome.report_rows(),
            vec![vec![
                "ena".to_owned(),
                "ERR123456".to_owned(),
                "run".to_owned(),
                "ERP000001".to_owned(),
                "ERR123456".to_owned(),
                "ena.portal.filereport".to_owned(),
                "manifest_intent_only".to_owned(),
                "2".to_owned(),
                "22".to_owned(),
                "not_materialized".to_owned(),
            ]]
        );
    }

    #[test]
    fn renders_manifest_intent_only_no_materialization_report() {
        let outcome = run_assemblyget(base_params()).expect("assemblyget core should succeed");

        assert_eq!(
            outcome.render_tsv_report(),
            "provider\trequested_accession\tobject_class\tassembly_accession\trun_accession\troute_endpoint\tmanifest_mode\tfile_count\ttotal_size_bytes\tmaterialization_status\n\
             ena\tERR123456\trun\tERP000001\tERR123456\tena.portal.filereport\tmanifest_intent_only\t2\t22\tnot_materialized"
        );
    }

    #[test]
    fn uses_placeholders_for_absent_optional_report_values() {
        let mut params = base_params();
        params.run_accession = None;
        params.total_size_bytes = None;

        let outcome = run_assemblyget(params).expect("missing optional values should report");

        assert_eq!(outcome.report_row()[4], "-");
        assert_eq!(outcome.report_row()[8], "-");
        assert_eq!(
            outcome.materialization_status,
            AssemblygetMaterializationStatus::NotMaterialized
        );
    }

    #[test]
    fn normalizes_provider_and_trims_accession() {
        let mut params = base_params();
        params.query = " ENA:ERR123456 ".to_owned();

        let outcome = run_assemblyget(params).expect("provider-qualified input should parse");

        assert_eq!(outcome.provider, "ena");
        assert_eq!(outcome.accession, "ERR123456");
    }

    #[test]
    fn normalizes_supported_object_class() {
        let mut params = base_params();
        params.object_class = " Study ".to_owned();

        let outcome = run_assemblyget(params).expect("supported object class should normalize");

        assert_eq!(outcome.object_class, "study");
    }

    #[test]
    fn rejects_unsupported_providers() {
        let mut params = base_params();
        params.query = "uniprot:P12345".to_owned();

        let error = run_assemblyget(params).expect_err("unsupported providers should fail");

        assert_eq!(error.code(), Some("tools.assemblyget.provider.unsupported"));
    }

    #[test]
    fn rejects_unsupported_object_classes() {
        let mut params = base_params();
        params.object_class = "sample".to_owned();

        let error = run_assemblyget(params).expect_err("unsupported object classes should fail");

        assert_eq!(
            error.code(),
            Some("tools.assemblyget.object_class.unsupported")
        );
    }

    #[test]
    fn rejects_unqualified_accessions() {
        let mut params = base_params();
        params.query = "ERR123456".to_owned();

        let error = run_assemblyget(params).expect_err("bare accessions should fail");

        assert_eq!(error.code(), Some("tools.assemblyget.query.unqualified"));
    }

    #[test]
    fn rejects_local_file_references() {
        let mut params = base_params();
        params.query = "./reads.fastq".to_owned();

        let error = run_assemblyget(params).expect_err("local files should fail");

        assert_eq!(error.code(), Some("tools.assemblyget.query.local_file"));
    }

    #[test]
    fn rejects_inline_payload_literals() {
        let mut params = base_params();
        params.query = ">read\nACGT".to_owned();

        let error = run_assemblyget(params).expect_err("inline payloads should fail");

        assert_eq!(error.code(), Some("tools.assemblyget.query.inline_payload"));
    }

    #[test]
    fn rejects_multi_accession_batches() {
        let mut params = base_params();
        params.query = "ena:ERR123456 ena:ERR654321".to_owned();

        let error = run_assemblyget(params).expect_err("batch inputs should fail");

        assert_eq!(error.code(), Some("tools.assemblyget.query.multiple"));
    }
}
