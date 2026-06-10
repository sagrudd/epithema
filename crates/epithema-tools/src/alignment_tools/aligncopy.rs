//! `aligncopy` implementation.

use epithema_core::Alignment;

use super::shared::{AlignmentInput, AlignmentToolError, load_alignment};

/// Typed parameters for `aligncopy`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AligncopyParams {
    /// Local alignment input path.
    pub input: AlignmentInput,
}

/// Structured `aligncopy` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AligncopyOutcome {
    /// Source input.
    pub input: AlignmentInput,
    /// Copied alignment payload.
    pub alignment: Alignment,
}

/// Returns `aligncopy` help text.
#[must_use]
pub fn aligncopy_help() -> &'static str {
    "Usage: epithema aligncopy <input>\n\nCopy a single aligned FASTA or Stockholm alignment unchanged. The CLI renders alignment payloads as Stockholm by default."
}

/// Executes `aligncopy`.
pub fn run_aligncopy(params: AligncopyParams) -> Result<AligncopyOutcome, AlignmentToolError> {
    let alignment = load_alignment(&params.input)?;
    Ok(AligncopyOutcome {
        input: params.input,
        alignment,
    })
}
