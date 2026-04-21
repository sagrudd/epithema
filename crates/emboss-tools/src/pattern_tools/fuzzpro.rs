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
    "Usage: emboss-rs fuzzpro <protein-input> <pattern>\n\nSearch protein sequence records for a deterministic literal pattern. Supported pattern syntax is exact amino-acid symbols plus X as a single-residue wildcard. Overlapping matches are reported. User-facing hit coordinates are reported as 1-based inclusive."
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

#[cfg(test)]
mod tests {
    use super::{FuzzproParams, run_fuzzpro};
    use crate::sequence_stream::SequenceInput;
    use emboss_core::ProteinPattern;
    use std::fs;

    fn write_temp_sequence_file(name: &str, contents: &str) -> std::path::PathBuf {
        let path = std::env::temp_dir().join(format!(
            "emboss-rs-fuzzpro-{name}-{}-{}.fasta",
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
    fn reports_exact_and_wildcard_matches_in_stable_order() {
        let outcome = run_fuzzpro(FuzzproParams {
            input: SequenceInput::new(fixture("protein_records.fasta")),
            pattern: ProteinPattern::parse("MX").expect("pattern should parse"),
        })
        .expect("fuzzpro should succeed");

        assert_eq!(outcome.hits.len(), 1);
        assert_eq!(outcome.hits[0].record_id, "protA");
        assert_eq!(outcome.hits[0].start, 0);
        assert_eq!(outcome.hits[0].end, 2);
        assert_eq!(outcome.hits[0].matched, "MA");
    }

    #[test]
    fn reports_overlapping_matches() {
        let path = write_temp_sequence_file("overlap", ">ovl\nMAMAM\n");
        let outcome = run_fuzzpro(FuzzproParams {
            input: SequenceInput::new(path.clone()),
            pattern: ProteinPattern::parse("MAM").expect("pattern should parse"),
        })
        .expect("fuzzpro should succeed");
        fs::remove_file(path).ok();

        assert_eq!(outcome.hits.len(), 2);
        assert_eq!(outcome.hits[0].start, 0);
        assert_eq!(outcome.hits[0].end, 3);
        assert_eq!(outcome.hits[1].start, 2);
        assert_eq!(outcome.hits[1].end, 5);
    }

    #[test]
    fn returns_no_hits_when_pattern_is_absent() {
        let outcome = run_fuzzpro(FuzzproParams {
            input: SequenceInput::new(fixture("protein_records.fasta")),
            pattern: ProteinPattern::parse("QQQ").expect("pattern should parse"),
        })
        .expect("fuzzpro should succeed");

        assert!(outcome.hits.is_empty());
    }

    #[test]
    fn rejects_nucleotide_input() {
        let error = run_fuzzpro(FuzzproParams {
            input: SequenceInput::new(fixture("three_records.fasta")),
            pattern: ProteinPattern::parse("MX").expect("pattern should parse"),
        })
        .expect_err("nucleotide input should fail");

        assert!(error.to_string().contains("expects protein input"));
    }
}
