//! `extractfeat` implementation.

use emboss_core::{ExtractedFeatureRecord, FeatureSelector, extract_selected_regions};
use emboss_diagnostics::{ErrorCategory, PlatformError};

use crate::feature_tools::shared::map_feature_error;
use crate::sequence_stream::{SequenceInput, ToolExecutionError, load_sequence_records};

/// Typed parameters for `extractfeat`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExtractfeatParams {
    /// Local annotated sequence input path.
    pub input: SequenceInput,
    /// Shared feature selector.
    pub selector: FeatureSelector,
}

/// Structured `extractfeat` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExtractfeatOutcome {
    /// Source input.
    pub input: SequenceInput,
    /// Shared feature selector.
    pub selector: FeatureSelector,
    /// Total extracted feature count.
    pub extracted_feature_count: usize,
    /// Extracted records in stable source/feature order.
    pub records: Vec<ExtractedFeatureRecord>,
}

/// Returns `extractfeat` help text.
#[must_use]
pub fn extractfeat_help() -> &'static str {
    "Usage: emboss-rs extractfeat <input> [--kind <kind>] [--name <name>] [--qualifier <key[=value]>] [--strand <forward|reverse|unknown>]\n\nExtract selected simple feature spans from an annotated EMBL or GenBank input. If no selector flags are supplied, all features are selected. One output record is emitted per selected simple feature, with the copied feature rebased onto the extracted local coordinate system."
}

/// Executes `extractfeat`.
pub fn run_extractfeat(
    params: ExtractfeatParams,
) -> Result<ExtractfeatOutcome, ToolExecutionError> {
    let mut records = Vec::new();

    for record in load_sequence_records(&params.input)? {
        match extract_selected_regions(&record, &params.selector) {
            Ok(mut extracted) => records.append(&mut extracted),
            Err(emboss_core::FeatureOperationError::NoMatchingFeatures) => {}
            Err(error) => return Err(map_feature_error("extractfeat", error)),
        }
    }

    if records.is_empty() {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            "extractfeat did not find any features matching the requested selector",
        )
        .with_code("tools.extractfeat.feature.no_match"));
    }

    Ok(ExtractfeatOutcome {
        input: params.input,
        selector: params.selector,
        extracted_feature_count: records.len(),
        records,
    })
}
