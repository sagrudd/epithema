//! `union` implementation.

use epithema_core::SequenceRecord;
use epithema_diagnostics::{ErrorCategory, PlatformError};

use crate::sequence_stream::{SequenceInput, ToolExecutionError, load_sequence_records};

/// Typed parameters for `union`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnionParams {
    /// Ordered local sequence inputs.
    pub inputs: Vec<SequenceInput>,
}

/// Structured `union` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnionOutcome {
    /// Ordered source inputs.
    pub inputs: Vec<SequenceInput>,
    /// Combined records in deterministic input order.
    pub records: Vec<SequenceRecord>,
}

/// Returns `union` help text.
#[must_use]
pub fn union_help() -> &'static str {
    "Usage: epithema union <input-a> <input-b> [input-c ...]\n\nConcatenate two or more local sequence inputs into one output record stream, preserving input order, per-input record order, and duplicates."
}

/// Executes `union`.
pub fn run_union(params: UnionParams) -> Result<UnionOutcome, ToolExecutionError> {
    if params.inputs.len() < 2 {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            "union requires at least two sequence inputs",
        )
        .with_code("tools.union.inputs.too_few"));
    }

    let mut records = Vec::new();
    for input in &params.inputs {
        records.extend(load_sequence_records(input)?);
    }

    Ok(UnionOutcome {
        inputs: params.inputs,
        records,
    })
}

#[cfg(test)]
mod tests {
    use super::{UnionParams, run_union};
    use crate::sequence_stream::SequenceInput;
    use std::fs;

    fn write_temp_sequence_file(name: &str, contents: &str) -> std::path::PathBuf {
        let path = std::env::temp_dir().join(format!(
            "epithema-union-{name}-{}-{}.fasta",
            std::process::id(),
            std::thread::current().name().unwrap_or("main")
        ));
        fs::write(&path, contents).expect("temporary sequence fixture should be written");
        path
    }

    fn fixture(name: &str) -> std::path::PathBuf {
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join(format!("../../crates/epithema-tools/tests/fixtures/{name}"))
    }

    #[test]
    fn concatenates_two_inputs_in_stable_order() {
        let outcome = run_union(UnionParams {
            inputs: vec![
                SequenceInput::new(fixture("three_records.fasta")),
                SequenceInput::new(fixture("two_records.fasta")),
            ],
        })
        .expect("union should succeed");

        let ids: Vec<_> = outcome
            .records
            .iter()
            .map(|record| record.identifier().accession().to_owned())
            .collect();
        assert_eq!(ids, vec!["alpha", "beta", "gamma", "delta", "epsilon"]);
    }

    #[test]
    fn preserves_duplicate_identifiers_and_records() {
        let duplicate_path = write_temp_sequence_file(
            "duplicates",
            ">alpha duplicate one\nAAAA\n>alpha duplicate two\nCCCC\n",
        );
        let outcome = run_union(UnionParams {
            inputs: vec![
                SequenceInput::new(fixture("three_records.fasta")),
                SequenceInput::new(duplicate_path.clone()),
            ],
        })
        .expect("union should succeed");
        fs::remove_file(duplicate_path).ok();

        let ids: Vec<_> = outcome
            .records
            .iter()
            .map(|record| record.identifier().accession().to_owned())
            .collect();
        assert_eq!(ids, vec!["alpha", "beta", "gamma", "alpha", "alpha"]);
        assert_eq!(outcome.records[3].residues(), "AAAA");
        assert_eq!(outcome.records[4].residues(), "CCCC");
    }

    #[test]
    fn supports_empty_plus_non_empty_only_when_non_empty_is_valid_and_empty_is_not() {
        let empty_path = write_temp_sequence_file("empty", "");
        let error = run_union(UnionParams {
            inputs: vec![
                SequenceInput::new(empty_path.clone()),
                SequenceInput::new(fixture("two_records.fasta")),
            ],
        })
        .expect_err("empty input should fail");
        fs::remove_file(empty_path).ok();

        assert!(error.to_string().contains("no FASTA records were found"));
    }

    #[test]
    fn rejects_too_few_inputs() {
        let error = run_union(UnionParams {
            inputs: vec![SequenceInput::new(fixture("three_records.fasta"))],
        })
        .expect_err("single input should fail");

        assert!(error.to_string().contains("at least two sequence inputs"));
    }

    #[test]
    fn rejects_malformed_input() {
        let malformed_path = write_temp_sequence_file("malformed", "ACGT\n");
        let error = run_union(UnionParams {
            inputs: vec![
                SequenceInput::new(fixture("three_records.fasta")),
                SequenceInput::new(malformed_path.clone()),
            ],
        })
        .expect_err("malformed input should fail");
        fs::remove_file(malformed_path).ok();

        assert!(error.to_string().contains("invalid fasta content"));
    }
}
