//! `revseq` implementation.

use emboss_core::SequenceRecord;
use emboss_diagnostics::{ErrorCategory, PlatformError};

use crate::sequence_stream::{SequenceInput, ToolExecutionError, load_sequence_records};

/// Typed parameters for `revseq`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RevseqParams {
    /// Local sequence input path.
    pub input: SequenceInput,
}

/// Structured `revseq` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RevseqOutcome {
    /// Source input.
    pub input: SequenceInput,
    /// Reversed records.
    pub records: Vec<SequenceRecord>,
}

/// Returns `revseq` help text.
#[must_use]
pub fn revseq_help() -> &'static str {
    "Usage: emboss-rs revseq <input>\n\nReverse sequence content for each input record. This v1 implementation performs plain reversal only, not reverse-complement."
}

/// Executes `revseq`.
pub fn run_revseq(params: RevseqParams) -> Result<RevseqOutcome, ToolExecutionError> {
    let records = load_sequence_records(&params.input)?
        .into_iter()
        .map(reverse_record)
        .collect::<Result<Vec<_>, _>>()?;

    Ok(RevseqOutcome {
        input: params.input,
        records,
    })
}

fn reverse_record(record: SequenceRecord) -> Result<SequenceRecord, ToolExecutionError> {
    let reversed: String = record.residues().chars().rev().collect();
    let mut updated = SequenceRecord::new(record.identifier().clone(), record.molecule(), reversed)
        .map_err(|error| {
            PlatformError::new(ErrorCategory::Validation, error.to_string())
                .with_code("tools.revseq.sequence.invalid")
        })?;
    updated = updated.with_metadata(record.metadata().clone());
    Ok(updated)
}
