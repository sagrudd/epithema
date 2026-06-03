//! `dreg` implementation.

use super::nucleotide_regex::CompiledNucleotideRegex;
use emboss_diagnostics::{ErrorCategory, PlatformError};

use crate::sequence_stream::{SequenceInput, ToolExecutionError, load_sequence_records};

/// Typed parameters for `dreg`.
#[derive(Clone, Debug)]
pub struct DregParams {
    /// Local nucleotide input path.
    pub input: SequenceInput,
    /// Parsed nucleotide regex pattern.
    pub pattern: CompiledNucleotideRegex,
}

/// One nucleotide regex hit.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DregHit {
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

/// Structured `dreg` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DregOutcome {
    /// Source input.
    pub input: SequenceInput,
    /// Pattern used for the search.
    pub pattern: String,
    /// Stable ordered hits.
    pub hits: Vec<DregHit>,
}

/// Returns `dreg` help text.
#[must_use]
pub fn dreg_help() -> &'static str {
    "Usage: emboss-rs dreg <nucleotide-input> <pattern>\n\nSearch nucleotide sequence records for a deterministic forward-strand regular expression. v1 applies a bounded Rust regex case-insensitively over normalized nucleotide residues, reports overlapping matches, and uses 1-based inclusive coordinates in rendered output."
}

/// Executes `dreg`.
pub fn run_dreg(params: DregParams) -> Result<DregOutcome, ToolExecutionError> {
    let pattern_text = params.pattern.raw().to_owned();
    let mut hits = Vec::new();

    for record in load_sequence_records(&params.input)? {
        validate_nucleotide_record("dreg", &record)?;
        hits.extend(
            params
                .pattern
                .scan(record.residues())
                .into_iter()
                .map(|hit| DregHit {
                    record_id: record.identifier().accession().to_owned(),
                    pattern: pattern_text.clone(),
                    start: hit.start(),
                    end: hit.end(),
                    matched: hit.matched().to_owned(),
                }),
        );
    }

    Ok(DregOutcome {
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

#[cfg(test)]
mod tests {
    use super::{DregParams, run_dreg};
    use crate::pattern_tools::CompiledNucleotideRegex;
    use crate::sequence_stream::SequenceInput;
    use std::fs;

    fn write_temp_sequence_file(name: &str, contents: &str) -> std::path::PathBuf {
        let path = std::env::temp_dir().join(format!(
            "emboss-rs-dreg-{name}-{}-{}.fasta",
            std::process::id(),
            std::thread::current().name().unwrap_or("main")
        ));
        fs::write(&path, contents).expect("temporary sequence fixture should be written");
        path
    }

    #[test]
    fn reports_overlapping_regex_hits() {
        let path = write_temp_sequence_file("hits", ">nuc\nATATA\n");
        let outcome = run_dreg(DregParams {
            input: SequenceInput::new(path.clone()),
            pattern: CompiledNucleotideRegex::parse("ATA").expect("pattern should parse"),
        })
        .expect("dreg should succeed");
        fs::remove_file(path).ok();

        assert_eq!(outcome.hits.len(), 2);
        assert_eq!(outcome.hits[0].start, 0);
        assert_eq!(outcome.hits[1].start, 2);
    }

    #[test]
    fn rejects_zero_width_regex() {
        let error = CompiledNucleotideRegex::parse("A*").expect_err("zero-width regex should fail");
        assert!(
            error
                .to_string()
                .contains("must consume at least one residue")
        );
    }
}
