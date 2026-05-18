//! `merger` implementation.

use emboss_core::SequenceRecord;

use crate::sequence_stream::{SequenceInput, ToolExecutionError};
use crate::sequence_transform::shared::{load_exactly_one_record, merge_records_by_exact_overlap};

/// Typed parameters for `merger`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MergerParams {
    /// Left input sequence.
    pub left: SequenceInput,
    /// Right input sequence.
    pub right: SequenceInput,
}

/// Structured `merger` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MergerOutcome {
    /// Left input path.
    pub left: SequenceInput,
    /// Right input path.
    pub right: SequenceInput,
    /// Longest exact overlap length used for the merge.
    pub overlap_length: usize,
    /// Merged sequence record.
    pub record: SequenceRecord,
}

/// Returns `merger` help text.
#[must_use]
pub fn merger_help() -> &'static str {
    "Usage: emboss-rs merger <left-input> <right-input>\n\nMerge exactly one sequence record from each input by the longest positive exact overlap between the left suffix and right prefix. v1 requires matching molecule kinds and emits one merged sequence record."
}

/// Executes `merger`.
pub fn run_merger(params: MergerParams) -> Result<MergerOutcome, ToolExecutionError> {
    let left = load_exactly_one_record(&params.left, "merger", "left")?;
    let right = load_exactly_one_record(&params.right, "merger", "right")?;
    let (record, overlap_length) = merge_records_by_exact_overlap("merger", left, right, None)?;

    Ok(MergerOutcome {
        left: params.left,
        right: params.right,
        overlap_length,
        record,
    })
}

#[cfg(test)]
mod tests {
    use super::{MergerParams, run_merger};
    use crate::sequence_stream::SequenceInput;
    use std::path::PathBuf;

    fn fixture(name: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("../../crates/emboss-tools/tests/fixtures/{name}"))
    }

    #[test]
    fn merges_two_sequences_by_longest_overlap() {
        let outcome = run_merger(MergerParams {
            left: SequenceInput::new(fixture("merger_left.fasta")),
            right: SequenceInput::new(fixture("merger_right.fasta")),
        })
        .expect("merger should succeed");

        assert_eq!(outcome.overlap_length, 3);
        assert_eq!(outcome.record.identifier().accession(), "left+right");
        assert_eq!(outcome.record.residues(), "ACGTAAGGG");
    }

    #[test]
    fn rejects_inputs_without_positive_overlap() {
        let error = run_merger(MergerParams {
            left: SequenceInput::new(fixture("merger_left.fasta")),
            right: SequenceInput::new(fixture("merger_no_overlap.fasta")),
        })
        .expect_err("merger should fail");

        assert_eq!(error.code(), Some("tools.merger.overlap.missing"));
    }
}
