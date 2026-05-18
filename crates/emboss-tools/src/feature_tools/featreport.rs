//! `featreport` implementation.

use emboss_core::{FeatureSelector, summarize_features};
use emboss_diagnostics::{ErrorCategory, PlatformError};

use crate::feature_tools::render::{render_feature_kind, render_feature_strand};
use crate::sequence_stream::{SequenceInput, ToolExecutionError, load_sequence_records};

/// Typed parameters for `featreport`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FeatreportParams {
    /// Local annotated sequence input path.
    pub input: SequenceInput,
    /// Shared feature selector.
    pub selector: FeatureSelector,
}

/// Structured `featreport` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FeatreportOutcome {
    /// Source input.
    pub input: SequenceInput,
    /// Shared feature selector.
    pub selector: FeatureSelector,
    /// Stable table rows in source/feature order.
    pub rows: Vec<Vec<String>>,
}

/// Returns `featreport` help text.
#[must_use]
pub fn featreport_help() -> &'static str {
    "Usage: emboss-rs featreport <input> [--kind <kind>] [--name <name>] [--qualifier <key[=value]>] [--strand <forward|reverse|unknown>]\n\nReport selected features from an annotated EMBL or GenBank input as a stable table. If no selector flags are supplied, all features are reported. Output rows preserve source-record order and feature order."
}

/// Executes `featreport`.
pub fn run_featreport(
    params: FeatreportParams,
) -> Result<FeatreportOutcome, ToolExecutionError> {
    let mut rows = Vec::new();

    for record in load_sequence_records(&params.input)? {
        for summary in summarize_features(&record, &params.selector) {
            rows.push(vec![
                record.identifier().accession().to_owned(),
                render_feature_kind(&summary.kind),
                render_feature_location_from_bounds_and_strand(
                    summary.bounds.start() + 1,
                    summary.bounds.end(),
                    summary.strand,
                ),
                (summary.bounds.start() + 1).to_string(),
                summary.bounds.end().to_string(),
                render_feature_strand(summary.strand).to_owned(),
                summary.name.unwrap_or_else(|| "-".to_owned()),
                summary.qualifier_count.to_string(),
            ]);
        }
    }

    if rows.is_empty() {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            "featreport did not find any features matching the requested selector",
        )
        .with_code("tools.featreport.feature.no_match"));
    }

    Ok(FeatreportOutcome {
        input: params.input,
        selector: params.selector,
        rows,
    })
}

fn render_feature_location_from_bounds_and_strand(
    start: usize,
    end: usize,
    strand: Option<emboss_core::Strand>,
) -> String {
    match strand {
        Some(emboss_core::Strand::Reverse) => format!("complement({start}..{end})"),
        _ => format!("{start}..{end}"),
    }
}

#[cfg(test)]
mod tests {
    use super::{FeatreportParams, run_featreport};
    use crate::sequence_stream::SequenceInput;

    #[test]
    fn reports_selected_features_in_stable_order() {
        let fixture = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/annotated_feature.gbk");
        let outcome = run_featreport(FeatreportParams {
            input: SequenceInput::new(&fixture),
            selector: emboss_core::FeatureSelector::Any,
        })
        .expect("featreport should execute");

        assert_eq!(outcome.rows.len(), 2);
        assert_eq!(outcome.rows[0][0], "FEAT1");
        assert_eq!(outcome.rows[0][1], "gene");
        assert_eq!(outcome.rows[1][1], "CDS");
    }
}
