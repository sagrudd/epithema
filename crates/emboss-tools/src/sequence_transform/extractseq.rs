//! `extractseq` implementation.

use emboss_core::{Interval, SequenceRecord};
use emboss_diagnostics::{ErrorCategory, PlatformError};

use crate::sequence_stream::{SequenceInput, ToolExecutionError, load_sequence_records};

/// Typed parameters for `extractseq`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExtractseqParams {
    /// Local sequence input path.
    pub input: SequenceInput,
    /// 1-based inclusive start.
    pub start: usize,
    /// 1-based inclusive end.
    pub end: usize,
}

/// Structured `extractseq` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExtractseqOutcome {
    /// Source input.
    pub input: SequenceInput,
    /// 1-based inclusive start.
    pub start: usize,
    /// 1-based inclusive end.
    pub end: usize,
    /// Extracted subsequences in input order.
    pub records: Vec<SequenceRecord>,
}

/// Returns `extractseq` help text.
#[must_use]
pub fn extractseq_help() -> &'static str {
    "Usage: emboss-rs extractseq <input> <start> <end>\n\nExtract the same 1-based inclusive region from each sequence record in a local FASTA, FASTQ, EMBL, or GenBank file. Coordinates must be within bounds for every record."
}

/// Executes `extractseq`.
pub fn run_extractseq(params: ExtractseqParams) -> Result<ExtractseqOutcome, ToolExecutionError> {
    if params.start == 0 || params.end == 0 || params.start > params.end {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            "extractseq requires 1-based inclusive coordinates with start <= end",
        )
        .with_code("tools.extractseq.coordinates.invalid"));
    }

    let records = load_sequence_records(&params.input)?
        .into_iter()
        .map(|record| extract_record(record, params.start, params.end))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(ExtractseqOutcome {
        input: params.input,
        start: params.start,
        end: params.end,
        records,
    })
}

fn extract_record(
    record: SequenceRecord,
    start: usize,
    end: usize,
) -> Result<SequenceRecord, ToolExecutionError> {
    if end > record.len() {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!(
                "requested region {start}..{end} is out of range for sequence '{}' of length {}",
                record.identifier().accession(),
                record.len()
            ),
        )
        .with_code("tools.extractseq.coordinates.out_of_range"));
    }

    let interval = Interval::new(start - 1, end).map_err(|error| {
        PlatformError::new(ErrorCategory::Validation, error.to_string())
            .with_code("tools.extractseq.interval.invalid")
    })?;
    let subsequence = record.subsequence(interval).map_err(|error| {
        PlatformError::new(ErrorCategory::Validation, error.to_string())
            .with_code("tools.extractseq.interval.invalid")
    })?;

    let mut extracted =
        SequenceRecord::new(record.identifier().clone(), record.molecule(), subsequence).map_err(
            |error| {
                PlatformError::new(ErrorCategory::Validation, error.to_string())
                    .with_code("tools.extractseq.record.invalid")
            },
        )?;
    extracted = extracted.with_metadata(record.metadata().clone());
    Ok(extracted)
}
