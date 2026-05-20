//! `trimest` implementation.

use emboss_core::SequenceRecord;
use emboss_diagnostics::{ErrorCategory, PlatformError};

use crate::sequence_stream::{SequenceInput, ToolExecutionError, load_sequence_records};

use super::shared::{rebuild_unannotated_record, require_nucleotide_record};

/// Typed parameters for `trimest`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TrimestParams {
    /// Input sequence stream.
    pub input: SequenceInput,
    /// Minimum terminal poly-A run required for trimming.
    pub min_tail: usize,
}

/// Structured `trimest` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TrimestOutcome {
    /// Source input.
    pub input: SequenceInput,
    /// Minimum terminal poly-A run required for trimming.
    pub min_tail: usize,
    /// Trimmed records in input order.
    pub records: Vec<SequenceRecord>,
}

/// Returns `trimest` help text.
#[must_use]
pub fn trimest_help() -> &'static str {
    "Usage: emboss-rs trimest <input> [--min-tail <count>]\n\nRemove a terminal 3' poly-A run from each nucleotide input record when the run length meets or exceeds the configured minimum. v1 trims only trailing 'A' runs and drops feature annotations after editing."
}

/// Executes `trimest`.
pub fn run_trimest(params: TrimestParams) -> Result<TrimestOutcome, ToolExecutionError> {
    if params.min_tail == 0 {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            "trimest requires min-tail >= 1",
        )
        .with_code("tools.trimest.min_tail.invalid"));
    }

    let records = load_sequence_records(&params.input)?
        .into_iter()
        .map(|record| trim_record(&record, params.min_tail))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(TrimestOutcome {
        input: params.input,
        min_tail: params.min_tail,
        records,
    })
}

fn trim_record(record: &SequenceRecord, min_tail: usize) -> Result<SequenceRecord, ToolExecutionError> {
    require_nucleotide_record("trimest", record)?;
    let tail_len = record
        .residues()
        .chars()
        .rev()
        .take_while(|symbol| *symbol == 'A')
        .count();
    if tail_len < min_tail {
        return Ok(record.clone());
    }
    if tail_len == record.len() {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!(
                "trimest would remove all residues from sequence '{}'",
                record.identifier().accession()
            ),
        )
        .with_code("tools.trimest.sequence.empty"));
    }

    let residues = record.residues()[..record.len() - tail_len].to_owned();
    rebuild_unannotated_record("trimest", record, residues)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::sequence_stream::SequenceInput;

    use super::{TrimestParams, run_trimest};

    fn fixture(name: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join(format!("../../crates/emboss-tools/tests/fixtures/{name}"))
    }

    #[test]
    fn trims_terminal_poly_a_runs() {
        let outcome = run_trimest(TrimestParams {
            input: SequenceInput::new(fixture("trimest_records.fasta")),
            min_tail: 4,
        })
        .expect("trimest should succeed");

        assert_eq!(outcome.records[0].residues(), "ACGT");
        assert_eq!(outcome.records[1].residues(), "TTGCAAA");
    }
}
