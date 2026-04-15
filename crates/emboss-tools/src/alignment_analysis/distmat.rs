//! `distmat` implementation.

use emboss_core::{AlignmentMode, DistanceMatrix};

use super::shared::build_distance_matrix;
use crate::sequence_stream::{SequenceInput, ToolExecutionError, load_sequence_records};

/// Typed parameters for `distmat`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DistmatParams {
    /// Local sequence-set input path.
    pub input: SequenceInput,
}

/// Structured `distmat` outcome.
#[derive(Clone, Debug, PartialEq)]
pub struct DistmatOutcome {
    /// Source input.
    pub input: SequenceInput,
    /// Computed distance matrix.
    pub matrix: DistanceMatrix,
}

/// Returns `distmat` help text.
#[must_use]
pub fn distmat_help() -> &'static str {
    "Usage: emboss-rs distmat <input>\n\nCompute a deterministic p-distance matrix for a local sequence set. v1 requires all input records to have equal length and reports mismatch_fraction = mismatches / sequence_length for every pair in stable input order."
}

/// Executes `distmat`.
pub fn run_distmat(params: DistmatParams) -> Result<DistmatOutcome, ToolExecutionError> {
    let records = load_sequence_records(&params.input)?;
    let matrix = build_distance_matrix(&records)?;
    Ok(DistmatOutcome {
        input: params.input,
        matrix,
    })
}

impl DistmatOutcome {
    /// Returns the matrix mode label.
    #[must_use]
    pub fn mode_label(&self) -> &'static str {
        match self.matrix.mode {
            AlignmentMode::Nucleotide => "nucleotide",
            AlignmentMode::Protein => "protein",
        }
    }
}
