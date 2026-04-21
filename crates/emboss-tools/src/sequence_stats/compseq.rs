//! `compseq` implementation.

use emboss_core::{MoleculeKind, ResidueComposition};

use crate::sequence_stream::{SequenceInput, ToolExecutionError, load_sequence_records};

/// Typed parameters for `compseq`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompseqParams {
    /// Local sequence input path.
    pub input: SequenceInput,
}

/// One per-record composition entry.
#[derive(Clone, Debug, PartialEq)]
pub struct CompseqRecord {
    /// Record identifier.
    pub record_id: String,
    /// Molecule kind.
    pub molecule: MoleculeKind,
    /// Raw sequence length.
    pub sequence_length: usize,
    /// Composition summary excluding gaps.
    pub composition: ResidueComposition,
}

/// Structured `compseq` outcome.
#[derive(Clone, Debug, PartialEq)]
pub struct CompseqOutcome {
    /// Source input.
    pub input: SequenceInput,
    /// Per-record composition summaries.
    pub records: Vec<CompseqRecord>,
    /// Aggregate composition across all records.
    pub aggregate: ResidueComposition,
}

/// Returns `compseq` help text.
#[must_use]
pub fn compseq_help() -> &'static str {
    "Usage: emboss-rs compseq <input>\n\nReport deterministic residue composition counts and frequencies for nucleotide or protein sequence records. The v1 implementation reports both per-record rows and an aggregate summary across all records. Gap symbols '-' are ignored; all other normalized residue symbols, including ambiguity and stop symbols, are counted."
}

/// Executes `compseq`.
pub fn run_compseq(params: CompseqParams) -> Result<CompseqOutcome, ToolExecutionError> {
    let mut aggregate = ResidueComposition::default();
    let records = load_sequence_records(&params.input)?
        .into_iter()
        .map(|record| {
            let composition = ResidueComposition::from_sequence(record.residues());
            aggregate.merge(&composition);
            CompseqRecord {
                record_id: record.identifier().accession().to_owned(),
                molecule: record.molecule(),
                sequence_length: record.len(),
                composition,
            }
        })
        .collect();

    Ok(CompseqOutcome {
        input: params.input,
        records,
        aggregate,
    })
}

#[cfg(test)]
mod tests {
    use super::{CompseqParams, run_compseq};
    use crate::sequence_stream::SequenceInput;
    use std::fs;

    fn write_temp_sequence_file(name: &str, contents: &str) -> std::path::PathBuf {
        let path = std::env::temp_dir().join(format!(
            "emboss-rs-compseq-{name}-{}-{}.fasta",
            std::process::id(),
            std::thread::current().name().unwrap_or("main")
        ));
        fs::write(&path, contents).expect("temporary sequence fixture should be written");
        path
    }

    fn fixture(name: &str) -> std::path::PathBuf {
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join(format!("../../crates/emboss-tools/tests/fixtures/{name}"))
    }

    #[test]
    fn reports_nucleotide_counts_and_aggregate_frequencies() {
        let outcome = run_compseq(CompseqParams {
            input: SequenceInput::new(fixture("three_records.fasta")),
        })
        .expect("compseq should succeed");

        assert_eq!(outcome.records.len(), 3);
        assert_eq!(outcome.records[0].record_id, "alpha");
        assert_eq!(outcome.records[0].sequence_length, 4);
        assert_eq!(outcome.records[0].composition.count_for('A'), 1);
        assert!((outcome.records[0].composition.frequency_for('A') - 0.25).abs() < 1e-9);
        assert_eq!(outcome.aggregate.count_for('T'), 5);
        assert_eq!(outcome.aggregate.counted_symbols(), 12);
    }

    #[test]
    fn reports_protein_and_stop_symbol_counts() {
        let outcome = run_compseq(CompseqParams {
            input: SequenceInput::new(fixture("protein_records.fasta")),
        })
        .expect("compseq should succeed");

        assert_eq!(outcome.records.len(), 2);
        assert_eq!(outcome.records[0].record_id, "protA");
        assert_eq!(outcome.records[0].composition.count_for('M'), 1);
        assert_eq!(outcome.records[0].composition.count_for('*'), 1);
        assert_eq!(outcome.aggregate.count_for('L'), 1);
        assert_eq!(outcome.aggregate.count_for('S'), 1);
    }

    #[test]
    fn retains_ambiguity_symbols_and_ignores_gaps() {
        let path = write_temp_sequence_file("ambiguous", ">amb\nACGN-*N\n");
        let outcome = run_compseq(CompseqParams {
            input: SequenceInput::new(path.clone()),
        })
        .expect("compseq should succeed");
        fs::remove_file(path).ok();

        assert_eq!(outcome.records.len(), 1);
        assert_eq!(outcome.records[0].composition.count_for('N'), 2);
        assert_eq!(outcome.records[0].composition.count_for('*'), 1);
        assert_eq!(outcome.records[0].composition.ignored_gap_symbols(), 1);
        assert_eq!(outcome.aggregate.counted_symbols(), 6);
    }

    #[test]
    fn rejects_empty_input() {
        let path = write_temp_sequence_file("empty", "");
        let error = run_compseq(CompseqParams {
            input: SequenceInput::new(path.clone()),
        })
        .expect_err("empty input should fail");
        fs::remove_file(path).ok();

        assert!(error.to_string().contains("no FASTA records were found"));
    }
}
