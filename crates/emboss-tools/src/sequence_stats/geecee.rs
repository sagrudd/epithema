//! `geecee` implementation.

use emboss_core::{Alphabet, GcSummary, MoleculeKind};
use emboss_diagnostics::{ErrorCategory, PlatformError};

use crate::sequence_stream::{SequenceInput, ToolExecutionError, load_sequence_records};

/// Typed parameters for `geecee`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GeeceeParams {
    /// Local nucleotide input path.
    pub input: SequenceInput,
}

/// One per-record GC summary.
#[derive(Clone, Debug, PartialEq)]
pub struct GeeceeRecord {
    /// Record identifier.
    pub record_id: String,
    /// Raw sequence length.
    pub sequence_length: usize,
    /// GC summary over the record.
    pub gc: GcSummary,
}

/// Structured `geecee` outcome.
#[derive(Clone, Debug, PartialEq)]
pub struct GeeceeOutcome {
    /// Source input.
    pub input: SequenceInput,
    /// Per-record GC summaries.
    pub records: Vec<GeeceeRecord>,
    /// Aggregate GC summary across all records.
    pub aggregate: GcSummary,
}

/// Returns `geecee` help text.
#[must_use]
pub fn geecee_help() -> &'static str {
    "Usage: emboss-rs geecee <nucleotide-input>\n\nReport deterministic GC counts and GC percentages for nucleotide sequence records. The v1 implementation reports both per-record rows and an aggregate summary across all records. GC percentage is computed over canonical A/C/G/T/U symbols only; ambiguous symbols such as N are excluded from the denominator and reported separately."
}

/// Executes `geecee`.
pub fn run_geecee(params: GeeceeParams) -> Result<GeeceeOutcome, ToolExecutionError> {
    let mut aggregate = GcSummary::default();
    let records = load_sequence_records(&params.input)?
        .into_iter()
        .map(|record| -> Result<GeeceeRecord, ToolExecutionError> {
            validate_nucleotide_record(&record)?;

            let gc = GcSummary::from_sequence(record.residues());
            aggregate.merge(&gc);
            Ok(GeeceeRecord {
                record_id: record.identifier().accession().to_owned(),
                sequence_length: record.len(),
                gc,
            })
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(GeeceeOutcome {
        input: params.input,
        records,
        aggregate,
    })
}

fn validate_nucleotide_record(
    record: &emboss_core::SequenceRecord,
) -> Result<(), ToolExecutionError> {
    if record.molecule().is_protein()
        || (!record.molecule().is_nucleotide() && !looks_like_nucleotide_record(record))
    {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!(
                "geecee expects nucleotide input but '{}' was classified as {}",
                record.identifier().accession(),
                record.molecule()
            ),
        )
        .with_code("tools.geecee.input.not_nucleotide"));
    }

    Ok(())
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
    use super::{GeeceeParams, run_geecee};
    use crate::sequence_stream::SequenceInput;
    use std::fs;

    fn write_temp_sequence_file(name: &str, contents: &str) -> std::path::PathBuf {
        let path = std::env::temp_dir().join(format!(
            "emboss-rs-geecee-{name}-{}-{}.fasta",
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
    fn reports_gc_statistics_for_nucleotide_records() {
        let outcome = run_geecee(GeeceeParams {
            input: SequenceInput::new(fixture("nucleotide_pattern_records.fasta")),
        })
        .expect("geecee should succeed");

        assert_eq!(outcome.records.len(), 2);
        assert_eq!(outcome.records[0].record_id, "nucA");
        assert_eq!(outcome.records[0].gc.gc_symbols, 4);
        assert_eq!(outcome.records[0].gc.canonical_symbols, 8);
        assert!((outcome.records[0].gc.gc_percent() - 50.0).abs() < 1e-9);
        assert_eq!(outcome.aggregate.gc_symbols, 7);
        assert_eq!(outcome.aggregate.canonical_symbols, 14);
    }

    #[test]
    fn excludes_ambiguous_symbols_from_gc_denominator() {
        let path = write_temp_sequence_file("ambiguous", ">amb\nACGNNT-\n");
        let outcome = run_geecee(GeeceeParams {
            input: SequenceInput::new(path.clone()),
        })
        .expect("geecee should succeed");
        fs::remove_file(path).ok();

        assert_eq!(outcome.records.len(), 1);
        assert_eq!(outcome.records[0].gc.gc_symbols, 2);
        assert_eq!(outcome.records[0].gc.canonical_symbols, 4);
        assert_eq!(outcome.records[0].gc.ambiguous_symbols, 2);
        assert_eq!(outcome.records[0].gc.ignored_gap_symbols, 1);
        assert!((outcome.records[0].gc.gc_percent() - 50.0).abs() < 1e-9);
    }

    #[test]
    fn accepts_rna_records() {
        let path = write_temp_sequence_file("rna", ">rna\nACGUUGC\n");
        let outcome = run_geecee(GeeceeParams {
            input: SequenceInput::new(path.clone()),
        })
        .expect("geecee should succeed");
        fs::remove_file(path).ok();

        assert_eq!(outcome.records[0].gc.gc_symbols, 4);
        assert_eq!(outcome.records[0].gc.canonical_symbols, 7);
        assert!((outcome.records[0].gc.gc_percent() - (4.0 * 100.0 / 7.0)).abs() < 1e-9);
    }

    #[test]
    fn rejects_non_nucleotide_input() {
        let error = run_geecee(GeeceeParams {
            input: SequenceInput::new(fixture("protein_stats_records.fasta")),
        })
        .expect_err("protein input should fail");

        assert!(error.to_string().contains("expects nucleotide input"));
    }

    #[test]
    fn rejects_empty_sequence_record() {
        let path = write_temp_sequence_file("empty-record", ">empty\n\n");
        let error = run_geecee(GeeceeParams {
            input: SequenceInput::new(path.clone()),
        })
        .expect_err("empty sequence records should fail through shared parsing");
        fs::remove_file(path).ok();

        assert!(error.to_string().contains("has no sequence residues"));
    }
}
