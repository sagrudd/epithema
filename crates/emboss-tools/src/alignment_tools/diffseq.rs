//! `diffseq` implementation.

use emboss_core::{
    AlignmentMode, GlobalAlignmentScoring, SequenceRecord, global_align, infer_alignment_mode,
};

use crate::sequence_stream::{SequenceInput, ToolExecutionError};
use crate::sequence_transform::load_exactly_one_record;

/// Typed parameters for `diffseq`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DiffseqParams {
    /// Left sequence input.
    pub asequence: SequenceInput,
    /// Right sequence input.
    pub bsequence: SequenceInput,
    /// Gap-open penalty.
    pub gap_open: i32,
    /// Gap-extend penalty.
    pub gap_extend: i32,
}

/// One contiguous difference block from a deterministic global alignment.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DiffseqBlock {
    /// Stable 1-based block ordinal.
    pub ordinal: usize,
    /// Difference classification.
    pub classification: String,
    /// Optional 1-based inclusive left-sequence start.
    pub a_start: Option<usize>,
    /// Optional 1-based inclusive left-sequence end.
    pub a_end: Option<usize>,
    /// Optional 1-based inclusive right-sequence start.
    pub b_start: Option<usize>,
    /// Optional 1-based inclusive right-sequence end.
    pub b_end: Option<usize>,
    /// Aligned left-sequence segment for the block.
    pub a_segment: String,
    /// Aligned right-sequence segment for the block.
    pub b_segment: String,
}

/// Structured `diffseq` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DiffseqOutcome {
    /// Left sequence input.
    pub asequence: SequenceInput,
    /// Right sequence input.
    pub bsequence: SequenceInput,
    /// Gap-open penalty.
    pub gap_open: i32,
    /// Gap-extend penalty.
    pub gap_extend: i32,
    /// Alignment mode used.
    pub mode: AlignmentMode,
    /// Total aligned length.
    pub aligned_length: usize,
    /// Difference blocks in left-to-right alignment order.
    pub blocks: Vec<DiffseqBlock>,
}

/// Returns `diffseq` help text.
#[must_use]
pub fn diffseq_help() -> &'static str {
    "Usage: emboss-rs diffseq <asequence> <bsequence> [--gap-open <penalty>] [--gap-extend <penalty>]\n\nCompare exactly one sequence from each input, compute a deterministic global alignment, and report contiguous mismatch or indel blocks with 1-based inclusive sequence coordinates."
}

/// Executes `diffseq`.
pub fn run_diffseq(params: DiffseqParams) -> Result<DiffseqOutcome, ToolExecutionError> {
    let left = load_exactly_one_record(&params.asequence, "diffseq", "asequence")?;
    let right = load_exactly_one_record(&params.bsequence, "diffseq", "bsequence")?;

    let mode = infer_alignment_mode(&left, &right).map_err(map_alignment_error)?;
    let scoring = match mode {
        AlignmentMode::Nucleotide => GlobalAlignmentScoring {
            gap_open: params.gap_open,
            gap_extend: params.gap_extend,
            ..GlobalAlignmentScoring::nucleotide_default()
        },
        AlignmentMode::Protein => GlobalAlignmentScoring {
            gap_open: params.gap_open,
            gap_extend: params.gap_extend,
            ..GlobalAlignmentScoring::protein_default()
        },
    };
    let result = global_align(&left, &right, scoring).map_err(map_alignment_error)?;
    let blocks = collect_difference_blocks(&left, &right, result.alignment.rows()[0].aligned(), result.alignment.rows()[1].aligned());

    Ok(DiffseqOutcome {
        asequence: params.asequence,
        bsequence: params.bsequence,
        gap_open: params.gap_open,
        gap_extend: params.gap_extend,
        mode,
        aligned_length: result.summary.aligned_length,
        blocks,
    })
}

fn collect_difference_blocks(
    _left: &SequenceRecord,
    _right: &SequenceRecord,
    left_aligned: &str,
    right_aligned: &str,
) -> Vec<DiffseqBlock> {
    let left_symbols: Vec<char> = left_aligned.chars().collect();
    let right_symbols: Vec<char> = right_aligned.chars().collect();
    let mut blocks = Vec::new();
    let mut left_position = 0usize;
    let mut right_position = 0usize;
    let mut index = 0usize;

    while index < left_symbols.len() {
        let left_symbol = left_symbols[index];
        let right_symbol = right_symbols[index];
        let differs = left_symbol != right_symbol;

        let left_before = left_position;
        let right_before = right_position;
        if left_symbol != '-' {
            left_position += 1;
        }
        if right_symbol != '-' {
            right_position += 1;
        }

        if !differs {
            index += 1;
            continue;
        }

        let block_left_start = left_before;
        let block_right_start = right_before;
        let mut left_seen = 0usize;
        let mut right_seen = 0usize;
        let mut left_gap_only = true;
        let mut right_gap_only = true;
        let mut left_segment = String::new();
        let mut right_segment = String::new();

        while index < left_symbols.len() && left_symbols[index] != right_symbols[index] {
            let block_left_symbol = left_symbols[index];
            let block_right_symbol = right_symbols[index];
            left_segment.push(block_left_symbol);
            right_segment.push(block_right_symbol);
            if block_left_symbol != '-' {
                left_seen += 1;
                left_gap_only = false;
            }
            if block_right_symbol != '-' {
                right_seen += 1;
                right_gap_only = false;
            }
            index += 1;
        }

        let classification = if left_gap_only {
            "gap_in_asequence"
        } else if right_gap_only {
            "gap_in_bsequence"
        } else if left_seen == right_seen {
            "substitution"
        } else {
            "mixed"
        };

        blocks.push(DiffseqBlock {
            ordinal: blocks.len() + 1,
            classification: classification.to_owned(),
            a_start: (!left_gap_only).then_some(block_left_start + 1),
            a_end: (!left_gap_only).then_some(block_left_start + left_seen),
            b_start: (!right_gap_only).then_some(block_right_start + 1),
            b_end: (!right_gap_only).then_some(block_right_start + right_seen),
            a_segment: left_segment,
            b_segment: right_segment,
        });
    }

    blocks
}

fn map_alignment_error(error: emboss_core::GlobalAlignmentError) -> ToolExecutionError {
    emboss_diagnostics::PlatformError::new(emboss_diagnostics::ErrorCategory::Validation, error.to_string())
        .with_code("tools.diffseq.alignment.invalid")
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::sequence_stream::SequenceInput;

    use super::{DiffseqParams, run_diffseq};

    fn fixture(name: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join(format!("../emboss-tools/tests/fixtures/{name}"))
    }

    #[test]
    fn reports_substitution_blocks() {
        let outcome = run_diffseq(DiffseqParams {
            asequence: SequenceInput::new(fixture("diffseq_left.fasta")),
            bsequence: SequenceInput::new(fixture("diffseq_right.fasta")),
            gap_open: 5,
            gap_extend: 1,
        })
        .expect("diffseq should execute");

        assert_eq!(outcome.blocks.len(), 1);
        assert_eq!(outcome.blocks[0].classification, "substitution");
        assert_eq!(outcome.blocks[0].a_start, Some(5));
        assert_eq!(outcome.blocks[0].b_start, Some(5));
        assert_eq!(outcome.blocks[0].a_segment, "A");
        assert_eq!(outcome.blocks[0].b_segment, "T");
    }
}
