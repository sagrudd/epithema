//! `aligncopypair` implementation.

use epithema_core::Alignment;
use epithema_diagnostics::{ErrorCategory, PlatformError};

use super::shared::{AlignmentInput, AlignmentToolError, load_alignment};

/// Typed parameters for `aligncopypair`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AligncopypairParams {
    /// Local alignment input path.
    pub input: AlignmentInput,
}

/// Structured `aligncopypair` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AligncopypairOutcome {
    /// Source input.
    pub input: AlignmentInput,
    /// Copied pairwise alignment payload.
    pub alignment: Alignment,
}

/// Returns `aligncopypair` help text.
#[must_use]
pub fn aligncopypair_help() -> &'static str {
    "Usage: epithema aligncopypair <input>\n\nCopy a single pairwise aligned FASTA or Stockholm alignment unchanged. Inputs with anything other than exactly two alignment rows are rejected."
}

/// Executes `aligncopypair`.
pub fn run_aligncopypair(
    params: AligncopypairParams,
) -> Result<AligncopypairOutcome, AlignmentToolError> {
    let alignment = load_alignment(&params.input)?;
    if !alignment.is_pairwise() {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            "aligncopypair requires exactly two alignment rows",
        )
        .with_code("tools.aligncopypair.input.not_pairwise"));
    }

    Ok(AligncopypairOutcome {
        input: params.input,
        alignment,
    })
}
