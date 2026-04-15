//! `seqcount` implementation.

use super::{SequenceInput, ToolExecutionError, load_sequence_records};

/// Typed parameters for `seqcount`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SeqcountParams {
    /// Local sequence input path.
    pub input: SequenceInput,
}

/// Structured `seqcount` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SeqcountOutcome {
    /// Input path used for counting.
    pub input: SequenceInput,
    /// Deterministic record count.
    pub count: usize,
}

/// Returns the `seqcount` help text.
#[must_use]
pub fn seqcount_help() -> &'static str {
    "Usage: emboss-rs seqcount <input>\n\nCount sequence records in a local FASTA, FASTQ, EMBL, or GenBank file."
}

/// Executes `seqcount`.
pub fn run_seqcount(params: SeqcountParams) -> Result<SeqcountOutcome, ToolExecutionError> {
    let count = load_sequence_records(&params.input)?.len();
    Ok(SeqcountOutcome {
        input: params.input,
        count,
    })
}
