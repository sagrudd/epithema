//! `refseqget` implementation.

use emboss_core::SequenceRecord;
use emboss_diagnostics::PlatformError;

/// Shared execution error for retrieval tools.
pub type ToolExecutionError = PlatformError;

/// Typed parameters for `refseqget`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RefseqgetParams {
    /// Provider identity used for retrieval.
    pub provider: String,
    /// Requested accession or provider-local locator.
    pub accession: String,
    /// Retrieved sequence record.
    pub record: SequenceRecord,
}

/// Structured `refseqget` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RefseqgetOutcome {
    /// Provider identity used for retrieval.
    pub provider: String,
    /// Requested accession or provider-local locator.
    pub accession: String,
    /// Retrieved sequence record.
    pub record: SequenceRecord,
}

/// Returns the `refseqget` help text.
#[must_use]
pub fn refseqget_help() -> &'static str {
    "Usage: emboss-rs refseqget <provider-qualified-accession>\n\nRetrieve a single provider-backed reference sequence through the governed acquisition seam and emit normalized FASTA."
}

/// Executes `refseqget`.
pub fn run_refseqget(params: RefseqgetParams) -> Result<RefseqgetOutcome, ToolExecutionError> {
    Ok(RefseqgetOutcome {
        provider: params.provider,
        accession: params.accession,
        record: params.record,
    })
}
