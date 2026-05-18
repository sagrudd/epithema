//! `palindrome` implementation.

use super::inverted_repeats::exact_palindromes;
use crate::sequence_stream::{SequenceInput, ToolExecutionError, load_sequence_records};

/// Typed parameters for `palindrome`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PalindromeParams {
    /// Local nucleotide input path.
    pub input: SequenceInput,
    /// Minimum palindrome length.
    pub min_length: usize,
    /// Maximum palindrome length.
    pub max_length: usize,
}

/// One palindromic hit.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PalindromeHit {
    /// Source record identifier.
    pub record_id: String,
    /// Zero-based inclusive start.
    pub start: usize,
    /// Zero-based half-open end.
    pub end: usize,
    /// Matched palindromic sequence.
    pub matched: String,
}

/// Structured `palindrome` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PalindromeOutcome {
    /// Source input.
    pub input: SequenceInput,
    /// Minimum palindrome length.
    pub min_length: usize,
    /// Maximum palindrome length.
    pub max_length: usize,
    /// Stable ordered hits.
    pub hits: Vec<PalindromeHit>,
}

/// Returns `palindrome` help text.
#[must_use]
pub fn palindrome_help() -> &'static str {
    "Usage: emboss-rs palindrome <nucleotide-input> [--min-length <length>] [--max-length <length>]\n\nReport exact reverse-complement palindromic windows in nucleotide sequence records. v1 searches the forward sequence only, reports all palindromic windows within the inclusive length range, and uses 1-based inclusive coordinates in rendered output."
}

/// Executes `palindrome`.
pub fn run_palindrome(params: PalindromeParams) -> Result<PalindromeOutcome, ToolExecutionError> {
    let mut hits = Vec::new();

    for record in load_sequence_records(&params.input)? {
        hits.extend(
            exact_palindromes(
                "palindrome",
                record.molecule(),
                record.residues(),
                params.min_length,
                params.max_length,
            )?
            .into_iter()
            .map(|region| PalindromeHit {
                record_id: record.identifier().accession().to_owned(),
                start: region.start,
                end: region.end,
                matched: region.matched,
            }),
        );
    }

    Ok(PalindromeOutcome {
        input: params.input,
        min_length: params.min_length,
        max_length: params.max_length,
        hits,
    })
}

#[cfg(test)]
mod tests {
    use super::{PalindromeParams, run_palindrome};
    use crate::sequence_stream::SequenceInput;
    use std::fs;

    fn write_temp_sequence_file(name: &str, contents: &str) -> std::path::PathBuf {
        let path = std::env::temp_dir().join(format!(
            "emboss-rs-palindrome-{name}-{}-{}.fasta",
            std::process::id(),
            std::thread::current().name().unwrap_or("main")
        ));
        fs::write(&path, contents).expect("temporary sequence fixture should be written");
        path
    }

    #[test]
    fn reports_exact_palindromic_windows() {
        let path = write_temp_sequence_file("pal", ">pal\nATGCATTA\n");
        let outcome = run_palindrome(PalindromeParams {
            input: SequenceInput::new(path.clone()),
            min_length: 6,
            max_length: 6,
        })
        .expect("palindrome should succeed");
        fs::remove_file(path).ok();

        assert_eq!(outcome.hits.len(), 1);
        assert_eq!(outcome.hits[0].start, 0);
        assert_eq!(outcome.hits[0].end, 6);
        assert_eq!(outcome.hits[0].matched, "ATGCAT");
    }
}
