//! `maskfeat` implementation.

use emboss_core::{FeatureSelector, SequenceRecord, mask_selected_features, select_features};
use emboss_diagnostics::{ErrorCategory, PlatformError};

use crate::feature_tools::shared::{effective_mask_symbol, map_feature_error};
use crate::sequence_stream::{SequenceInput, ToolExecutionError, load_sequence_records};

/// Typed parameters for `maskfeat`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MaskfeatParams {
    /// Local annotated sequence input path.
    pub input: SequenceInput,
    /// Shared feature selector.
    pub selector: FeatureSelector,
    /// Optional explicit mask symbol.
    pub mask_char: Option<char>,
}

/// Structured `maskfeat` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MaskfeatOutcome {
    /// Source input.
    pub input: SequenceInput,
    /// Shared feature selector.
    pub selector: FeatureSelector,
    /// Optional explicit mask symbol.
    pub mask_char: Option<char>,
    /// Total selected feature count across all records.
    pub selected_feature_count: usize,
    /// Masked records in input order.
    pub records: Vec<SequenceRecord>,
}

/// Returns `maskfeat` help text.
#[must_use]
pub fn maskfeat_help() -> &'static str {
    "Usage: emboss-rs maskfeat <input> [--kind <kind>] [--name <name>] [--qualifier <key[=value]>] [--strand <forward|reverse|unknown>] [--mask-char <char>]\n\nMask selected simple feature spans in an annotated EMBL or GenBank input. If no selector flags are supplied, all features are selected. Overlapping selected spans are masked in place, features are preserved in the result payload, and the default mask symbol is N for nucleotide records and X for protein records."
}

/// Executes `maskfeat`.
pub fn run_maskfeat(params: MaskfeatParams) -> Result<MaskfeatOutcome, ToolExecutionError> {
    let mut selected_feature_count = 0usize;
    let mut records = Vec::new();

    for record in load_sequence_records(&params.input)? {
        let matched = select_features(&record, &params.selector).len();
        selected_feature_count += matched;
        if matched == 0 {
            records.push(record);
            continue;
        }

        let mask_symbol = effective_mask_symbol(&record, params.mask_char);
        records.push(
            mask_selected_features(&record, &params.selector, mask_symbol)
                .map_err(|error| map_feature_error("maskfeat", error))?,
        );
    }

    if selected_feature_count == 0 {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            "maskfeat did not find any features matching the requested selector",
        )
        .with_code("tools.maskfeat.feature.no_match"));
    }

    Ok(MaskfeatOutcome {
        input: params.input,
        selector: params.selector,
        mask_char: params.mask_char,
        selected_feature_count,
        records,
    })
}
