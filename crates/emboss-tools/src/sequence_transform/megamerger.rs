//! `megamerger` implementation.

use emboss_core::{MoleculeKind, SequenceRecord};

use crate::sequence_stream::{SequenceInput, ToolExecutionError};
use crate::sequence_transform::shared::{load_exactly_one_record, merge_records_by_exact_overlap};

/// Typed parameters for `megamerger`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MegamergerParams {
    /// Left DNA input sequence.
    pub left: SequenceInput,
    /// Right DNA input sequence.
    pub right: SequenceInput,
}

/// Structured `megamerger` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MegamergerOutcome {
    /// Left input path.
    pub left: SequenceInput,
    /// Right input path.
    pub right: SequenceInput,
    /// Longest exact overlap length used for the merge.
    pub overlap_length: usize,
    /// Merged DNA sequence record.
    pub record: SequenceRecord,
}

/// Returns `megamerger` help text.
#[must_use]
pub fn megamerger_help() -> &'static str {
    "Usage: emboss-rs megamerger <left-input> <right-input>\n\nMerge exactly one DNA sequence record from each input by the longest positive exact overlap between the left suffix and right prefix. v1 is DNA-only and emits one merged sequence record."
}

/// Executes `megamerger`.
pub fn run_megamerger(params: MegamergerParams) -> Result<MegamergerOutcome, ToolExecutionError> {
    let left = load_exactly_one_record(&params.left, "megamerger", "left")?;
    let right = load_exactly_one_record(&params.right, "megamerger", "right")?;
    let (record, overlap_length) =
        merge_records_by_exact_overlap("megamerger", left, right, Some(MoleculeKind::Dna))?;

    Ok(MegamergerOutcome {
        left: params.left,
        right: params.right,
        overlap_length,
        record,
    })
}

#[cfg(test)]
mod tests {
    use super::{MegamergerParams, run_megamerger};
    use crate::sequence_stream::SequenceInput;
    use emboss_core::MoleculeKind;
    use std::path::PathBuf;

    fn fixture(name: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join(format!("../../crates/emboss-tools/tests/fixtures/{name}"))
    }

    #[test]
    fn merges_two_dna_sequences_by_longest_overlap() {
        let outcome = run_megamerger(MegamergerParams {
            left: SequenceInput::new(fixture("merger_left.fasta")),
            right: SequenceInput::new(fixture("merger_right.fasta")),
        })
        .expect("megamerger should succeed");

        assert_eq!(outcome.overlap_length, 3);
        assert_eq!(outcome.record.residues(), "ACGTAAGGG");
        assert_eq!(outcome.record.molecule(), MoleculeKind::Dna);
    }

    #[test]
    fn rejects_non_dna_inputs() {
        let error = run_megamerger(MegamergerParams {
            left: SequenceInput::new(fixture("merger_left.fasta")),
            right: SequenceInput::new(fixture("merger_protein.fasta")),
        })
        .expect_err("megamerger should reject protein");

        assert_eq!(error.code(), Some("tools.megamerger.molecule.invalid"));
    }
}
