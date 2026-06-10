//! `seqcount` implementation.

use super::{SequenceInput, ToolExecutionError, load_sequence_records};

/// Typed parameters for `seqcount`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SeqcountParams {
    /// Local sequence input path.
    pub input: SequenceInput,
}

/// Structured `seqcount` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SeqcountOutcome {
    /// Input path used for counting.
    pub input: SequenceInput,
    /// Deterministic record count.
    pub count: usize,
}

/// Returns the `seqcount` help text.
#[must_use]
pub fn seqcount_help() -> &'static str {
    "Usage: epithema seqcount <input>\n\nCount sequence records in one local sequence input using the shared FASTA, FASTQ, EMBL, or GenBank readers. v1 reports one deterministic count for one input path."
}

/// Executes `seqcount`.
pub fn run_seqcount(params: SeqcountParams) -> Result<SeqcountOutcome, ToolExecutionError> {
    let count = load_sequence_records(&params.input)?.len();
    Ok(SeqcountOutcome {
        input: params.input,
        count,
    })
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::{SeqcountParams, run_seqcount};
    use crate::sequence_stream::SequenceInput;

    fn fixture(path: &str) -> std::path::PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join(path)
    }

    fn temporary_fixture_path(suffix: &str) -> std::path::PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be valid")
            .as_nanos();
        std::env::temp_dir().join(format!(
            "epithema-seqcount-{}-{unique}.{suffix}",
            std::process::id()
        ))
    }

    #[test]
    fn counts_single_record_input() {
        let path = temporary_fixture_path("fasta");
        fs::write(&path, ">single example\nACGT\n").expect("fixture should write");

        let outcome = run_seqcount(SeqcountParams {
            input: SequenceInput::new(&path),
        })
        .expect("seqcount should succeed");

        assert_eq!(outcome.count, 1);

        let _ = fs::remove_file(path);
    }

    #[test]
    fn counts_multi_record_input() {
        let outcome = run_seqcount(SeqcountParams {
            input: SequenceInput::new(fixture(
                "../epithema-tools/tests/fixtures/three_records.fasta",
            )),
        })
        .expect("seqcount should succeed");

        assert_eq!(outcome.count, 3);
    }

    #[test]
    fn rejects_empty_input() {
        let path = temporary_fixture_path("fasta");
        fs::write(&path, "").expect("fixture should write");

        let error = run_seqcount(SeqcountParams {
            input: SequenceInput::new(&path),
        })
        .expect_err("empty input should fail");

        assert!(error.to_string().contains("no FASTA records were found"));

        let _ = fs::remove_file(path);
    }

    #[test]
    fn rejects_malformed_input() {
        let path = temporary_fixture_path("fasta");
        fs::write(&path, "ACGT\n").expect("fixture should write");

        let error = run_seqcount(SeqcountParams {
            input: SequenceInput::new(&path),
        })
        .expect_err("malformed fasta should fail");

        assert!(error.to_string().contains("invalid fasta content"));

        let _ = fs::remove_file(path);
    }
}
