//! `descseq` implementation.

use emboss_core::SequenceRecord;
use emboss_diagnostics::{ErrorCategory, PlatformError};

use crate::sequence_stream::{SequenceInput, ToolExecutionError, load_sequence_records};

/// Typed parameters for `descseq`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DescseqParams {
    /// Local sequence input path.
    pub input: SequenceInput,
    /// Replacement description text.
    pub description: Option<String>,
    /// Clear existing descriptions instead of replacing them.
    pub clear: bool,
}

/// Structured `descseq` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DescseqOutcome {
    /// Source input.
    pub input: SequenceInput,
    /// Updated records in input order.
    pub records: Vec<SequenceRecord>,
}

/// Returns `descseq` help text.
#[must_use]
pub fn descseq_help() -> &'static str {
    "Usage: emboss-rs descseq <input> --description <text>\n       emboss-rs descseq <input> --clear\n\nReplace or clear the description field for each input sequence record."
}

/// Executes `descseq`.
pub fn run_descseq(params: DescseqParams) -> Result<DescseqOutcome, ToolExecutionError> {
    if params.clear == params.description.is_some() {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            "descseq requires exactly one of --description or --clear",
        )
        .with_code("tools.descseq.arguments.invalid"));
    }

    let records = load_sequence_records(&params.input)?
        .into_iter()
        .map(|record| update_description(record, params.description.as_deref()))
        .collect();

    Ok(DescseqOutcome {
        input: params.input,
        records,
    })
}

fn update_description(record: SequenceRecord, description: Option<&str>) -> SequenceRecord {
    let mut metadata = record.metadata().clone();
    metadata.description = description.map(ToOwned::to_owned);
    record.with_metadata(metadata)
}
