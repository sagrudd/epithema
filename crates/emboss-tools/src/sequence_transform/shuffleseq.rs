//! `shuffleseq` implementation.

use emboss_core::SequenceRecord;
use emboss_diagnostics::{ErrorCategory, PlatformError};

use crate::sequence_stream::{SequenceInput, ToolExecutionError, load_sequence_records};
use crate::sequence_transform::shared::{derived_seed, shuffled_residues};

/// Typed parameters for `shuffleseq`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ShuffleseqParams {
    /// Input sequence stream.
    pub input: SequenceInput,
    /// Base deterministic shuffle seed.
    pub seed: u64,
}

/// Structured `shuffleseq` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ShuffleseqOutcome {
    /// Source input.
    pub input: SequenceInput,
    /// Base deterministic shuffle seed.
    pub seed: u64,
    /// Shuffled records in input order.
    pub records: Vec<SequenceRecord>,
}

/// Returns `shuffleseq` help text.
#[must_use]
pub fn shuffleseq_help() -> &'static str {
    "Usage: emboss-rs shuffleseq <input> [--seed <value>]\n\nShuffle residues within each input record deterministically while preserving composition. v1 uses a documented fixed-seed default when --seed is omitted."
}

/// Executes `shuffleseq`.
pub fn run_shuffleseq(params: ShuffleseqParams) -> Result<ShuffleseqOutcome, ToolExecutionError> {
    let records = load_sequence_records(&params.input)?
        .into_iter()
        .enumerate()
        .map(|(ordinal, record)| shuffle_record(record, derived_seed(params.seed, ordinal)))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(ShuffleseqOutcome {
        input: params.input,
        seed: params.seed,
        records,
    })
}

fn shuffle_record(record: SequenceRecord, seed: u64) -> Result<SequenceRecord, ToolExecutionError> {
    let metadata = record.metadata().clone();
    let shuffled = shuffled_residues(record.residues(), seed);
    SequenceRecord::new(record.identifier().clone(), record.molecule(), shuffled)
        .map(|record| record.with_metadata(metadata))
        .map_err(|error| {
            PlatformError::new(ErrorCategory::Validation, error.to_string())
                .with_code("tools.shuffleseq.sequence.invalid")
        })
}

#[cfg(test)]
mod tests {
    use super::{ShuffleseqParams, run_shuffleseq};
    use crate::sequence_stream::SequenceInput;
    use std::collections::BTreeMap;
    use std::path::PathBuf;

    fn fixture(name: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join(format!("../../crates/emboss-tools/tests/fixtures/{name}"))
    }

    fn composition(residues: &str) -> BTreeMap<char, usize> {
        let mut counts = BTreeMap::new();
        for residue in residues.chars() {
            *counts.entry(residue).or_insert(0) += 1;
        }
        counts
    }

    #[test]
    fn shuffles_residues_deterministically() {
        let first = run_shuffleseq(ShuffleseqParams {
            input: SequenceInput::new(fixture("three_records.fasta")),
            seed: 7,
        })
        .expect("shuffleseq should succeed");
        let second = run_shuffleseq(ShuffleseqParams {
            input: SequenceInput::new(fixture("three_records.fasta")),
            seed: 7,
        })
        .expect("shuffleseq should succeed");

        assert_eq!(first.records, second.records);
        assert_eq!(first.records[0].residues(), "CGTA");
    }

    #[test]
    fn preserves_residue_composition() {
        let outcome = run_shuffleseq(ShuffleseqParams {
            input: SequenceInput::new(fixture("three_records.fasta")),
            seed: 13,
        })
        .expect("shuffleseq should succeed");

        assert_eq!(
            composition(outcome.records[0].residues()),
            composition("ACGT")
        );
        assert_eq!(
            composition(outcome.records[1].residues()),
            composition("TTTT")
        );
    }
}
