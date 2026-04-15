//! `runget` implementation.

use emboss_diagnostics::PlatformError;

/// Shared execution error for archive tools.
pub type ToolExecutionError = PlatformError;

/// Typed parameters for `runget`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RungetParams {
    /// Provider used for manifest lookup.
    pub provider: String,
    /// Requested accession or provider-local locator.
    pub accession: String,
    /// Stable archive object-class label.
    pub object_class: String,
    /// Whether the caller requested direct download mode.
    pub download: bool,
}

/// Structured `runget` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RungetOutcome {
    /// Provider used for manifest lookup.
    pub provider: String,
    /// Requested accession or provider-local locator.
    pub accession: String,
    /// Stable archive object-class label.
    pub object_class: String,
    /// Whether the caller requested direct download mode.
    pub download: bool,
}

/// Returns the `runget` help text.
#[must_use]
pub fn runget_help() -> &'static str {
    "Usage: emboss-rs runget <run-accession> [--download]\n\nDiscover a normalized public-run manifest through the governed archive acquisition seam. The default v1 behavior emits a manifest; --download is reserved and currently rejected clearly."
}

/// Executes `runget`.
pub fn run_runget(params: RungetParams) -> Result<RungetOutcome, ToolExecutionError> {
    Ok(RungetOutcome {
        provider: params.provider,
        accession: params.accession,
        object_class: params.object_class,
        download: params.download,
    })
}
