//! `einverted` implementation.

use super::inverted_repeats::exact_inverted_repeats;
use crate::sequence_stream::{SequenceInput, ToolExecutionError, load_sequence_records};

/// Typed parameters for `einverted`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EinvertedParams {
    /// Local nucleotide input path.
    pub input: SequenceInput,
    /// Minimum inverted-repeat arm length.
    pub min_arm_length: usize,
    /// Maximum spacer length between arms.
    pub max_gap_length: usize,
}

/// One exact inverted-repeat hit.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EinvertedHit {
    /// Source record identifier.
    pub record_id: String,
    /// Zero-based inclusive left-arm start.
    pub left_start: usize,
    /// Zero-based half-open left-arm end.
    pub left_end: usize,
    /// Zero-based inclusive right-arm start.
    pub right_start: usize,
    /// Zero-based half-open right-arm end.
    pub right_end: usize,
    /// Gap length between arms.
    pub gap_length: usize,
    /// Left-arm sequence.
    pub left_arm: String,
    /// Right-arm sequence.
    pub right_arm: String,
}

/// Structured `einverted` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EinvertedOutcome {
    /// Source input.
    pub input: SequenceInput,
    /// Minimum arm length.
    pub min_arm_length: usize,
    /// Maximum gap length.
    pub max_gap_length: usize,
    /// Stable ordered hits.
    pub hits: Vec<EinvertedHit>,
}

/// Returns `einverted` help text.
#[must_use]
pub fn einverted_help() -> &'static str {
    "Usage: epithema einverted <nucleotide-input> [--min-arm-length <length>] [--max-gap-length <length>]\n\nReport exact inverted-repeat arms in nucleotide sequence records. v1 searches for exact reverse-complement arm pairs, allows a bounded spacer, reports overlapping hits, and uses 1-based inclusive coordinates in rendered output."
}

/// Executes `einverted`.
pub fn run_einverted(params: EinvertedParams) -> Result<EinvertedOutcome, ToolExecutionError> {
    let mut hits = Vec::new();

    for record in load_sequence_records(&params.input)? {
        hits.extend(
            exact_inverted_repeats(
                "einverted",
                record.molecule(),
                record.residues(),
                params.min_arm_length,
                params.max_gap_length,
            )?
            .into_iter()
            .map(|region| EinvertedHit {
                record_id: record.identifier().accession().to_owned(),
                left_start: region.left_start,
                left_end: region.left_end,
                right_start: region.right_start,
                right_end: region.right_end,
                gap_length: region.gap_length,
                left_arm: region.left_arm,
                right_arm: region.right_arm,
            }),
        );
    }

    Ok(EinvertedOutcome {
        input: params.input,
        min_arm_length: params.min_arm_length,
        max_gap_length: params.max_gap_length,
        hits,
    })
}

#[cfg(test)]
mod tests {
    use super::{EinvertedParams, run_einverted};
    use crate::sequence_stream::SequenceInput;
    use std::fs;

    fn write_temp_sequence_file(name: &str, contents: &str) -> std::path::PathBuf {
        let path = std::env::temp_dir().join(format!(
            "epithema-einverted-{name}-{}-{}.fasta",
            std::process::id(),
            std::thread::current().name().unwrap_or("main")
        ));
        fs::write(&path, contents).expect("temporary sequence fixture should be written");
        path
    }

    #[test]
    fn reports_exact_inverted_repeat_arms() {
        let path = write_temp_sequence_file("inv", ">inv\nATGCNNGCAT\n");
        let outcome = run_einverted(EinvertedParams {
            input: SequenceInput::new(path.clone()),
            min_arm_length: 4,
            max_gap_length: 2,
        })
        .expect("einverted should succeed");
        fs::remove_file(path).ok();

        assert!(!outcome.hits.is_empty());
        assert!(outcome.hits.iter().any(|hit| {
            hit.left_start == 0
                && hit.right_start == 6
                && hit.gap_length == 2
                && hit.left_arm == "ATGC"
                && hit.right_arm == "GCAT"
        }));
    }
}
