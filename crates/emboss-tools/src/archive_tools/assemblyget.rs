//! `assemblyget` analytical core.

use emboss_diagnostics::{ErrorCategory, PlatformError};

/// Shared execution error for archive tools.
pub type ToolExecutionError = PlatformError;

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

/// Executes the bounded `assemblyget` analytical core.
pub fn run_assemblyget(
    params: AssemblygetParams,
) -> Result<AssemblygetOutcome, ToolExecutionError> {
    let (provider, accession) = parse_provider_qualified_query(&params.query)?;
    let object_class = require_non_empty(
        &params.object_class,
        "assemblyget requires a non-empty archive object class",
        "tools.assemblyget.object_class.empty",
    )?;
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
    use super::{AssemblygetMaterializationStatus, AssemblygetParams, run_assemblyget};

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
    fn normalizes_provider_and_trims_accession() {
        let mut params = base_params();
        params.query = " ENA:ERR123456 ".to_owned();

        let outcome = run_assemblyget(params).expect("provider-qualified input should parse");

        assert_eq!(outcome.provider, "ena");
        assert_eq!(outcome.accession, "ERR123456");
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
