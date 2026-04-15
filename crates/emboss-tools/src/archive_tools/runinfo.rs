//! `runinfo` implementation.

use emboss_diagnostics::PlatformError;

/// Shared execution error for archive tools.
pub type ToolExecutionError = PlatformError;

/// Typed parameters for `runinfo`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuninfoParams {
    /// Provider used for metadata lookup.
    pub provider: String,
    /// Requested accession or provider-local locator.
    pub accession: String,
    /// Stable archive object-class label.
    pub object_class: String,
    /// Linked run accession when available.
    pub run_accession: Option<String>,
    /// Linked experiment accession when available.
    pub experiment_accession: Option<String>,
    /// Linked sample accession when available.
    pub sample_accession: Option<String>,
    /// Linked study accession when available.
    pub study_accession: Option<String>,
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
}

/// Structured `runinfo` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuninfoOutcome {
    /// Provider used for metadata lookup.
    pub provider: String,
    /// Requested accession or provider-local locator.
    pub accession: String,
    /// Stable archive object-class label.
    pub object_class: String,
    /// Linked run accession when available.
    pub run_accession: Option<String>,
    /// Linked experiment accession when available.
    pub experiment_accession: Option<String>,
    /// Linked sample accession when available.
    pub sample_accession: Option<String>,
    /// Linked study accession when available.
    pub study_accession: Option<String>,
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
}

/// Returns the `runinfo` help text.
#[must_use]
pub fn runinfo_help() -> &'static str {
    "Usage: emboss-rs runinfo <archive-accession>\n\nNormalize ENA or SRA archive metadata for one accession-backed archive object and emit a structured report."
}

/// Executes `runinfo`.
pub fn run_runinfo(params: RuninfoParams) -> Result<RuninfoOutcome, ToolExecutionError> {
    Ok(RuninfoOutcome {
        provider: params.provider,
        accession: params.accession,
        object_class: params.object_class,
        run_accession: params.run_accession,
        experiment_accession: params.experiment_accession,
        sample_accession: params.sample_accession,
        study_accession: params.study_accession,
        platform: params.platform,
        instrument_model: params.instrument_model,
        library_layout: params.library_layout,
        library_strategy: params.library_strategy,
        library_source: params.library_source,
    })
}
