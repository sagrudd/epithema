//! `fuzznuc` implementation.

use emboss_core::{NucleotidePattern, PatternMatch};
use emboss_diagnostics::{ErrorCategory, PlatformError};

use crate::sequence_stream::{SequenceInput, ToolExecutionError, load_sequence_records};

/// Typed parameters for `fuzznuc`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FuzznucParams {
    /// Local nucleotide input path.
    pub input: SequenceInput,
    /// Parsed nucleotide search pattern.
    pub pattern: NucleotidePattern,
}

/// One nucleotide pattern hit.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FuzznucHit {
    /// Source record identifier.
    pub record_id: String,
    /// Matched pattern text.
    pub pattern: String,
    /// Strand label for the current search policy.
    pub strand: String,
    /// Zero-based inclusive start.
    pub start: usize,
    /// Zero-based half-open end.
    pub end: usize,
    /// Matched input slice.
    pub matched: String,
}

/// Structured `fuzznuc` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FuzznucOutcome {
    /// Source input.
    pub input: SequenceInput,
    /// Pattern used for the search.
    pub pattern: String,
    /// Stable ordered hits.
    pub hits: Vec<FuzznucHit>,
}

/// Returns `fuzznuc` help text.
#[must_use]
pub fn fuzznuc_help() -> &'static str {
    "Usage: emboss-rs fuzznuc <nucleotide-input> <pattern>\n\nSearch nucleotide sequence records for a deterministic forward-strand pattern. Supported pattern syntax is exact literal nucleotide text plus IUPAC ambiguity symbols such as N, R, and Y. User-facing hit coordinates are reported as 1-based inclusive."
}

/// Executes `fuzznuc`.
pub fn run_fuzznuc(params: FuzznucParams) -> Result<FuzznucOutcome, ToolExecutionError> {
    let pattern_text = params.pattern.raw().to_owned();
    let mut hits = Vec::new();

    for record in load_sequence_records(&params.input)? {
        validate_nucleotide_record("fuzznuc", &record)?;
        hits.extend(
            params
                .pattern
                .scan(record.residues())
                .into_iter()
                .map(|hit| build_hit(record.identifier().accession(), &pattern_text, hit)),
        );
    }

    Ok(FuzznucOutcome {
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

fn build_hit(record_id: &str, pattern: &str, hit: PatternMatch) -> FuzznucHit {
    FuzznucHit {
        record_id: record_id.to_owned(),
        pattern: pattern.to_owned(),
        strand: "forward".to_owned(),
        start: hit.start(),
        end: hit.end(),
        matched: hit.matched().to_owned(),
    }
}
