//! `fuzztran` implementation.

use emboss_core::{PatternMatch, ProteinPattern, translate_dna_frame};
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
    "Usage: emboss-rs fuzztran <nucleotide-input> <protein-pattern>\n\nTranslate nucleotide sequence records in all three forward reading frames and search the translated amino-acid sequences for a deterministic protein pattern. Supported protein pattern syntax is exact amino-acid symbols plus X as a single-residue wildcard. Hits are reported in both translated amino-acid coordinates and original nucleotide coordinates using 1-based inclusive reporting."
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
    if record.molecule().is_protein() {
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
