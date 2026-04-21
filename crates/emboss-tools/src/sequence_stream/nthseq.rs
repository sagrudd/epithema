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
    "Usage: emboss-rs nthseq <input> <index>\n\nSelect exactly one 1-based indexed sequence record from one local FASTA, FASTQ, EMBL, or GenBank input."
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

#[cfg(test)]
mod tests {
    use super::{NthseqParams, run_nthseq};
    use crate::sequence_stream::SequenceInput;
    use std::fs;

    fn write_temp_sequence_file(name: &str, contents: &str) -> std::path::PathBuf {
        let path = std::env::temp_dir().join(format!(
            "emboss-rs-nthseq-{name}-{}-{}.fasta",
            std::process::id(),
            std::thread::current().name().unwrap_or("main")
        ));
        fs::write(&path, contents).expect("temporary sequence fixture should be written");
        path
    }

    fn three_record_fixture() -> std::path::PathBuf {
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../crates/emboss-tools/tests/fixtures/three_records.fasta")
    }

    #[test]
    fn selects_first_record() {
        let outcome = run_nthseq(NthseqParams {
            input: SequenceInput::new(three_record_fixture()),
            index: 1,
        })
        .expect("nthseq should succeed");

        assert_eq!(outcome.total_count, 3);
        assert_eq!(outcome.record.identifier().accession(), "alpha");
    }

    #[test]
    fn selects_interior_record() {
        let outcome = run_nthseq(NthseqParams {
            input: SequenceInput::new(three_record_fixture()),
            index: 2,
        })
        .expect("nthseq should succeed");

        assert_eq!(outcome.record.identifier().accession(), "beta");
    }

    #[test]
    fn selects_last_record() {
        let outcome = run_nthseq(NthseqParams {
            input: SequenceInput::new(three_record_fixture()),
            index: 3,
        })
        .expect("nthseq should succeed");

        assert_eq!(outcome.record.identifier().accession(), "gamma");
    }

    #[test]
    fn rejects_zero_index() {
        let error = run_nthseq(NthseqParams {
            input: SequenceInput::new(three_record_fixture()),
            index: 0,
        })
        .expect_err("zero index should fail");

        assert!(error.to_string().contains("must be 1 or greater"));
    }

    #[test]
    fn rejects_out_of_range_index() {
        let error = run_nthseq(NthseqParams {
            input: SequenceInput::new(three_record_fixture()),
            index: 4,
        })
        .expect_err("out of range index should fail");

        assert!(error.to_string().contains("out of range"));
    }

    #[test]
    fn rejects_empty_input() {
        let path = write_temp_sequence_file("empty", "");
        let error = run_nthseq(NthseqParams {
            input: SequenceInput::new(path.clone()),
            index: 1,
        })
        .expect_err("empty input should fail");
        fs::remove_file(path).ok();

        assert!(error.to_string().contains("no FASTA records were found"));
    }

    #[test]
    fn rejects_malformed_input() {
        let path = write_temp_sequence_file("malformed", "ACGT\n");
        let error = run_nthseq(NthseqParams {
            input: SequenceInput::new(path.clone()),
            index: 1,
        })
        .expect_err("malformed input should fail");
        fs::remove_file(path).ok();

        assert!(error.to_string().contains("invalid fasta content"));
    }
}
