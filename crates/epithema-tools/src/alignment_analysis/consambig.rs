//! `consambig` implementation.

use epithema_core::{ConsensusStrategy, SequenceRecord};

use super::shared::{derive_consensus, load_consensus_input};
use crate::alignment_tools::AlignmentInput;
use crate::sequence_stream::ToolExecutionError;

/// Typed parameters for `consambig`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConsambigParams {
    /// Local alignment input.
    pub input: AlignmentInput,
}

/// Structured `consambig` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConsambigOutcome {
    /// Source alignment input.
    pub input: AlignmentInput,
    /// Derived ambiguity-aware consensus record.
    pub consensus: SequenceRecord,
}

/// Returns `consambig` help text.
#[must_use]
pub fn consambig_help() -> &'static str {
    "Usage: epithema consambig <input>\n\nDerive an ambiguity-aware consensus sequence from one aligned FASTA or Stockholm alignment. v1 ignores gaps when counting a column, emits IUPAC nucleotide ambiguity symbols when a nucleotide column contains multiple supported exact bases, and emits X for ambiguous protein columns."
}

/// Executes `consambig`.
pub fn run_consambig(params: ConsambigParams) -> Result<ConsambigOutcome, ToolExecutionError> {
    let alignment = load_consensus_input(&params.input)?;
    let consensus = derive_consensus(
        &alignment,
        ConsensusStrategy::Ambiguous,
        "consambig",
        &params.input.path,
    )?;
    Ok(ConsambigOutcome {
        input: params.input,
        consensus,
    })
}
