//! `cons` implementation.

use emboss_core::{ConsensusStrategy, SequenceRecord};

use super::shared::{derive_consensus, load_consensus_input};
use crate::alignment_tools::AlignmentInput;
use crate::sequence_stream::ToolExecutionError;

/// Typed parameters for `cons`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConsParams {
    /// Local alignment input.
    pub input: AlignmentInput,
}

/// Structured `cons` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConsOutcome {
    /// Source alignment input.
    pub input: AlignmentInput,
    /// Derived simple consensus record.
    pub consensus: SequenceRecord,
}

/// Returns `cons` help text.
#[must_use]
pub fn cons_help() -> &'static str {
    "Usage: emboss-rs cons <input>\n\nDerive a simple consensus sequence from one aligned FASTA or Stockholm alignment. v1 ignores gaps when counting a column, uses the majority non-gap residue when unique, and falls back to N for nucleotide columns or X for protein columns when there is no unique winner."
}

/// Executes `cons`.
pub fn run_cons(params: ConsParams) -> Result<ConsOutcome, ToolExecutionError> {
    let alignment = load_consensus_input(&params.input)?;
    let consensus = derive_consensus(
        &alignment,
        ConsensusStrategy::Simple,
        "consensus",
        &params.input.path,
    )?;
    Ok(ConsOutcome {
        input: params.input,
        consensus,
    })
}
