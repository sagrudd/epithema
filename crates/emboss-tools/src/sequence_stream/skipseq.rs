//! `skipseq` implementation.

use super::{SequenceInput, ToolExecutionError, load_sequence_records};
use emboss_core::SequenceRecord;

/// Typed parameters for `skipseq`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SkipseqParams {
    /// Local sequence input path.
    pub input: SequenceInput,
    /// Number of leading records to skip.
    pub count: usize,
}

/// Structured `skipseq` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SkipseqOutcome {
    /// Input path used for selection.
    pub input: SequenceInput,
    /// Number of leading records skipped.
    pub skipped_count: usize,
    /// Total record count in the input.
    pub total_count: usize,
    /// Remaining records after skipping.
    pub records: Vec<SequenceRecord>,
}

/// Returns the `skipseq` help text.
#[must_use]
pub fn skipseq_help() -> &'static str {
    "Usage: emboss-rs skipseq <input> <count>\n\nSkip the first N sequence records from a local FASTA, FASTQ, EMBL, or GenBank file and return the remainder."
}

/// Executes `skipseq`.
pub fn run_skipseq(params: SkipseqParams) -> Result<SkipseqOutcome, ToolExecutionError> {
    let mut records = load_sequence_records(&params.input)?;
    let total_count = records.len();
    let skipped_count = params.count.min(total_count);
    let remaining = records.split_off(skipped_count);

    Ok(SkipseqOutcome {
        input: params.input,
        skipped_count,
        total_count,
        records: remaining,
    })
}
