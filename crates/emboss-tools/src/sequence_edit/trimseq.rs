//! `trimseq` implementation.

use emboss_core::SequenceRecord;
use emboss_diagnostics::{ErrorCategory, PlatformError};

use crate::sequence_stream::{SequenceInput, ToolExecutionError, load_sequence_records};

/// Typed parameters for `trimseq`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TrimseqParams {
    /// Local sequence input path.
    pub input: SequenceInput,
    /// Residues to trim from the left.
    pub left_trim: usize,
    /// Residues to trim from the right.
    pub right_trim: usize,
}

/// Structured `trimseq` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TrimseqOutcome {
    /// Source input.
    pub input: SequenceInput,
    /// Left trim count.
    pub left_trim: usize,
    /// Right trim count.
    pub right_trim: usize,
    /// Trimmed records in input order.
    pub records: Vec<SequenceRecord>,
}

/// Returns `trimseq` help text.
#[must_use]
pub fn trimseq_help() -> &'static str {
    "Usage: emboss-rs trimseq <input> [--left <count>] [--right <count>]\n\nTrim explicit residue counts from the left and right ends of each input record. The total trim must leave at least one residue."
}

/// Executes `trimseq`.
pub fn run_trimseq(params: TrimseqParams) -> Result<TrimseqOutcome, ToolExecutionError> {
    let records = load_sequence_records(&params.input)?
        .into_iter()
        .map(|record| trim_record(record, params.left_trim, params.right_trim))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(TrimseqOutcome {
        input: params.input,
        left_trim: params.left_trim,
        right_trim: params.right_trim,
        records,
    })
}

fn trim_record(
    record: SequenceRecord,
    left_trim: usize,
    right_trim: usize,
) -> Result<SequenceRecord, ToolExecutionError> {
    let total_trim = left_trim.saturating_add(right_trim);
    if total_trim >= record.len() {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!(
                "trimming {} residues would exhaust sequence '{}' of length {}",
                total_trim,
                record.identifier().accession(),
                record.len()
            ),
        )
        .with_code("tools.trimseq.trim.exhausted"));
    }

    let end = record.len() - right_trim;
    let trimmed = &record.residues()[left_trim..end];
    let mut updated = SequenceRecord::new(record.identifier().clone(), record.molecule(), trimmed)
        .map_err(|error| {
            PlatformError::new(ErrorCategory::Validation, error.to_string())
                .with_code("tools.trimseq.sequence.invalid")
        })?;
    updated = updated.with_metadata(record.metadata().clone());
    Ok(updated)
}
