//! `seqretsetall` implementation.

use epithema_core::SequenceRecord;
use epithema_diagnostics::{ErrorCategory, PlatformError};

use super::SeqretSource;

/// Shared execution error for retrieval tools.
pub type ToolExecutionError = PlatformError;

/// One ordered resolved input set for `seqretsetall`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SeqretsetallInputSet {
    /// Resolved input source.
    pub source: SeqretSource,
    /// Normalized records produced from that source.
    pub records: Vec<SequenceRecord>,
}

/// Typed parameters for `seqretsetall`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SeqretsetallParams {
    /// Ordered resolved input sets.
    pub inputs: Vec<SeqretsetallInputSet>,
}

/// Structured `seqretsetall` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SeqretsetallOutcome {
    /// Ordered resolved sources.
    pub inputs: Vec<SeqretSource>,
    /// Ordered normalized record sets, preserving per-input grouping.
    pub record_sets: Vec<Vec<SequenceRecord>>,
    /// Total record count across every input set.
    pub total_records: usize,
}

/// Returns the `seqretsetall` help text.
#[must_use]
pub fn seqretsetall_help() -> &'static str {
    "Usage: epithema seqretsetall <input-a> <input-b> [input-c ...]\n\nNormalize two or more local or provider-backed sequence inputs and preserve them as ordered output record sets."
}

/// Executes `seqretsetall`.
pub fn run_seqretsetall(
    params: SeqretsetallParams,
) -> Result<SeqretsetallOutcome, ToolExecutionError> {
    if params.inputs.len() < 2 {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            "seqretsetall requires at least two sequence inputs",
        )
        .with_code("tools.seqretsetall.inputs.too_few"));
    }

    let mut inputs = Vec::with_capacity(params.inputs.len());
    let mut record_sets = Vec::with_capacity(params.inputs.len());
    let mut total_records = 0usize;

    for input in params.inputs {
        if input.records.is_empty() {
            return Err(PlatformError::new(
                ErrorCategory::Validation,
                "seqretsetall requires each input set to produce at least one sequence record",
            )
            .with_code("tools.seqretsetall.input.empty"));
        }

        total_records += input.records.len();
        inputs.push(input.source);
        record_sets.push(input.records);
    }

    Ok(SeqretsetallOutcome {
        inputs,
        record_sets,
        total_records,
    })
}

#[cfg(test)]
mod tests {
    use epithema_core::{MoleculeKind, SequenceIdentifier, SequenceRecord};

    use super::{SeqretsetallInputSet, SeqretsetallParams, run_seqretsetall};
    use crate::retrieval_tools::SeqretSource;
    use crate::sequence_stream::{SequenceInput, load_sequence_records};

    fn fixture(name: &str) -> std::path::PathBuf {
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join(format!("../../crates/epithema-tools/tests/fixtures/{name}"))
    }

    #[test]
    fn preserves_input_partitioning_and_order() {
        let local_records =
            load_sequence_records(&SequenceInput::new(fixture("three_records.fasta")))
                .expect("fixture should load");
        let retrieved_record = SequenceRecord::new(
            SequenceIdentifier::new("AB000263").expect("identifier should build"),
            MoleculeKind::Dna,
            "ACGT",
        )
        .expect("record should build");
        let outcome = run_seqretsetall(SeqretsetallParams {
            inputs: vec![
                SeqretsetallInputSet {
                    source: SeqretSource::LocalPath(fixture("three_records.fasta")),
                    records: local_records,
                },
                SeqretsetallInputSet {
                    source: SeqretSource::Retrieved {
                        provider: "ena".to_owned(),
                        accession: "AB000263".to_owned(),
                    },
                    records: vec![retrieved_record],
                },
            ],
        })
        .expect("seqretsetall should succeed");

        assert_eq!(outcome.inputs.len(), 2);
        assert_eq!(outcome.record_sets.len(), 2);
        assert_eq!(outcome.record_sets[0].len(), 3);
        assert_eq!(outcome.record_sets[1].len(), 1);
        assert_eq!(outcome.total_records, 4);
        assert_eq!(outcome.record_sets[0][0].identifier().accession(), "alpha");
        assert_eq!(
            outcome.record_sets[1][0].identifier().accession(),
            "AB000263"
        );
    }

    #[test]
    fn rejects_too_few_inputs() {
        let records = load_sequence_records(&SequenceInput::new(fixture("three_records.fasta")))
            .expect("fixture should load");
        let error = run_seqretsetall(SeqretsetallParams {
            inputs: vec![SeqretsetallInputSet {
                source: SeqretSource::LocalPath(fixture("three_records.fasta")),
                records,
            }],
        })
        .expect_err("single input should fail");

        assert!(error.to_string().contains("at least two sequence inputs"));
    }

    #[test]
    fn rejects_empty_input_sets() {
        let error = run_seqretsetall(SeqretsetallParams {
            inputs: vec![
                SeqretsetallInputSet {
                    source: SeqretSource::LocalPath(fixture("three_records.fasta")),
                    records: load_sequence_records(&SequenceInput::new(fixture(
                        "three_records.fasta",
                    )))
                    .expect("fixture should load"),
                },
                SeqretsetallInputSet {
                    source: SeqretSource::Retrieved {
                        provider: "ena".to_owned(),
                        accession: "EMPTY".to_owned(),
                    },
                    records: Vec::new(),
                },
            ],
        })
        .expect_err("empty input sets should fail");

        assert!(error.to_string().contains("at least one sequence record"));
    }
}
