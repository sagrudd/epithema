//! `ngsget` implementation.

use std::path::PathBuf;

use epithema_diagnostics::PlatformError;

/// Shared execution error for NGS archive tools.
pub type ToolExecutionError = PlatformError;

/// Typed parameters for `ngsget`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NgsgetParams {
    /// Requested study, sample, experiment, or run accession.
    pub accession: String,
    /// Provider selection: `auto`, `ena`, or `sra`.
    pub provider: String,
    /// Output root for materialized files and provenance.
    pub output_root: PathBuf,
    /// Whether raw/submitted assets were requested.
    pub include_raw: bool,
    /// Existing download roots searched before network retrieval.
    pub existing_download_roots: Vec<PathBuf>,
    /// Maximum number of concurrent direct-download workers.
    pub download_threads: usize,
    /// Direct-download transport mode: `https`, `auto`, or `aspera`.
    pub download_transport: String,
    /// Number of normalized runs in the manifest.
    pub run_count: usize,
    /// Number of selected assets.
    pub selected_asset_count: usize,
    /// Number of failed materialization records.
    pub failed_record_count: usize,
}

/// Structured `ngsget` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NgsgetOutcome {
    /// Requested study, sample, experiment, or run accession.
    pub accession: String,
    /// Provider used for manifest expansion.
    pub provider: String,
    /// Output root for materialized files and provenance.
    pub output_root: PathBuf,
    /// Whether raw/submitted assets were requested.
    pub include_raw: bool,
    /// Number of normalized runs in the manifest.
    pub run_count: usize,
    /// Number of selected assets.
    pub selected_asset_count: usize,
    /// Number of failed materialization records.
    pub failed_record_count: usize,
    /// Maximum number of concurrent direct-download workers.
    pub download_threads: usize,
    /// Direct-download transport mode.
    pub download_transport: String,
}

/// Returns the bounded `ngsget` help text.
#[must_use]
pub fn ngsget_help() -> &'static str {
    "Usage: epithema ngsget <accession> [--provider auto|ena|sra] [--out <dir>] [--raw] [--threads <n>] [--transport https|auto|aspera] [--ascp <path>] [--aspera-key <path>] [--aspera-rate <rate>] [--check-downloads <path>]\n\nMaterialize generated FASTQ assets for one public ENA or SRA study, sample, experiment, or run accession. Use --raw to include submitted raw/alignment files and provider-native SRA archives. Use --threads to allow 1 to 20 concurrent direct downloads; the default is 1. Use --transport aspera to download ENA-compatible public file URLs with IBM Aspera ascp, or --transport auto to use Aspera when ascp and an auth key are discoverable and otherwise fall back to HTTPS. Use --ascp to set the ascp executable, --aspera-key to set the ENA public Aspera auth key, and --aspera-rate to set the ascp target rate. Use --check-downloads to recursively search an existing download tree before network retrieval. The service copies verified matches into the output tree, leaves originals intact, resumes partial downloads when possible, and reports same-name checksum mismatches as failed materialization records."
}

/// Executes `ngsget`.
pub fn run_ngsget(params: NgsgetParams) -> Result<NgsgetOutcome, ToolExecutionError> {
    Ok(NgsgetOutcome {
        accession: params.accession,
        provider: params.provider,
        output_root: params.output_root,
        include_raw: params.include_raw,
        run_count: params.run_count,
        selected_asset_count: params.selected_asset_count,
        failed_record_count: params.failed_record_count,
        download_threads: params.download_threads,
        download_transport: params.download_transport,
    })
}
