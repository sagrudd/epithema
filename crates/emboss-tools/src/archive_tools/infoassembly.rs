//! `infoassembly` implementation.

use emboss_diagnostics::{ErrorCategory, PlatformError};

/// Shared execution error for archive tools.
pub type ToolExecutionError = PlatformError;

/// Typed parameters for `infoassembly`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InfoassemblyParams {
    /// Provider used for metadata lookup.
    pub provider: String,
    /// Requested accession or provider-local locator.
    pub accession: String,
    /// Stable archive object-class label.
    pub object_class: String,
    /// Canonical study- or project-level identifier when available.
    pub assembly_accession: Option<String>,
    /// Linked run accession when available.
    pub run_accession: Option<String>,
    /// Linked experiment accession when available.
    pub experiment_accession: Option<String>,
    /// Linked sample accession when available.
    pub sample_accession: Option<String>,
    /// Sequencing platform when available.
    pub platform: Option<String>,
    /// Instrument model when available.
    pub instrument_model: Option<String>,
    /// Library layout when available.
    pub library_layout: Option<String>,
    /// Library strategy when available.
    pub library_strategy: Option<String>,
    /// Library source when available.
    pub library_source: Option<String>,
    /// Count of normalized public files exposed by the provider route.
    pub file_count: usize,
    /// Aggregate byte size across normalized public files when known.
    pub total_size_bytes: Option<u64>,
    /// Stable provider-local route label used for the lookup.
    pub route_endpoint: String,
}

/// Structured `infoassembly` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InfoassemblyOutcome {
    /// Provider used for metadata lookup.
    pub provider: String,
    /// Requested accession or provider-local locator.
    pub accession: String,
    /// Stable archive object-class label.
    pub object_class: String,
    /// Canonical assembly/study identifier selected for reporting.
    pub assembly_accession: String,
    /// Linked run accession when available.
    pub run_accession: Option<String>,
    /// Linked experiment accession when available.
    pub experiment_accession: Option<String>,
    /// Linked sample accession when available.
    pub sample_accession: Option<String>,
    /// Sequencing platform when available.
    pub platform: Option<String>,
    /// Instrument model when available.
    pub instrument_model: Option<String>,
    /// Library layout when available.
    pub library_layout: Option<String>,
    /// Library strategy when available.
    pub library_strategy: Option<String>,
    /// Library source when available.
    pub library_source: Option<String>,
    /// Count of normalized public files exposed by the provider route.
    pub file_count: usize,
    /// Aggregate byte size across normalized public files when known.
    pub total_size_bytes: Option<u64>,
    /// Stable provider-local route label used for the lookup.
    pub route_endpoint: String,
}

/// Returns the `infoassembly` help text.
#[must_use]
pub fn infoassembly_help() -> &'static str {
    "Usage: emboss-rs infoassembly <archive-accession>\n\nNormalize provider-backed archive metadata into a bounded assembly-first report."
}

/// Executes `infoassembly`.
pub fn run_infoassembly(
    params: InfoassemblyParams,
) -> Result<InfoassemblyOutcome, ToolExecutionError> {
    let provider = params.provider.trim();
    if provider.is_empty() {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            "infoassembly requires a non-empty provider label",
        )
        .with_code("tools.infoassembly.provider.empty"));
    }

    let accession = params.accession.trim();
    if accession.is_empty() {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            "infoassembly requires a non-empty archive accession",
        )
        .with_code("tools.infoassembly.accession.empty"));
    }

    let object_class = params.object_class.trim();
    if object_class.is_empty() {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            "infoassembly requires a non-empty archive object class",
        )
        .with_code("tools.infoassembly.object_class.empty"));
    }

    let route_endpoint = params.route_endpoint.trim();
    if route_endpoint.is_empty() {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            "infoassembly requires a non-empty archive route endpoint label",
        )
        .with_code("tools.infoassembly.route.empty"));
    }

    let assembly_accession = params
        .assembly_accession
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
        .or_else(|| {
            if object_class.eq_ignore_ascii_case("study")
                || object_class.eq_ignore_ascii_case("project")
            {
                Some(accession.to_owned())
            } else {
                None
            }
        })
        .ok_or_else(|| {
            PlatformError::new(
                ErrorCategory::Validation,
                "infoassembly requires a study/project identifier or linked study accession",
            )
            .with_code("tools.infoassembly.assembly_accession_missing")
        })?;

    Ok(InfoassemblyOutcome {
        provider: provider.to_owned(),
        accession: accession.to_owned(),
        object_class: object_class.to_owned(),
        assembly_accession,
        run_accession: params.run_accession,
        experiment_accession: params.experiment_accession,
        sample_accession: params.sample_accession,
        platform: params.platform,
        instrument_model: params.instrument_model,
        library_layout: params.library_layout,
        library_strategy: params.library_strategy,
        library_source: params.library_source,
        file_count: params.file_count,
        total_size_bytes: params.total_size_bytes,
        route_endpoint: route_endpoint.to_owned(),
    })
}

#[cfg(test)]
mod tests {
    use super::{InfoassemblyParams, run_infoassembly};

    fn base_params() -> InfoassemblyParams {
        InfoassemblyParams {
            provider: "ena".to_owned(),
            accession: "ERR123456".to_owned(),
            object_class: "run".to_owned(),
            assembly_accession: Some("ERP000001".to_owned()),
            run_accession: Some("ERR123456".to_owned()),
            experiment_accession: Some("ERX000001".to_owned()),
            sample_accession: Some("ERS000001".to_owned()),
            platform: Some("ILLUMINA".to_owned()),
            instrument_model: Some("NovaSeq 6000".to_owned()),
            library_layout: Some("PAIRED".to_owned()),
            library_strategy: Some("WGS".to_owned()),
            library_source: Some("GENOMIC".to_owned()),
            file_count: 2,
            total_size_bytes: Some(22),
            route_endpoint: "ena.portal.filereport".to_owned(),
        }
    }

    #[test]
    fn preserves_linked_study_identifier_as_assembly_accession() {
        let outcome = run_infoassembly(base_params()).expect("infoassembly should succeed");

        assert_eq!(outcome.provider, "ena");
        assert_eq!(outcome.accession, "ERR123456");
        assert_eq!(outcome.object_class, "run");
        assert_eq!(outcome.assembly_accession, "ERP000001");
        assert_eq!(outcome.file_count, 2);
        assert_eq!(outcome.total_size_bytes, Some(22));
        assert_eq!(outcome.route_endpoint, "ena.portal.filereport");
    }

    #[test]
    fn falls_back_to_requested_accession_for_study_objects() {
        let mut params = base_params();
        params.object_class = "study".to_owned();
        params.accession = "ERP000001".to_owned();
        params.assembly_accession = None;

        let outcome = run_infoassembly(params).expect("study accessions should succeed");

        assert_eq!(outcome.assembly_accession, "ERP000001");
    }

    #[test]
    fn rejects_non_study_records_without_linked_study_identifier() {
        let mut params = base_params();
        params.assembly_accession = None;

        let error = run_infoassembly(params).expect_err("missing assembly identifier should fail");

        assert_eq!(
            error.code(),
            Some("tools.infoassembly.assembly_accession_missing")
        );
    }
}
