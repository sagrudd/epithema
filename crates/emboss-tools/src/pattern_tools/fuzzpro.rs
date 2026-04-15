//! `fuzzpro` implementation.

use emboss_core::{PatternMatch, ProteinPattern};
use emboss_diagnostics::{ErrorCategory, PlatformError};

use crate::sequence_stream::{SequenceInput, ToolExecutionError, load_sequence_records};

/// Typed parameters for `fuzzpro`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FuzzproParams {
    /// Local protein input path.
    pub input: SequenceInput,
    /// Parsed protein pattern.
    pub pattern: ProteinPattern,
}

/// One protein pattern hit.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FuzzproHit {
    /// Source record identifier.
    pub record_id: String,
    /// Matched pattern text.
    pub pattern: String,
    /// Zero-based inclusive start.
    pub start: usize,
    /// Zero-based half-open end.
    pub end: usize,
    /// Matched input slice.
    pub matched: String,
}

/// Structured `fuzzpro` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FuzzproOutcome {
    /// Source input.
    pub input: SequenceInput,
    /// Pattern used for the search.
    pub pattern: String,
    /// Stable ordered hits.
    pub hits: Vec<FuzzproHit>,
}

/// Returns `fuzzpro` help text.
#[must_use]
pub fn fuzzpro_help() -> &'static str {
    "Usage: emboss-rs fuzzpro <protein-input> <pattern>\n\nSearch protein sequence records for a deterministic literal pattern. Supported pattern syntax is exact amino-acid symbols plus X as a single-residue wildcard. User-facing hit coordinates are reported as 1-based inclusive."
}

/// Executes `fuzzpro`.
pub fn run_fuzzpro(params: FuzzproParams) -> Result<FuzzproOutcome, ToolExecutionError> {
    let pattern_text = params.pattern.raw().to_owned();
    let mut hits = Vec::new();

    for record in load_sequence_records(&params.input)? {
        validate_protein_record("fuzzpro", &record)?;
        hits.extend(
            params
                .pattern
                .scan(record.residues())
                .into_iter()
                .map(|hit| build_hit(record.identifier().accession(), &pattern_text, hit)),
        );
    }

    Ok(FuzzproOutcome {
        input: params.input,
        pattern: pattern_text,
        hits,
    })
}

fn validate_protein_record(
    tool: &str,
    record: &emboss_core::SequenceRecord,
) -> Result<(), ToolExecutionError> {
    if record.molecule().is_nucleotide() {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!(
                "{tool} expects protein input but '{}' was classified as {}",
                record.identifier().accession(),
                record.molecule()
            ),
        )
        .with_code(format!("tools.{tool}.input.not_protein")));
    }

    Ok(())
}

fn build_hit(record_id: &str, pattern: &str, hit: PatternMatch) -> FuzzproHit {
    FuzzproHit {
        record_id: record_id.to_owned(),
        pattern: pattern.to_owned(),
        start: hit.start(),
        end: hit.end(),
        matched: hit.matched().to_owned(),
    }
}
