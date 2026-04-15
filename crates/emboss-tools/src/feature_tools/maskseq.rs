//! `maskseq` implementation.

use emboss_core::{Interval, SequenceRecord, mask_intervals};
use emboss_diagnostics::{ErrorCategory, PlatformError};

use crate::feature_tools::shared::{effective_mask_symbol, map_feature_error};
use crate::sequence_stream::{SequenceInput, ToolExecutionError, load_sequence_records};

/// Typed parameters for `maskseq`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MaskseqParams {
    /// Local sequence input path.
    pub input: SequenceInput,
    /// 1-based inclusive masking intervals converted to zero-based half-open core intervals.
    pub intervals: Vec<Interval>,
    /// Optional explicit mask symbol.
    pub mask_char: Option<char>,
}

/// Structured `maskseq` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MaskseqOutcome {
    /// Source input.
    pub input: SequenceInput,
    /// Applied intervals in core coordinates.
    pub intervals: Vec<Interval>,
    /// Optional explicit mask symbol.
    pub mask_char: Option<char>,
    /// Masked records in input order.
    pub records: Vec<SequenceRecord>,
}

/// Returns `maskseq` help text.
#[must_use]
pub fn maskseq_help() -> &'static str {
    "Usage: emboss-rs maskseq <input> <start:end> [start:end ...] [--mask-char <char>]\n\nMask one or more explicit 1-based inclusive intervals in each input sequence record. Overlapping intervals are allowed and are applied deterministically in place. The default mask symbol is N for nucleotide records and X for protein records."
}

/// Executes `maskseq`.
pub fn run_maskseq(params: MaskseqParams) -> Result<MaskseqOutcome, ToolExecutionError> {
    if params.intervals.is_empty() {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            "maskseq requires at least one interval",
        )
        .with_code("tools.maskseq.interval.missing"));
    }

    let records = load_sequence_records(&params.input)?
        .into_iter()
        .map(|record| {
            let mask_symbol = effective_mask_symbol(&record, params.mask_char);
            mask_intervals(&record, &params.intervals, mask_symbol)
                .map_err(|error| map_feature_error("maskseq", error))
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(MaskseqOutcome {
        input: params.input,
        intervals: params.intervals,
        mask_char: params.mask_char,
        records,
    })
}
