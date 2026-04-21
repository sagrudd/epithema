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
    "Usage: emboss-rs notseq <input> <index>\n\nExclude the 1-based indexed sequence record from one local FASTA, FASTQ, EMBL, or GenBank input and return the remaining records in their original order."
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

#[cfg(test)]
mod tests {
    use super::{NotseqParams, run_notseq};
    use crate::sequence_stream::SequenceInput;
    use std::fs;

    fn write_temp_sequence_file(name: &str, contents: &str) -> std::path::PathBuf {
        let path = std::env::temp_dir().join(format!(
            "emboss-rs-notseq-{name}-{}-{}.fasta",
            std::process::id(),
            std::thread::current().name().unwrap_or("main")
        ));
        fs::write(&path, contents).expect("temporary sequence fixture should be written");
        path
    }

    #[test]
    fn excludes_requested_record_and_preserves_order() {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../crates/emboss-tools/tests/fixtures/three_records.fasta");
        let outcome = run_notseq(NotseqParams {
            input: SequenceInput::new(path),
            excluded_index: 2,
        })
        .expect("notseq should succeed");

        assert_eq!(outcome.total_count, 3);
        assert_eq!(outcome.records.len(), 2);
        assert_eq!(outcome.records[0].identifier().accession(), "alpha");
        assert_eq!(outcome.records[1].identifier().accession(), "gamma");
    }

    #[test]
    fn can_exclude_first_record_and_leave_tail() {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../crates/emboss-tools/tests/fixtures/three_records.fasta");
        let outcome = run_notseq(NotseqParams {
            input: SequenceInput::new(path),
            excluded_index: 1,
        })
        .expect("notseq should succeed");

        assert_eq!(outcome.records.len(), 2);
        assert_eq!(outcome.records[0].identifier().accession(), "beta");
        assert_eq!(outcome.records[1].identifier().accession(), "gamma");
    }

    #[test]
    fn can_exclude_only_record_and_return_empty_stream() {
        let path = write_temp_sequence_file("single", ">single\nACGT\n");
        let outcome = run_notseq(NotseqParams {
            input: SequenceInput::new(path.clone()),
            excluded_index: 1,
        })
        .expect("notseq should succeed");
        fs::remove_file(path).ok();

        assert_eq!(outcome.total_count, 1);
        assert!(outcome.records.is_empty());
    }

    #[test]
    fn rejects_zero_index() {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../crates/emboss-tools/tests/fixtures/three_records.fasta");
        let error = run_notseq(NotseqParams {
            input: SequenceInput::new(path),
            excluded_index: 0,
        })
        .expect_err("zero index should fail");

        assert!(error.to_string().contains("must be 1 or greater"));
    }

    #[test]
    fn rejects_out_of_range_index() {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../crates/emboss-tools/tests/fixtures/three_records.fasta");
        let error = run_notseq(NotseqParams {
            input: SequenceInput::new(path),
            excluded_index: 4,
        })
        .expect_err("out of range index should fail");

        assert!(error.to_string().contains("out of range"));
    }

    #[test]
    fn rejects_malformed_input() {
        let path = write_temp_sequence_file("malformed", "ACGT\n");
        let error = run_notseq(NotseqParams {
            input: SequenceInput::new(path.clone()),
            excluded_index: 1,
        })
        .expect_err("malformed input should fail");
        fs::remove_file(path).ok();

        assert!(error.to_string().contains("invalid fasta content"));
    }
}
