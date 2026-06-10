//! `twofeat` implementation.

use epithema_core::{FeatureSelector, SequenceRecord};
use epithema_diagnostics::{ErrorCategory, PlatformError};

use crate::feature_tools::render::{
    render_feature_kind, render_feature_location, render_feature_strand,
};
use crate::sequence_stream::{SequenceInput, ToolExecutionError, load_sequence_records};

/// Typed parameters for `twofeat`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TwofeatParams {
    /// Local annotated sequence input path.
    pub input: SequenceInput,
    /// Selector for the first feature in each neighbouring pair.
    pub a_selector: FeatureSelector,
    /// Selector for the second feature in each neighbouring pair.
    pub b_selector: FeatureSelector,
    /// Optional minimum nearest-end distance between neighbouring features.
    pub min_range: Option<usize>,
    /// Optional maximum nearest-end distance between neighbouring features.
    pub max_range: Option<usize>,
}

/// Structured `twofeat` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TwofeatOutcome {
    /// Source input.
    pub input: SequenceInput,
    /// Selector for the first feature.
    pub a_selector: FeatureSelector,
    /// Selector for the second feature.
    pub b_selector: FeatureSelector,
    /// Optional minimum nearest-end distance.
    pub min_range: Option<usize>,
    /// Optional maximum nearest-end distance.
    pub max_range: Option<usize>,
    /// Stable report rows in source-record and neighbouring-pair order.
    pub rows: Vec<Vec<String>>,
}

/// Returns `twofeat` help text.
#[must_use]
pub fn twofeat_help() -> &'static str {
    "Usage: epithema twofeat <input> [--a-kind <kind>] [--a-name <name>] [--a-qualifier <key[=value]>] [--a-strand <forward|reverse|unknown>] [--b-kind <kind>] [--b-name <name>] [--b-qualifier <key[=value]>] [--b-strand <forward|reverse|unknown>] [--min-range <count>] [--max-range <count>]\n\nReport neighbouring feature pairs from an annotated EMBL or GenBank input. Epithema v1 defines neighbouring pairs conservatively as adjacent features in source order, supports existing selector semantics for feature kind/name/qualifier/strand, and measures optional distances from the nearest feature bounds."
}

/// Executes `twofeat`.
pub fn run_twofeat(params: TwofeatParams) -> Result<TwofeatOutcome, ToolExecutionError> {
    let mut rows = Vec::new();

    for record in load_sequence_records(&params.input)? {
        rows.extend(rows_for_record(
            &record,
            &params.a_selector,
            &params.b_selector,
            params.min_range,
            params.max_range,
        ));
    }

    if rows.is_empty() {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            "twofeat did not find any neighbouring feature pairs matching the requested selectors and distance constraints",
        )
        .with_code("tools.twofeat.feature_pair.no_match"));
    }

    Ok(TwofeatOutcome {
        input: params.input,
        a_selector: params.a_selector,
        b_selector: params.b_selector,
        min_range: params.min_range,
        max_range: params.max_range,
        rows,
    })
}

fn rows_for_record(
    record: &SequenceRecord,
    a_selector: &FeatureSelector,
    b_selector: &FeatureSelector,
    min_range: Option<usize>,
    max_range: Option<usize>,
) -> Vec<Vec<String>> {
    let mut rows = Vec::new();

    for window in record.features().windows(2) {
        let a_feature = &window[0];
        let b_feature = &window[1];
        if !a_selector.matches(a_feature) || !b_selector.matches(b_feature) {
            continue;
        }

        let a_bounds = a_feature.location.bounds();
        let b_bounds = b_feature.location.bounds();
        let gap = nearest_end_gap(a_bounds.end(), b_bounds.start());
        if min_range.is_some_and(|minimum| gap < minimum) {
            continue;
        }
        if max_range.is_some_and(|maximum| gap > maximum) {
            continue;
        }

        rows.push(vec![
            record.identifier().accession().to_owned(),
            render_feature_kind(&a_feature.kind),
            render_feature_location(&a_feature.location),
            (a_bounds.start() + 1).to_string(),
            a_bounds.end().to_string(),
            render_feature_strand(a_feature.location.strand()).to_owned(),
            a_feature.name.clone().unwrap_or_else(|| "-".to_owned()),
            render_feature_kind(&b_feature.kind),
            render_feature_location(&b_feature.location),
            (b_bounds.start() + 1).to_string(),
            b_bounds.end().to_string(),
            render_feature_strand(b_feature.location.strand()).to_owned(),
            b_feature.name.clone().unwrap_or_else(|| "-".to_owned()),
            gap.to_string(),
            describe_relation(a_bounds.end(), b_bounds.start()).to_owned(),
            describe_strand_relation(a_feature.location.strand(), b_feature.location.strand())
                .to_owned(),
        ]);
    }

    rows
}

fn nearest_end_gap(left_end: usize, right_start: usize) -> usize {
    right_start.saturating_sub(left_end)
}

fn describe_relation(left_end: usize, right_start: usize) -> &'static str {
    if right_start < left_end {
        "overlap"
    } else if right_start == left_end {
        "abutting"
    } else {
        "separated"
    }
}

fn describe_strand_relation(
    a_strand: Option<epithema_core::Strand>,
    b_strand: Option<epithema_core::Strand>,
) -> &'static str {
    match (a_strand, b_strand) {
        (Some(epithema_core::Strand::Forward), Some(epithema_core::Strand::Forward))
        | (Some(epithema_core::Strand::Reverse), Some(epithema_core::Strand::Reverse)) => "same",
        (Some(epithema_core::Strand::Forward), Some(epithema_core::Strand::Reverse))
        | (Some(epithema_core::Strand::Reverse), Some(epithema_core::Strand::Forward)) => {
            "opposite"
        }
        _ => "unknown",
    }
}

#[cfg(test)]
mod tests {
    use epithema_core::{FeatureKind, FeatureSelector};

    use super::{TwofeatParams, run_twofeat};
    use crate::sequence_stream::SequenceInput;

    #[test]
    fn reports_adjacent_gene_cds_pair() {
        let fixture = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/annotated_feature.gbk");
        let outcome = run_twofeat(TwofeatParams {
            input: SequenceInput::new(&fixture),
            a_selector: FeatureSelector::Kind(FeatureKind::Gene),
            b_selector: FeatureSelector::Kind(FeatureKind::CodingSequence),
            min_range: None,
            max_range: None,
        })
        .expect("twofeat should execute");

        assert_eq!(outcome.rows.len(), 1);
        assert_eq!(outcome.rows[0][0], "FEAT1");
        assert_eq!(outcome.rows[0][1], "gene");
        assert_eq!(outcome.rows[0][7], "CDS");
        assert_eq!(outcome.rows[0][13], "1");
        assert_eq!(outcome.rows[0][14], "separated");
    }

    #[test]
    fn respects_distance_constraints() {
        let fixture = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/annotated_feature.gbk");
        let error = run_twofeat(TwofeatParams {
            input: SequenceInput::new(&fixture),
            a_selector: FeatureSelector::Kind(FeatureKind::Gene),
            b_selector: FeatureSelector::Kind(FeatureKind::CodingSequence),
            min_range: Some(2),
            max_range: None,
        })
        .expect_err("distance constraint should filter the pair");

        assert_eq!(error.code(), Some("tools.twofeat.feature_pair.no_match"));
    }
}
