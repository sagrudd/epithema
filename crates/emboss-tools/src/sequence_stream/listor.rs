//! `listor` implementation.

use emboss_core::SequenceRecord;

use super::set_ops::{SequenceSetOperator, apply_sequence_set_operator};
use super::{SequenceInput, ToolExecutionError, load_sequence_records};

/// Typed parameters for `listor`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ListorParams {
    /// First input sequence set.
    pub first: SequenceInput,
    /// Second input sequence set.
    pub second: SequenceInput,
    /// Logical operator to apply.
    pub operator: SequenceSetOperator,
}

/// Structured `listor` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ListorOutcome {
    /// First input sequence set.
    pub first: SequenceInput,
    /// Second input sequence set.
    pub second: SequenceInput,
    /// Logical operator applied.
    pub operator: SequenceSetOperator,
    /// Stable result records.
    pub records: Vec<SequenceRecord>,
    /// Duplicate count removed from the first input before set comparison.
    pub first_duplicate_count: usize,
    /// Duplicate count removed from the second input before set comparison.
    pub second_duplicate_count: usize,
}

/// Returns `listor` help text.
#[must_use]
pub fn listor_help() -> &'static str {
    "Usage: emboss-rs listor <first-input> <second-input> [--operator <OR|AND|XOR|NOT>]\n\nCombine two local sequence sets with a deterministic logical set operator. EMBOSS-RS v1 compares normalized sequence content plus molecule kind, removes exact duplicates within each input before applying the operator, preserves first-seen representatives, and emits sequence records rather than a historical USA list file."
}

/// Executes `listor`.
pub fn run_listor(params: ListorParams) -> Result<ListorOutcome, ToolExecutionError> {
    let (records, first_duplicate_count, second_duplicate_count) = apply_sequence_set_operator(
        load_sequence_records(&params.first)?,
        load_sequence_records(&params.second)?,
        params.operator,
    );

    Ok(ListorOutcome {
        first: params.first,
        second: params.second,
        operator: params.operator,
        records,
        first_duplicate_count,
        second_duplicate_count,
    })
}

#[cfg(test)]
mod tests {
    use super::{ListorParams, run_listor};
    use crate::sequence_stream::{SequenceInput, set_ops::SequenceSetOperator};
    use std::fs;

    fn write_temp_sequence_file(name: &str, contents: &str) -> std::path::PathBuf {
        let path = std::env::temp_dir().join(format!(
            "emboss-rs-listor-{name}-{}-{}.fasta",
            std::process::id(),
            std::thread::current().name().unwrap_or("main")
        ));
        fs::write(&path, contents).expect("temporary sequence fixture should be written");
        path
    }

    #[test]
    fn computes_or_in_stable_order() {
        let outcome = run_listor(ListorParams {
            first: SequenceInput::new(
                std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join("../../crates/emboss-tools/tests/fixtures/listor_first.fasta"),
            ),
            second: SequenceInput::new(
                std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join("../../crates/emboss-tools/tests/fixtures/listor_second.fasta"),
            ),
            operator: SequenceSetOperator::Or,
        })
        .expect("listor should succeed");

        let ids: Vec<_> = outcome
            .records
            .iter()
            .map(|record| record.identifier().accession().to_owned())
            .collect();
        assert_eq!(ids, vec!["alpha", "beta", "delta"]);
    }

    #[test]
    fn computes_not_against_second_input() {
        let outcome = run_listor(ListorParams {
            first: SequenceInput::new(
                std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join("../../crates/emboss-tools/tests/fixtures/listor_first.fasta"),
            ),
            second: SequenceInput::new(
                std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join("../../crates/emboss-tools/tests/fixtures/listor_second.fasta"),
            ),
            operator: SequenceSetOperator::Not,
        })
        .expect("listor should succeed");

        assert_eq!(outcome.records.len(), 1);
        assert_eq!(outcome.records[0].identifier().accession(), "beta");
    }

    #[test]
    fn removes_duplicates_within_each_input_before_set_logic() {
        let first = write_temp_sequence_file("first", ">a\nACGT\n>b\nACGT\n");
        let second = write_temp_sequence_file("second", ">x\nTTTT\n>y\nTTTT\n");
        let outcome = run_listor(ListorParams {
            first: SequenceInput::new(&first),
            second: SequenceInput::new(&second),
            operator: SequenceSetOperator::Or,
        })
        .expect("listor should succeed");
        fs::remove_file(first).ok();
        fs::remove_file(second).ok();

        assert_eq!(outcome.first_duplicate_count, 1);
        assert_eq!(outcome.second_duplicate_count, 1);
        assert_eq!(outcome.records.len(), 2);
    }
}
