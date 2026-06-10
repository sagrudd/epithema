//! `needle` implementation.

use epithema_core::{AlignmentMode, PairwiseAlignmentResult};

use super::shared::{align_pair, load_singleton_record};
use crate::sequence_stream::{SequenceInput, ToolExecutionError};

/// Typed parameters for `needle`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NeedleParams {
    /// Singleton query sequence input.
    pub query: SequenceInput,
    /// Singleton target sequence input.
    pub target: SequenceInput,
    /// Optional gap-open penalty override.
    pub gap_open: Option<i32>,
    /// Optional gap-extend penalty override.
    pub gap_extend: Option<i32>,
}

/// Structured `needle` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NeedleOutcome {
    /// Source query input.
    pub query: SequenceInput,
    /// Source target input.
    pub target: SequenceInput,
    /// Gap-open penalty used.
    pub gap_open: i32,
    /// Gap-extend penalty used.
    pub gap_extend: i32,
    /// Completed pairwise alignment result.
    pub result: PairwiseAlignmentResult,
}

/// Returns `needle` help text.
#[must_use]
pub fn needle_help() -> &'static str {
    "Usage: epithema needle <query-input> <target-input> [--gap-open <penalty>] [--gap-extend <penalty>]\n\nPerform deterministic global pairwise alignment between exactly one query record and one target record. Nucleotide mode uses match=1 mismatch=-1 gap_open=5 gap_extend=1 by default. Protein mode uses match=2 mismatch=-1 gap_open=8 gap_extend=1 by default. The CLI renders the resulting pairwise alignment as Stockholm."
}

/// Executes `needle`.
pub fn run_needle(params: NeedleParams) -> Result<NeedleOutcome, ToolExecutionError> {
    let query = load_singleton_record(&params.query, "needle", "query")?;
    let target = load_singleton_record(&params.target, "needle", "target")?;
    let result = align_pair(&query, &target, params.gap_open, params.gap_extend)?;
    let mode = result.summary.mode;
    let defaults = default_penalties(mode);

    Ok(NeedleOutcome {
        query: params.query,
        target: params.target,
        gap_open: params.gap_open.unwrap_or(defaults.0),
        gap_extend: params.gap_extend.unwrap_or(defaults.1),
        result,
    })
}

fn default_penalties(mode: AlignmentMode) -> (i32, i32) {
    match mode {
        AlignmentMode::Nucleotide => (5, 1),
        AlignmentMode::Protein => (8, 1),
    }
}
