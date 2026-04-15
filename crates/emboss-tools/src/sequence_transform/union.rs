//! `union` implementation.

use emboss_core::SequenceRecord;
use emboss_diagnostics::{ErrorCategory, PlatformError};

use crate::sequence_stream::{SequenceInput, ToolExecutionError, load_sequence_records};

/// Typed parameters for `union`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnionParams {
    /// Ordered local sequence inputs.
    pub inputs: Vec<SequenceInput>,
}

/// Structured `union` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnionOutcome {
    /// Ordered source inputs.
    pub inputs: Vec<SequenceInput>,
    /// Combined records in deterministic input order.
    pub records: Vec<SequenceRecord>,
}

/// Returns `union` help text.
#[must_use]
pub fn union_help() -> &'static str {
    "Usage: emboss-rs union <input-a> <input-b> [input-c ...]\n\nConcatenate two or more local sequence inputs into a single output record stream, preserving input order and duplicates."
}

/// Executes `union`.
pub fn run_union(params: UnionParams) -> Result<UnionOutcome, ToolExecutionError> {
    if params.inputs.len() < 2 {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            "union requires at least two sequence inputs",
        )
        .with_code("tools.union.inputs.too_few"));
    }

    let mut records = Vec::new();
    for input in &params.inputs {
        records.extend(load_sequence_records(input)?);
    }

    Ok(UnionOutcome {
        inputs: params.inputs,
        records,
    })
}
