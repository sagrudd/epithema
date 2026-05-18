//! `featmerge` implementation.

use std::collections::{BTreeMap, BTreeSet};

use emboss_core::{FeatureSelector, SequenceRecord, copy_selected_features};
use emboss_diagnostics::{ErrorCategory, PlatformError};

use crate::sequence_stream::{SequenceInput, ToolExecutionError, load_sequence_records};

/// Typed parameters for `featmerge`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FeatmergeParams {
    /// Left annotated input path.
    pub left: SequenceInput,
    /// Right annotated input path.
    pub right: SequenceInput,
    /// Shared selector applied to right-side features before merging.
    pub selector: FeatureSelector,
}

/// Structured `featmerge` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FeatmergeOutcome {
    /// Left input.
    pub left: SequenceInput,
    /// Right input.
    pub right: SequenceInput,
    /// Shared selector.
    pub selector: FeatureSelector,
    /// Number of merged right-side features appended or admitted after deduplication.
    pub merged_feature_count: usize,
    /// Updated left-derived records in input order.
    pub records: Vec<SequenceRecord>,
}

/// Returns `featmerge` help text.
#[must_use]
pub fn featmerge_help() -> &'static str {
    "Usage: emboss-rs featmerge <left> <right> [--kind <kind>] [--name <name>] [--qualifier <key[=value]>] [--strand <forward|reverse|unknown>]\n\nMerge selected right-hand features into identifier-matched left-hand annotated records. Records are paired by stable identifier, both inputs must contain the same identifier set, paired record lengths must match, and exact duplicate feature entries are skipped deterministically."
}

/// Executes `featmerge`.
pub fn run_featmerge(
    params: FeatmergeParams,
) -> Result<FeatmergeOutcome, ToolExecutionError> {
    let left_records = load_sequence_records(&params.left)?;
    let right_records = load_sequence_records(&params.right)?;
    let right_by_id = index_records("right", &right_records)?;
    let left_ids = left_records
        .iter()
        .map(|record| record.identifier().accession().to_owned())
        .collect::<BTreeSet<_>>();
    let right_ids = right_by_id.keys().cloned().collect::<BTreeSet<_>>();

    if left_ids != right_ids {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            "featmerge requires left and right inputs to contain the same record identifiers",
        )
        .with_code("tools.featmerge.records.identifier_mismatch"));
    }

    let mut merged_feature_count = 0usize;
    let mut records = Vec::new();

    for left in left_records {
        let accession = left.identifier().accession();
        let right = right_by_id
            .get(accession)
            .expect("identifier sets were checked");
        if left.len() != right.len() {
            return Err(PlatformError::new(
                ErrorCategory::Validation,
                format!("featmerge requires matching record lengths for identifier '{accession}'"),
            )
            .with_code("tools.featmerge.records.length_mismatch"));
        }

        let selected = copy_selected_features(right, &params.selector);
        let mut merged = left.clone();
        for feature in selected {
            if merged.features().iter().any(|existing| *existing == feature) {
                continue;
            }
            merged.add_feature(feature).map_err(|error| {
                PlatformError::new(ErrorCategory::Validation, error.to_string())
                    .with_code("tools.featmerge.feature.invalid")
            })?;
            merged_feature_count += 1;
        }
        records.push(merged);
    }

    if merged_feature_count == 0 {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            "featmerge did not admit any selected right-hand features after deterministic deduplication",
        )
        .with_code("tools.featmerge.feature.no_merge"));
    }

    Ok(FeatmergeOutcome {
        left: params.left,
        right: params.right,
        selector: params.selector,
        merged_feature_count,
        records,
    })
}

fn index_records<'a>(
    label: &str,
    records: &'a [SequenceRecord],
) -> Result<BTreeMap<String, &'a SequenceRecord>, ToolExecutionError> {
    let mut indexed = BTreeMap::new();
    for record in records {
        let accession = record.identifier().accession().to_owned();
        if indexed.insert(accession.clone(), record).is_some() {
            return Err(PlatformError::new(
                ErrorCategory::Validation,
                format!("duplicate {label} identifier '{accession}' is not supported"),
            )
            .with_code("tools.featmerge.records.duplicate_identifier"));
        }
    }
    Ok(indexed)
}

#[cfg(test)]
mod tests {
    use super::{FeatmergeParams, run_featmerge};
    use crate::sequence_stream::SequenceInput;

    #[test]
    fn merges_selected_right_features_onto_left_records() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
        let outcome = run_featmerge(FeatmergeParams {
            left: SequenceInput::new(root.join("annotated_feature.gbk")),
            right: SequenceInput::new(root.join("annotated_merge_right.gbk")),
            selector: emboss_core::FeatureSelector::Any,
        })
        .expect("featmerge should execute");

        assert_eq!(outcome.records.len(), 1);
        assert_eq!(outcome.merged_feature_count, 1);
        assert_eq!(outcome.records[0].features().len(), 3);
    }
}
