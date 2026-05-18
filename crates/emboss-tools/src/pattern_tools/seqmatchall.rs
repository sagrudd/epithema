//! `seqmatchall` implementation.

use super::exact_words::maximal_exact_regions;
use crate::sequence_stream::{SequenceInput, ToolExecutionError, load_sequence_records};

/// Typed parameters for `seqmatchall`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SeqmatchallParams {
    /// Multi-record sequence input.
    pub input: SequenceInput,
    /// Minimum exact shared word length.
    pub word_size: usize,
}

/// One exact shared region between one record pair.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SeqmatchallHit {
    /// Left record identifier.
    pub left_id: String,
    /// Right record identifier.
    pub right_id: String,
    /// Zero-based inclusive start in the left sequence.
    pub left_start: usize,
    /// Zero-based half-open end in the left sequence.
    pub left_end: usize,
    /// Zero-based inclusive start in the right sequence.
    pub right_start: usize,
    /// Zero-based half-open end in the right sequence.
    pub right_end: usize,
    /// Exact shared region.
    pub matched: String,
}

/// Structured `seqmatchall` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SeqmatchallOutcome {
    /// Source input.
    pub input: SequenceInput,
    /// Minimum exact shared word length.
    pub word_size: usize,
    /// Stable ordered pairwise exact shared regions.
    pub hits: Vec<SeqmatchallHit>,
}

/// Returns `seqmatchall` help text.
#[must_use]
pub fn seqmatchall_help() -> &'static str {
    "Usage: emboss-rs seqmatchall <input> [--word-size <length>]\n\nReport maximal exact ungapped shared regions for every record pair in one sequence set. v1 compares each pair once in input order, reports all regions meeting the minimum word size, and uses 1-based inclusive coordinates in rendered output."
}

/// Executes `seqmatchall`.
pub fn run_seqmatchall(
    params: SeqmatchallParams,
) -> Result<SeqmatchallOutcome, ToolExecutionError> {
    let records = load_sequence_records(&params.input)?;
    let mut hits = Vec::new();

    for left_index in 0..records.len() {
        for right_index in left_index + 1..records.len() {
            let left = &records[left_index];
            let right = &records[right_index];
            hits.extend(
                maximal_exact_regions("seqmatchall", left, right, params.word_size)?
                    .into_iter()
                    .map(|region| SeqmatchallHit {
                        left_id: left.identifier().accession().to_owned(),
                        right_id: right.identifier().accession().to_owned(),
                        left_start: region.left_start,
                        left_end: region.left_end,
                        right_start: region.right_start,
                        right_end: region.right_end,
                        matched: region.matched,
                    }),
            );
        }
    }

    Ok(SeqmatchallOutcome {
        input: params.input,
        word_size: params.word_size,
        hits,
    })
}

#[cfg(test)]
mod tests {
    use super::{SeqmatchallParams, run_seqmatchall};
    use crate::sequence_stream::SequenceInput;
    use std::fs;

    fn write_temp_sequence_file(name: &str, contents: &str) -> std::path::PathBuf {
        let path = std::env::temp_dir().join(format!(
            "emboss-rs-seqmatchall-{name}-{}-{}.fasta",
            std::process::id(),
            std::thread::current().name().unwrap_or("main")
        ));
        fs::write(&path, contents).expect("temporary fixture should be written");
        path
    }

    #[test]
    fn reports_all_against_all_hits_once_per_pair() {
        let input = write_temp_sequence_file("set", ">a\nTTACGTAA\n>b\nGGACGTCC\n>c\nCCACGTTT\n");
        let outcome = run_seqmatchall(SeqmatchallParams {
            input: SequenceInput::new(input.clone()),
            word_size: 4,
        })
        .expect("seqmatchall should succeed");
        fs::remove_file(input).ok();

        assert_eq!(outcome.hits.len(), 3);
        assert_eq!(outcome.hits[0].left_id, "a");
        assert_eq!(outcome.hits[0].right_id, "b");
        assert_eq!(outcome.hits[1].left_id, "a");
        assert_eq!(outcome.hits[1].right_id, "c");
        assert_eq!(outcome.hits[2].left_id, "b");
        assert_eq!(outcome.hits[2].right_id, "c");
    }
}
