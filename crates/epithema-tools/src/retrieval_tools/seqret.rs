//! `seqret` implementation.

use std::path::PathBuf;

use epithema_core::SequenceRecord;
use epithema_diagnostics::PlatformError;

/// Shared execution error for retrieval tools.
pub type ToolExecutionError = PlatformError;

/// Typed resolved source for `seqret`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SeqretSource {
    /// Local file normalization path.
    LocalPath(PathBuf),
    /// Provider-backed single-sequence retrieval path.
    Retrieved {
        /// Provider identity used for retrieval.
        provider: String,
        /// Requested accession or provider-local locator.
        accession: String,
    },
}

/// Typed parameters for `seqret`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SeqretParams {
    /// Resolved input source.
    pub source: SeqretSource,
    /// Parsed records to normalize.
    pub records: Vec<SequenceRecord>,
}

/// Structured `seqret` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SeqretOutcome {
    /// Resolved input source.
    pub source: SeqretSource,
    /// Normalized output records.
    pub records: Vec<SequenceRecord>,
}

/// Returns the `seqret` help text.
#[must_use]
pub fn seqret_help() -> &'static str {
    "Usage: epithema seqret <input>\n\nNormalize a local sequence file or retrieve a single provider-backed accession and emit normalized FASTA."
}

/// Executes `seqret`.
pub fn run_seqret(params: SeqretParams) -> Result<SeqretOutcome, ToolExecutionError> {
    Ok(SeqretOutcome {
        source: params.source,
        records: params.records,
    })
}
