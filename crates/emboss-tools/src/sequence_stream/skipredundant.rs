//! `skipredundant` implementation.

use emboss_core::SequenceRecord;

use super::set_ops::unique_records_in_order;
use super::{SequenceInput, ToolExecutionError, load_sequence_records};

/// Typed parameters for `skipredundant`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SkipredundantParams {
    /// Local sequence input path.
    pub input: SequenceInput,
}

/// Structured `skipredundant` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SkipredundantOutcome {
    /// Source input.
    pub input: SequenceInput,
    /// Stable non-redundant record set.
    pub records: Vec<SequenceRecord>,
    /// Input record count before redundancy removal.
    pub total_count: usize,
    /// Number of exact redundant records removed.
    pub redundant_count: usize,
}

/// Returns `skipredundant` help text.
#[must_use]
pub fn skipredundant_help() -> &'static str {
    "Usage: emboss-rs skipredundant <input>\n\nRemove exact redundant sequences from one local input set. EMBOSS-RS v1 defines redundancy conservatively as identical normalized sequence content plus molecule kind, keeps the first representative in source order, and does not implement historical alignment-threshold modes."
}

/// Executes `skipredundant`.
pub fn run_skipredundant(
    params: SkipredundantParams,
) -> Result<SkipredundantOutcome, ToolExecutionError> {
    let records = load_sequence_records(&params.input)?;
    let total_count = records.len();
    let (records, redundant_count) = unique_records_in_order(records);

    Ok(SkipredundantOutcome {
        input: params.input,
        records,
        total_count,
        redundant_count,
    })
}

#[cfg(test)]
mod tests {
    use super::{SkipredundantParams, run_skipredundant};
    use crate::sequence_stream::SequenceInput;

    #[test]
    fn removes_exact_duplicate_sequences_and_keeps_first_seen() {
        let outcome = run_skipredundant(SkipredundantParams {
            input: SequenceInput::new(
                std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join("../../crates/emboss-tools/tests/fixtures/skipredundant_records.fasta"),
            ),
        })
        .expect("skipredundant should succeed");

        assert_eq!(outcome.total_count, 4);
        assert_eq!(outcome.redundant_count, 2);
        assert_eq!(outcome.records.len(), 2);
        assert_eq!(outcome.records[0].identifier().accession(), "keep_alpha");
        assert_eq!(outcome.records[1].identifier().accession(), "keep_gamma");
    }
}
