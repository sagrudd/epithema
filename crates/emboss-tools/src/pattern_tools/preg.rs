//! `preg` implementation.

use emboss_diagnostics::{ErrorCategory, PlatformError};

use super::protein_regex::CompiledProteinRegex;
use crate::sequence_stream::{SequenceInput, ToolExecutionError, load_sequence_records};

/// Typed parameters for `preg`.
#[derive(Clone, Debug)]
pub struct PregParams {
    /// Local protein input path.
    pub input: SequenceInput,
    /// Parsed protein regular expression.
    pub pattern: CompiledProteinRegex,
}

/// One protein regex hit.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PregHit {
    /// Source record identifier.
    pub record_id: String,
    /// Searched regular expression.
    pub pattern: String,
    /// Zero-based inclusive start.
    pub start: usize,
    /// Zero-based half-open end.
    pub end: usize,
    /// Matched normalized input slice.
    pub matched: String,
}

/// Structured `preg` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PregOutcome {
    /// Source input.
    pub input: SequenceInput,
    /// Pattern used for the search.
    pub pattern: String,
    /// Stable ordered hits.
    pub hits: Vec<PregHit>,
}

/// Returns `preg` help text.
#[must_use]
pub fn preg_help() -> &'static str {
    "Usage: emboss-rs preg <protein-input> <regex>\n\nSearch protein sequence records with a deterministic bounded regular expression. v1 accepts Rust-regex-compatible ASCII patterns, rejects empty or zero-width expressions, scans forward only, and reports overlapping matches with 1-based inclusive coordinates."
}

/// Executes `preg`.
pub fn run_preg(params: PregParams) -> Result<PregOutcome, ToolExecutionError> {
    let pattern_text = params.pattern.raw().to_owned();
    let mut hits = Vec::new();

    for record in load_sequence_records(&params.input)? {
        validate_protein_record("preg", &record)?;
        hits.extend(
            params
                .pattern
                .scan(record.residues())
                .into_iter()
                .map(|hit| PregHit {
                    record_id: record.identifier().accession().to_owned(),
                    pattern: pattern_text.clone(),
                    start: hit.start(),
                    end: hit.end(),
                    matched: hit.matched().to_owned(),
                }),
        );
    }

    Ok(PregOutcome {
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

#[cfg(test)]
mod tests {
    use super::{PregParams, run_preg};
    use crate::pattern_tools::protein_regex::CompiledProteinRegex;
    use crate::sequence_stream::SequenceInput;
    use std::fs;

    fn write_temp_sequence_file(name: &str, contents: &str) -> std::path::PathBuf {
        let path = std::env::temp_dir().join(format!(
            "emboss-rs-preg-{name}-{}-{}.fasta",
            std::process::id(),
            std::thread::current().name().unwrap_or("main")
        ));
        fs::write(&path, contents).expect("temporary fixture should be written");
        path
    }

    #[test]
    fn reports_overlapping_regex_hits() {
        let path = write_temp_sequence_file("overlap", ">pregA\nMAMAM\n");
        let outcome = run_preg(PregParams {
            input: SequenceInput::new(path.clone()),
            pattern: CompiledProteinRegex::parse("MAM").expect("regex should parse"),
        })
        .expect("preg should succeed");
        fs::remove_file(path).ok();

        assert_eq!(outcome.hits.len(), 2);
        assert_eq!(outcome.hits[0].start, 0);
        assert_eq!(outcome.hits[1].start, 2);
    }

    #[test]
    fn rejects_zero_width_regex() {
        let error = CompiledProteinRegex::parse(".*").expect_err("zero-width capable regex");
        assert!(
            error
                .to_string()
                .contains("must consume at least one residue")
        );
    }
}
