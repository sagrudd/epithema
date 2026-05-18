//! `oddcomp` implementation.

use emboss_core::{Alphabet, MoleculeKind, SequenceRecord};
use emboss_diagnostics::{ErrorCategory, PlatformError};

use crate::sequence_stream::{SequenceInput, ToolExecutionError, load_sequence_records};

/// Typed parameters for `oddcomp`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OddcompParams {
    /// Local protein input path.
    pub input: SequenceInput,
    /// One or more exact protein words to count.
    pub query_words: Vec<String>,
}

/// One per-record per-word result row.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OddcompRow {
    /// Stable record identifier.
    pub record_id: String,
    /// Queried word.
    pub query_word: String,
    /// Word length.
    pub word_length: usize,
    /// Overlapping exact count.
    pub count: usize,
    /// Whether the record contains the word at least once.
    pub contains: bool,
    /// Number of counted windows for this word length.
    pub counted_windows: usize,
}

/// Structured `oddcomp` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OddcompOutcome {
    /// Source input.
    pub input: SequenceInput,
    /// Stable normalized query words in user order.
    pub query_words: Vec<String>,
    /// Per-record rows in source order and query order.
    pub rows: Vec<OddcompRow>,
}

/// Returns `oddcomp` help text.
#[must_use]
pub fn oddcomp_help() -> &'static str {
    "Usage: emboss-rs oddcomp <protein-input> --word <residue_word> [--word <residue_word> ...]\n\nReport deterministic exact protein word-composition counts for one or more query words. The v1 implementation counts overlapping literal words, preserves record order and query order, and emits one stable row per record per query word."
}

/// Executes `oddcomp`.
pub fn run_oddcomp(params: OddcompParams) -> Result<OddcompOutcome, ToolExecutionError> {
    let records = load_sequence_records(&params.input)?;
    let query_words = normalize_query_words(params.query_words)?;

    let mut rows = Vec::new();
    for record in records {
        validate_protein_record(&record)?;
        for query_word in &query_words {
            let (count, counted_windows) = count_query_word(record.residues(), query_word);
            rows.push(OddcompRow {
                record_id: record.identifier().accession().to_owned(),
                query_word: query_word.clone(),
                word_length: query_word.len(),
                count,
                contains: count > 0,
                counted_windows,
            });
        }
    }

    Ok(OddcompOutcome {
        input: params.input,
        query_words,
        rows,
    })
}

fn normalize_query_words(query_words: Vec<String>) -> Result<Vec<String>, ToolExecutionError> {
    let mut normalized = Vec::new();

    for query_word in query_words {
        if query_word.is_empty() {
            return Err(PlatformError::new(
                ErrorCategory::Validation,
                "oddcomp query words must not be empty",
            )
            .with_code("tools.oddcomp.word.empty")
            .into());
        }
        let normalized_word = query_word.to_ascii_uppercase();
        Alphabet::Protein
            .validate(MoleculeKind::Protein, &normalized_word)
            .map_err(|error| {
                PlatformError::new(
                    ErrorCategory::Validation,
                    format!("oddcomp query word '{query_word}' is not a valid protein word"),
                )
                .with_code("tools.oddcomp.word.invalid")
                .with_detail(error.to_string())
            })?;
        normalized.push(normalized_word);
    }

    Ok(normalized)
}

fn validate_protein_record(record: &SequenceRecord) -> Result<(), ToolExecutionError> {
    if record.molecule().is_nucleotide() {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!(
                "oddcomp expects protein input but '{}' was classified as {}",
                record.identifier().accession(),
                record.molecule()
            ),
        )
        .with_code("tools.oddcomp.input.not_protein")
        .into());
    }

    Alphabet::Protein
        .validate(MoleculeKind::Protein, record.residues())
        .map_err(|error| {
            PlatformError::new(
                ErrorCategory::Validation,
                format!(
                    "oddcomp expects protein input but '{}' could not be validated as protein",
                    record.identifier().accession()
                ),
            )
            .with_code("tools.oddcomp.input.not_protein")
            .with_detail(error.to_string())
            .into()
        })
}

fn count_query_word(sequence: &str, query_word: &str) -> (usize, usize) {
    if sequence.len() < query_word.len() {
        return (0, 0);
    }

    let counted_windows = sequence.len() - query_word.len() + 1;
    let mut count = 0usize;
    for start in 0..=sequence.len() - query_word.len() {
        if &sequence[start..start + query_word.len()] == query_word {
            count += 1;
        }
    }

    (count, counted_windows)
}

#[cfg(test)]
mod tests {
    use super::{OddcompParams, run_oddcomp};
    use crate::sequence_stream::SequenceInput;

    #[test]
    fn reports_per_record_word_counts() {
        let outcome = run_oddcomp(OddcompParams {
            input: SequenceInput::new(
                "/Users/stephen/Projects/emboss-rs/crates/emboss-tools/tests/fixtures/oddcomp_records.fasta",
            ),
            query_words: vec!["MAM".to_owned(), "QQQ".to_owned()],
        })
        .expect("oddcomp should execute");

        assert_eq!(outcome.rows.len(), 6);
        assert_eq!(outcome.rows[0].record_id, "oddA");
        assert_eq!(outcome.rows[0].query_word, "MAM");
        assert_eq!(outcome.rows[0].count, 1);
        assert_eq!(outcome.rows[3].record_id, "oddB");
        assert_eq!(outcome.rows[3].query_word, "QQQ");
        assert_eq!(outcome.rows[3].count, 2);
    }

    #[test]
    fn rejects_nucleotide_input() {
        let error = run_oddcomp(OddcompParams {
            input: SequenceInput::new(
                "/Users/stephen/Projects/emboss-rs/crates/emboss-tools/tests/fixtures/three_records.fasta",
            ),
            query_words: vec!["AAA".to_owned()],
        })
        .expect_err("nucleotide input should fail");

        assert!(error.to_string().contains("expects protein input"));
    }

    #[test]
    fn rejects_invalid_query_words() {
        let error = run_oddcomp(OddcompParams {
            input: SequenceInput::new(
                "/Users/stephen/Projects/emboss-rs/crates/emboss-tools/tests/fixtures/oddcomp_records.fasta",
            ),
            query_words: vec!["AX?".to_owned()],
        })
        .expect_err("invalid query word should fail");

        assert!(error.to_string().contains("not a valid protein word"));
    }
}
