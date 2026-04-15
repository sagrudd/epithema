//! `nthseq` implementation.

use emboss_diagnostics::{ErrorCategory, PlatformError};

use super::{SequenceInput, ToolExecutionError, load_sequence_records};
use emboss_core::SequenceRecord;

/// Typed parameters for `nthseq`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NthseqParams {
    /// Local sequence input path.
    pub input: SequenceInput,
    /// 1-based record index to select.
    pub index: usize,
}

/// Structured `nthseq` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NthseqOutcome {
    /// Input path used for selection.
    pub input: SequenceInput,
    /// 1-based record index selected.
    pub index: usize,
    /// Total record count in the input.
    pub total_count: usize,
    /// Selected sequence record.
    pub record: SequenceRecord,
}

/// Returns the `nthseq` help text.
#[must_use]
pub fn nthseq_help() -> &'static str {
    "Usage: emboss-rs nthseq <input> <index>\n\nSelect the 1-based Nth sequence record from a local FASTA, FASTQ, EMBL, or GenBank file."
}

/// Executes `nthseq`.
pub fn run_nthseq(params: NthseqParams) -> Result<NthseqOutcome, ToolExecutionError> {
    if params.index == 0 {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            "sequence index must be 1 or greater",
        )
        .with_code("tools.nthseq.index.invalid"));
    }

    let mut records = load_sequence_records(&params.input)?;
    let total_count = records.len();
    if params.index > total_count {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!(
                "requested sequence index {} is out of range for {} records",
                params.index, total_count
            ),
        )
        .with_code("tools.nthseq.index.out_of_range"));
    }

    Ok(NthseqOutcome {
        input: params.input,
        index: params.index,
        total_count,
        record: records.remove(params.index - 1),
    })
}
