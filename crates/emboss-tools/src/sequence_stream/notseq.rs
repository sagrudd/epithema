//! `notseq` implementation.

use emboss_core::SequenceRecord;
use emboss_diagnostics::{ErrorCategory, PlatformError};

use super::{SequenceInput, ToolExecutionError, load_sequence_records};

/// Typed parameters for `notseq`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NotseqParams {
    /// Local sequence input path.
    pub input: SequenceInput,
    /// 1-based record index to exclude.
    pub excluded_index: usize,
}

/// Structured `notseq` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NotseqOutcome {
    /// Input path used for exclusion.
    pub input: SequenceInput,
    /// 1-based excluded index.
    pub excluded_index: usize,
    /// Total record count in the input.
    pub total_count: usize,
    /// Remaining records after exclusion.
    pub records: Vec<SequenceRecord>,
}

/// Returns the `notseq` help text.
#[must_use]
pub fn notseq_help() -> &'static str {
    "Usage: emboss-rs notseq <input> <index>\n\nReturn all sequence records except the 1-based excluded index from a local FASTA, FASTQ, EMBL, or GenBank file."
}

/// Executes `notseq`.
pub fn run_notseq(params: NotseqParams) -> Result<NotseqOutcome, ToolExecutionError> {
    if params.excluded_index == 0 {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            "excluded sequence index must be 1 or greater",
        )
        .with_code("tools.notseq.index.invalid"));
    }

    let mut records = load_sequence_records(&params.input)?;
    let total_count = records.len();
    if params.excluded_index > total_count {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!(
                "excluded sequence index {} is out of range for {} records",
                params.excluded_index, total_count
            ),
        )
        .with_code("tools.notseq.index.out_of_range"));
    }

    records.remove(params.excluded_index - 1);

    Ok(NotseqOutcome {
        input: params.input,
        excluded_index: params.excluded_index,
        total_count,
        records,
    })
}
