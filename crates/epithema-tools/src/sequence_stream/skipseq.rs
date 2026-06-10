//! `skipseq` implementation.

use super::{SequenceInput, ToolExecutionError, load_sequence_records};
use epithema_core::SequenceRecord;

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
    "Usage: epithema skipseq <input> <count>\n\nSkip a non-negative number of leading sequence records from one local FASTA, FASTQ, EMBL, or GenBank input and return the remaining records in their original order."
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

#[cfg(test)]
mod tests {
    use super::{SkipseqParams, run_skipseq};
    use crate::sequence_stream::SequenceInput;
    use std::fs;

    fn write_temp_sequence_file(name: &str, contents: &str) -> std::path::PathBuf {
        let path = std::env::temp_dir().join(format!(
            "epithema-skipseq-{name}-{}-{}.fasta",
            std::process::id(),
            std::thread::current().name().unwrap_or("main")
        ));
        fs::write(&path, contents).expect("temporary sequence fixture should be written");
        path
    }

    fn three_record_fixture() -> std::path::PathBuf {
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../crates/epithema-tools/tests/fixtures/three_records.fasta")
    }

    #[test]
    fn skip_zero_returns_all_records() {
        let outcome = run_skipseq(SkipseqParams {
            input: SequenceInput::new(three_record_fixture()),
            count: 0,
        })
        .expect("skipseq should succeed");

        assert_eq!(outcome.total_count, 3);
        assert_eq!(outcome.skipped_count, 0);
        assert_eq!(outcome.records.len(), 3);
        assert_eq!(outcome.records[0].identifier().accession(), "alpha");
    }

    #[test]
    fn skip_one_returns_tail() {
        let outcome = run_skipseq(SkipseqParams {
            input: SequenceInput::new(three_record_fixture()),
            count: 1,
        })
        .expect("skipseq should succeed");

        assert_eq!(outcome.skipped_count, 1);
        assert_eq!(outcome.records.len(), 2);
        assert_eq!(outcome.records[0].identifier().accession(), "beta");
        assert_eq!(outcome.records[1].identifier().accession(), "gamma");
    }

    #[test]
    fn skip_interior_count_returns_remaining_suffix() {
        let outcome = run_skipseq(SkipseqParams {
            input: SequenceInput::new(three_record_fixture()),
            count: 2,
        })
        .expect("skipseq should succeed");

        assert_eq!(outcome.skipped_count, 2);
        assert_eq!(outcome.records.len(), 1);
        assert_eq!(outcome.records[0].identifier().accession(), "gamma");
    }

    #[test]
    fn skip_all_returns_empty_stream() {
        let outcome = run_skipseq(SkipseqParams {
            input: SequenceInput::new(three_record_fixture()),
            count: 3,
        })
        .expect("skipseq should succeed");

        assert_eq!(outcome.skipped_count, 3);
        assert!(outcome.records.is_empty());
    }

    #[test]
    fn skip_beyond_end_returns_empty_stream() {
        let outcome = run_skipseq(SkipseqParams {
            input: SequenceInput::new(three_record_fixture()),
            count: 99,
        })
        .expect("skipseq should succeed");

        assert_eq!(outcome.skipped_count, 3);
        assert!(outcome.records.is_empty());
    }

    #[test]
    fn rejects_empty_input() {
        let path = write_temp_sequence_file("empty", "");
        let error = run_skipseq(SkipseqParams {
            input: SequenceInput::new(path.clone()),
            count: 1,
        })
        .expect_err("empty input should fail");
        fs::remove_file(path).ok();

        assert!(error.to_string().contains("no FASTA records were found"));
    }

    #[test]
    fn rejects_malformed_input() {
        let path = write_temp_sequence_file("malformed", "ACGT\n");
        let error = run_skipseq(SkipseqParams {
            input: SequenceInput::new(path.clone()),
            count: 1,
        })
        .expect_err("malformed input should fail");
        fs::remove_file(path).ok();

        assert!(error.to_string().contains("invalid fasta content"));
    }
}
