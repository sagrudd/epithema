//! `matcher` implementation.

use epithema_core::DirectMatchSummary;

use super::shared::{compare_sequences, load_singleton_sequence};
use crate::sequence_stream::{SequenceInput, ToolExecutionError};

/// Typed parameters for `matcher`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MatcherParams {
    /// Singleton query input.
    pub query: SequenceInput,
    /// Singleton target input.
    pub target: SequenceInput,
}

/// Structured `matcher` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MatcherOutcome {
    /// Source query input.
    pub query: SequenceInput,
    /// Source target input.
    pub target: SequenceInput,
    /// Direct match summary.
    pub summary: DirectMatchSummary,
}

/// Returns `matcher` help text.
#[must_use]
pub fn matcher_help() -> &'static str {
    "Usage: epithema matcher <query-input> <target-input>\n\nCompare exactly one query record and one target record without gapped alignment. v1 performs ungapped positional comparison over the shared overlap, reports compared length, identities, mismatches, identity percent over the compared overlap, and notes any input length difference."
}

/// Executes `matcher`.
pub fn run_matcher(params: MatcherParams) -> Result<MatcherOutcome, ToolExecutionError> {
    let query = load_singleton_sequence(&params.query, "matcher", "query")?;
    let target = load_singleton_sequence(&params.target, "matcher", "target")?;
    let summary = compare_sequences(&query, &target)?;

    Ok(MatcherOutcome {
        query: params.query,
        target: params.target,
        summary,
    })
}
