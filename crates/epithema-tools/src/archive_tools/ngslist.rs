//! `ngslist` implementation.

use epithema_diagnostics::PlatformError;

/// Shared execution error for NGS archive tools.
pub type ToolExecutionError = PlatformError;

/// Stable `ngslist` output format.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NgslistFormat {
    /// Stable tabular report.
    Table,
    /// Stable JSON text report.
    Json,
}

impl NgslistFormat {
    /// Returns the stable lowercase format label.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Table => "table",
            Self::Json => "json",
        }
    }
}

/// Typed parameters for `ngslist`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NgslistParams {
    /// Requested study, sample, experiment, or run accession.
    pub accession: String,
    /// Provider selection: `auto`, `ena`, or `sra`.
    pub provider: String,
    /// Output rendering format.
    pub format: NgslistFormat,
    /// Number of normalized runs in the manifest.
    pub run_count: usize,
    /// Number of asset rows in the manifest.
    pub asset_count: usize,
    /// Provider route endpoint used for manifest expansion.
    pub route_endpoint: String,
}

/// Structured `ngslist` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NgslistOutcome {
    /// Requested study, sample, experiment, or run accession.
    pub accession: String,
    /// Provider used for manifest expansion.
    pub provider: String,
    /// Output rendering format.
    pub format: NgslistFormat,
    /// Number of normalized runs in the manifest.
    pub run_count: usize,
    /// Number of asset rows in the manifest.
    pub asset_count: usize,
    /// Provider route endpoint used for manifest expansion.
    pub route_endpoint: String,
}

/// Returns the bounded `ngslist` help text.
#[must_use]
pub fn ngslist_help() -> &'static str {
    "Usage: epithema ngslist <accession> [--provider auto|ena|sra] [--format table|json]\n\nList generated FASTQ, provider-native SRA archives, submitted raw/alignment files, and related NGS assets associated with one public ENA or SRA study, sample, experiment, or run accession."
}

/// Executes `ngslist`.
pub fn run_ngslist(params: NgslistParams) -> Result<NgslistOutcome, ToolExecutionError> {
    Ok(NgslistOutcome {
        accession: params.accession,
        provider: params.provider,
        format: params.format,
        run_count: params.run_count,
        asset_count: params.asset_count,
        route_endpoint: params.route_endpoint,
    })
}
