//! `fuzztran` implementation.

use emboss_core::{Alphabet, MoleculeKind, PatternMatch, ProteinPattern, translate_dna_frame};
use emboss_diagnostics::{ErrorCategory, PlatformError};

use crate::sequence_stream::{SequenceInput, ToolExecutionError, load_sequence_records};

/// Typed parameters for `fuzztran`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FuzztranParams {
    /// Local nucleotide input path.
    pub input: SequenceInput,
    /// Parsed protein search pattern.
    pub pattern: ProteinPattern,
}

/// One translated-frame pattern hit.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FuzztranHit {
    /// Source record identifier.
    pub record_id: String,
    /// Matched pattern text.
    pub pattern: String,
    /// User-facing forward frame label, 1 through 3.
    pub frame: usize,
    /// Zero-based inclusive amino-acid start within the translated frame.
    pub amino_start: usize,
    /// Zero-based half-open amino-acid end within the translated frame.
    pub amino_end: usize,
    /// Zero-based inclusive nucleotide start in the original sequence.
    pub nucleotide_start: usize,
    /// Zero-based half-open nucleotide end in the original sequence.
    pub nucleotide_end: usize,
    /// Matched translated amino-acid text.
    pub matched: String,
}

/// Structured `fuzztran` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FuzztranOutcome {
    /// Source input.
    pub input: SequenceInput,
    /// Pattern used for the search.
    pub pattern: String,
    /// Stable ordered hits.
    pub hits: Vec<FuzztranHit>,
}

/// Returns `fuzztran` help text.
#[must_use]
pub fn fuzztran_help() -> &'static str {
    "Usage: emboss-rs fuzztran <nucleotide-input> <protein-pattern>\n\nTranslate nucleotide sequence records in all three forward reading frames and search the translated amino-acid sequences for a deterministic protein pattern. Supported protein pattern syntax is exact amino-acid symbols plus X as a single-residue wildcard. Overlapping translated matches are reported. Hits are reported in both translated amino-acid coordinates and original nucleotide coordinates using 1-based inclusive reporting."
}

/// Executes `fuzztran`.
pub fn run_fuzztran(params: FuzztranParams) -> Result<FuzztranOutcome, ToolExecutionError> {
    let pattern_text = params.pattern.raw().to_owned();
    let mut hits = Vec::new();

    for record in load_sequence_records(&params.input)? {
        validate_nucleotide_record("fuzztran", &record)?;
        for frame_offset in 0..3 {
            let translated =
                translate_dna_frame(record.residues(), frame_offset).map_err(|error| {
                    PlatformError::new(ErrorCategory::Validation, error.to_string())
                        .with_code("tools.fuzztran.translation.failed")
                })?;
            hits.extend(params.pattern.scan(&translated).into_iter().map(|hit| {
                build_hit(
                    record.identifier().accession(),
                    &pattern_text,
                    frame_offset,
                    hit,
                )
            }));
        }
    }

    Ok(FuzztranOutcome {
        input: params.input,
        pattern: pattern_text,
        hits,
    })
}

fn validate_nucleotide_record(
    tool: &str,
    record: &emboss_core::SequenceRecord,
) -> Result<(), ToolExecutionError> {
    if record.molecule().is_protein()
        || (!record.molecule().is_nucleotide() && !looks_like_nucleotide_record(record))
    {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!(
                "{tool} expects nucleotide input but '{}' was classified as {}",
                record.identifier().accession(),
                record.molecule()
            ),
        )
        .with_code(format!("tools.{tool}.input.not_nucleotide")));
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

fn build_hit(
    record_id: &str,
    pattern: &str,
    frame_offset: usize,
    hit: PatternMatch,
) -> FuzztranHit {
    let amino_start = hit.start();
    let amino_end = hit.end();
    let nucleotide_start = frame_offset + amino_start * 3;
    let nucleotide_end = frame_offset + amino_end * 3;

    FuzztranHit {
        record_id: record_id.to_owned(),
        pattern: pattern.to_owned(),
        frame: frame_offset + 1,
        amino_start,
        amino_end,
        nucleotide_start,
        nucleotide_end,
        matched: hit.matched().to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::{FuzztranParams, run_fuzztran};
    use crate::sequence_stream::SequenceInput;
    use emboss_core::ProteinPattern;
    use std::fs;

    fn write_temp_sequence_file(name: &str, contents: &str) -> std::path::PathBuf {
        let path = std::env::temp_dir().join(format!(
            "emboss-rs-fuzztran-{name}-{}-{}.fasta",
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
    fn reports_forward_frame_match_with_amino_and_nucleotide_coordinates() {
        let outcome = run_fuzztran(FuzztranParams {
            input: SequenceInput::new(fixture("checktrans_nucleotide.fasta")),
            pattern: ProteinPattern::parse("MA").expect("pattern should parse"),
        })
        .expect("fuzztran should succeed");

        assert_eq!(outcome.hits.len(), 1);
        assert_eq!(outcome.hits[0].record_id, "cdsA");
        assert_eq!(outcome.hits[0].frame, 1);
        assert_eq!(outcome.hits[0].amino_start, 0);
        assert_eq!(outcome.hits[0].amino_end, 2);
        assert_eq!(outcome.hits[0].nucleotide_start, 0);
        assert_eq!(outcome.hits[0].nucleotide_end, 6);
        assert_eq!(outcome.hits[0].matched, "MA");
    }

    #[test]
    fn reports_overlapping_translated_matches() {
        let path = write_temp_sequence_file("overlap", ">ovl\nATGGCTATGGCTATG\n");
        let outcome = run_fuzztran(FuzztranParams {
            input: SequenceInput::new(path.clone()),
            pattern: ProteinPattern::parse("MAM").expect("pattern should parse"),
        })
        .expect("fuzztran should succeed");
        fs::remove_file(path).ok();

        assert_eq!(outcome.hits.len(), 2);
        assert_eq!(outcome.hits[0].frame, 1);
        assert_eq!(outcome.hits[0].amino_start, 0);
        assert_eq!(outcome.hits[0].amino_end, 3);
        assert_eq!(outcome.hits[0].nucleotide_start, 0);
        assert_eq!(outcome.hits[0].nucleotide_end, 9);
        assert_eq!(outcome.hits[1].amino_start, 2);
        assert_eq!(outcome.hits[1].amino_end, 5);
        assert_eq!(outcome.hits[1].nucleotide_start, 6);
        assert_eq!(outcome.hits[1].nucleotide_end, 15);
    }

    #[test]
    fn returns_no_hits_when_pattern_is_absent() {
        let outcome = run_fuzztran(FuzztranParams {
            input: SequenceInput::new(fixture("checktrans_nucleotide.fasta")),
            pattern: ProteinPattern::parse("QQQ").expect("pattern should parse"),
        })
        .expect("fuzztran should succeed");

        assert!(outcome.hits.is_empty());
    }

    #[test]
    fn rejects_protein_input() {
        let error = run_fuzztran(FuzztranParams {
            input: SequenceInput::new(fixture("protein_records.fasta")),
            pattern: ProteinPattern::parse("MA").expect("pattern should parse"),
        })
        .expect_err("protein input should fail");

        assert!(error.to_string().contains("invalid codon"));
    }

    #[test]
    fn rejects_ambiguous_nucleotide_translation() {
        let path = write_temp_sequence_file("ambiguous", ">amb\nATGNNNTAA\n");
        let error = run_fuzztran(FuzztranParams {
            input: SequenceInput::new(path.clone()),
            pattern: ProteinPattern::parse("MX").expect("pattern should parse"),
        })
        .expect_err("ambiguous nucleotide translation should fail");
        fs::remove_file(path).ok();

        assert!(error.to_string().contains("invalid codon"));
    }
}
