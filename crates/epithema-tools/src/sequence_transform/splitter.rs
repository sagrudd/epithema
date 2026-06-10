//! `splitter` implementation.

use epithema_core::SequenceRecord;
use epithema_diagnostics::{ErrorCategory, PlatformError};

use crate::sequence_stream::{SequenceInput, ToolExecutionError, load_sequence_records};

/// Typed parameters for `splitter`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SplitterParams {
    /// Local sequence input path.
    pub input: SequenceInput,
    /// Number of records per partition.
    pub chunk_size: usize,
}

/// Structured `splitter` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SplitterOutcome {
    /// Source input.
    pub input: SequenceInput,
    /// Chunk size used.
    pub chunk_size: usize,
    /// Deterministic partitions in input order.
    pub partitions: Vec<Vec<SequenceRecord>>,
}

/// Returns `splitter` help text.
#[must_use]
pub fn splitter_help() -> &'static str {
    "Usage: epithema splitter <input> <chunk-size>\n\nPartition a local sequence input into fixed-size chunks of N records each. The final partition may be smaller."
}

/// Executes `splitter`.
pub fn run_splitter(params: SplitterParams) -> Result<SplitterOutcome, ToolExecutionError> {
    if params.chunk_size == 0 {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            "splitter requires chunk size >= 1",
        )
        .with_code("tools.splitter.chunk_size.invalid"));
    }

    let records = load_sequence_records(&params.input)?;
    let mut partitions = Vec::new();
    for chunk in records.chunks(params.chunk_size) {
        partitions.push(chunk.to_vec());
    }

    Ok(SplitterOutcome {
        input: params.input,
        chunk_size: params.chunk_size,
        partitions,
    })
}
