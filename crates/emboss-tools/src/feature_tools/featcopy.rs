//! `featcopy` implementation.

use std::collections::{BTreeMap, BTreeSet};

use emboss_core::{FeatureSelector, SequenceRecord, copy_selected_features};
use emboss_diagnostics::{ErrorCategory, PlatformError};

use crate::sequence_stream::{SequenceInput, ToolExecutionError, load_sequence_records};

/// Typed parameters for `featcopy`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FeatcopyParams {
    /// Local annotated source input path.
    pub source: SequenceInput,
    /// Local target input path.
    pub target: SequenceInput,
    /// Shared feature selector.
    pub selector: FeatureSelector,
}

/// Structured `featcopy` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FeatcopyOutcome {
    /// Local annotated source input path.
    pub source: SequenceInput,
    /// Local target input path.
    pub target: SequenceInput,
    /// Shared feature selector.
    pub selector: FeatureSelector,
    /// Number of copied features.
    pub copied_feature_count: usize,
    /// Updated target records in input order.
    pub records: Vec<SequenceRecord>,
}

/// Returns `featcopy` help text.
#[must_use]
pub fn featcopy_help() -> &'static str {
    "Usage: emboss-rs featcopy <source> <target> [--kind <kind>] [--name <name>] [--qualifier <key[=value]>] [--strand <forward|reverse|unknown>]\n\nCopy selected features from an annotated source input to matching target records. Records are paired by stable identifier, both inputs must contain the same identifier set, and paired record lengths must match. Existing target features are preserved and copied source features are appended in stable order."
}

/// Executes `featcopy`.
pub fn run_featcopy(params: FeatcopyParams) -> Result<FeatcopyOutcome, ToolExecutionError> {
    let source_records = load_sequence_records(&params.source)?;
    let target_records = load_sequence_records(&params.target)?;
    let source_by_id = index_records("source", &source_records)?;
    let target_ids = target_records
        .iter()
        .map(|record| record.identifier().accession().to_owned())
        .collect::<BTreeSet<_>>();
    let source_ids = source_by_id.keys().cloned().collect::<BTreeSet<_>>();

    if source_ids != target_ids {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            "featcopy requires source and target inputs to contain the same record identifiers",
        )
        .with_code("tools.featcopy.records.identifier_mismatch"));
    }

    let mut copied_feature_count = 0usize;
    let mut records = Vec::new();

    for target in target_records {
        let accession = target.identifier().accession();
        let source = source_by_id
            .get(accession)
            .expect("identifier sets were checked");
        if source.len() != target.len() {
            return Err(PlatformError::new(
                ErrorCategory::Validation,
                format!("featcopy requires matching record lengths for identifier '{accession}'"),
            )
            .with_code("tools.featcopy.records.length_mismatch"));
        }

        let selected = copy_selected_features(source, &params.selector);
        copied_feature_count += selected.len();

        let mut updated = target.clone();
        for feature in selected {
            updated.add_feature(feature).map_err(|error| {
                PlatformError::new(ErrorCategory::Validation, error.to_string())
                    .with_code("tools.featcopy.feature.invalid")
            })?;
        }
        records.push(updated);
    }

    if copied_feature_count == 0 {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            "featcopy did not find any features matching the requested selector",
        )
        .with_code("tools.featcopy.feature.no_match"));
    }

    Ok(FeatcopyOutcome {
        source: params.source,
        target: params.target,
        selector: params.selector,
        copied_feature_count,
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
            .with_code("tools.featcopy.records.duplicate_identifier"));
        }
    }
    Ok(indexed)
}
