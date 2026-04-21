//! `pepstats` implementation.

use emboss_core::{Alphabet, MoleculeKind, ResidueComposition, protein_molecular_weight};
use emboss_diagnostics::{ErrorCategory, PlatformError};

use crate::sequence_stream::{SequenceInput, ToolExecutionError, load_sequence_records};

/// Typed parameters for `pepstats`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PepstatsParams {
    /// Local protein input path.
    pub input: SequenceInput,
}

/// One per-record protein statistics summary.
#[derive(Clone, Debug, PartialEq)]
pub struct PepstatsRecord {
    /// Record identifier.
    pub record_id: String,
    /// Raw sequence length.
    pub sequence_length: usize,
    /// Number of non-gap, non-stop residues contributing to mass.
    pub residue_length: usize,
    /// Number of stop symbols present.
    pub stop_count: usize,
    /// Deterministic composition summary excluding gaps.
    pub composition: ResidueComposition,
    /// Estimated molecular weight in Daltons.
    pub molecular_weight: f64,
}

/// Structured `pepstats` outcome.
#[derive(Clone, Debug, PartialEq)]
pub struct PepstatsOutcome {
    /// Source input.
    pub input: SequenceInput,
    /// Per-record protein statistics.
    pub records: Vec<PepstatsRecord>,
}

/// Returns `pepstats` help text.
#[must_use]
pub fn pepstats_help() -> &'static str {
    "Usage: emboss-rs pepstats <protein-input>\n\nReport deterministic protein summary statistics for each sequence record. The v1 implementation includes raw sequence length, residue length excluding stop symbols, amino-acid composition counts and frequencies, and an average-residue molecular-weight estimate with water added once per chain. Isoelectric-point estimation is deferred."
}

/// Executes `pepstats`.
pub fn run_pepstats(params: PepstatsParams) -> Result<PepstatsOutcome, ToolExecutionError> {
    let records = load_sequence_records(&params.input)?
        .into_iter()
        .map(|record| -> Result<PepstatsRecord, ToolExecutionError> {
            validate_protein_record(&record)?;

            let composition = ResidueComposition::from_sequence(record.residues());
            let stop_count = composition.count_for('*');
            let residue_length = composition.counted_symbols().saturating_sub(stop_count);
            let molecular_weight =
                protein_molecular_weight(record.residues()).map_err(|error| {
                    PlatformError::new(ErrorCategory::Validation, error.to_string())
                        .with_code("tools.pepstats.residue.unsupported")
                })?;

            Ok(PepstatsRecord {
                record_id: record.identifier().accession().to_owned(),
                sequence_length: record.len(),
                residue_length,
                stop_count,
                composition,
                molecular_weight,
            })
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(PepstatsOutcome {
        input: params.input,
        records,
    })
}

fn validate_protein_record(record: &emboss_core::SequenceRecord) -> Result<(), ToolExecutionError> {
    if record.molecule().is_nucleotide() {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!(
                "pepstats expects protein input but '{}' was classified as {}",
                record.identifier().accession(),
                record.molecule()
            ),
        )
        .with_code("tools.pepstats.input.not_protein"));
    }

    if record.molecule().is_protein() || looks_like_protein_record(record) {
        return Ok(());
    }

    if looks_like_nucleotide_record(record) {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!(
                "pepstats expects protein input but '{}' was classified as {}",
                record.identifier().accession(),
                record.molecule()
            ),
        )
        .with_code("tools.pepstats.input.not_protein"));
    }

    if !record.molecule().is_protein() {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!(
                "pepstats expects protein input but '{}' could not be validated as protein",
                record.identifier().accession()
            ),
        )
        .with_code("tools.pepstats.input.not_protein"));
    }

    Ok(())
}

fn looks_like_protein_record(record: &emboss_core::SequenceRecord) -> bool {
    Alphabet::Protein
        .validate(MoleculeKind::Protein, record.residues())
        .is_ok()
}

fn looks_like_nucleotide_record(record: &emboss_core::SequenceRecord) -> bool {
    Alphabet::Dna
        .validate(MoleculeKind::Dna, record.residues())
        .is_ok()
        || Alphabet::Rna
            .validate(MoleculeKind::Rna, record.residues())
            .is_ok()
}

#[cfg(test)]
mod tests {
    use super::{PepstatsParams, run_pepstats};
    use crate::sequence_stream::SequenceInput;
    use std::fs;

    fn write_temp_sequence_file(name: &str, contents: &str) -> std::path::PathBuf {
        let path = std::env::temp_dir().join(format!(
            "emboss-rs-pepstats-{name}-{}-{}.fasta",
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
    fn reports_basic_protein_statistics() {
        let outcome = run_pepstats(PepstatsParams {
            input: SequenceInput::new(fixture("protein_stats_records.fasta")),
        })
        .expect("pepstats should succeed");

        assert_eq!(outcome.records.len(), 2);
        assert_eq!(outcome.records[0].record_id, "pepA");
        assert_eq!(outcome.records[0].sequence_length, 3);
        assert_eq!(outcome.records[0].residue_length, 2);
        assert_eq!(outcome.records[0].stop_count, 1);
        assert_eq!(outcome.records[0].composition.count_for('M'), 1);
        assert!((outcome.records[0].molecular_weight - 220.286_68).abs() < 1e-6);
    }

    #[test]
    fn rejects_ambiguous_residues_for_mass_estimation() {
        let path = write_temp_sequence_file("ambiguous", ">pep\nMZX\n");
        let error = run_pepstats(PepstatsParams {
            input: SequenceInput::new(path.clone()),
        })
        .expect_err("unsupported ambiguous residues should fail");
        fs::remove_file(path).ok();

        assert!(error.to_string().contains("unsupported residue"));
    }

    #[test]
    fn rejects_nucleotide_like_input() {
        let error = run_pepstats(PepstatsParams {
            input: SequenceInput::new(fixture("nucleotide_pattern_records.fasta")),
        })
        .expect_err("nucleotide input should fail");

        assert!(error.to_string().contains("expects protein input"));
    }

    #[test]
    fn rejects_empty_sequence_record() {
        let path = write_temp_sequence_file("empty", ">pep\n\n");
        let error = run_pepstats(PepstatsParams {
            input: SequenceInput::new(path.clone()),
        })
        .expect_err("empty sequence record should fail through shared parsing");
        fs::remove_file(path).ok();

        assert!(error.to_string().contains("has no sequence residues"));
    }
}
