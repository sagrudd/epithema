//! `splitsource` implementation.

use epithema_core::{FeatureKind, Interval, SequenceIdentifier, SequenceMetadata, SequenceRecord};
use epithema_diagnostics::{ErrorCategory, PlatformError};

use crate::sequence_stream::{SequenceInput, ToolExecutionError, load_sequence_records};

/// Typed parameters for `splitsource`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SplitsourceParams {
    /// Local annotated sequence input path.
    pub input: SequenceInput,
}

/// Structured `splitsource` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SplitsourceOutcome {
    /// Source input.
    pub input: SequenceInput,
    /// Total source-feature fragments emitted.
    pub fragment_count: usize,
    /// Split fragments in source-record and source-feature order.
    pub records: Vec<SequenceRecord>,
}

/// Returns `splitsource` help text.
#[must_use]
pub fn splitsource_help() -> &'static str {
    "Usage: epithema splitsource <input>\n\nSplit annotated EMBL or GenBank sequence records into source-feature fragments. Epithema v1 requires each processed record to contain two or more simple `source` features, emits one subsequence per source feature in source order, and returns unannotated fragment records with inherited metadata plus a derived description."
}

/// Executes `splitsource`.
pub fn run_splitsource(
    params: SplitsourceParams,
) -> Result<SplitsourceOutcome, ToolExecutionError> {
    let mut records = Vec::new();

    for record in load_sequence_records(&params.input)? {
        let source_intervals = source_intervals(&record)?;
        if source_intervals.len() < 2 {
            return Err(PlatformError::new(
                ErrorCategory::Validation,
                format!(
                    "splitsource requires at least two simple source features per record; '{}' has {}",
                    record.identifier().accession(),
                    source_intervals.len()
                ),
            )
            .with_code("tools.splitsource.source_features.too_few"));
        }

        for (index, interval) in source_intervals.into_iter().enumerate() {
            let subsequence = record.subsequence(interval).map_err(|error| {
                PlatformError::new(ErrorCategory::Validation, error.to_string())
                    .with_code("tools.splitsource.interval.out_of_bounds")
            })?;
            let identifier = SequenceIdentifier::new(format!(
                "{}-source-{}",
                record.identifier().accession(),
                index + 1
            ))
            .map_err(|error| {
                PlatformError::new(ErrorCategory::Validation, error.to_string())
                    .with_code("tools.splitsource.identifier.invalid")
            })?;
            let metadata = source_fragment_metadata(record.metadata(), index + 1, interval);
            let derived = SequenceRecord::new(identifier, record.molecule(), subsequence)
                .map_err(|error| {
                    PlatformError::new(ErrorCategory::Validation, error.to_string())
                        .with_code("tools.splitsource.record.invalid")
                })?
                .with_metadata(metadata);
            records.push(derived);
        }
    }

    if records.is_empty() {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            "splitsource did not emit any source fragments",
        )
        .with_code("tools.splitsource.fragments.none"));
    }

    Ok(SplitsourceOutcome {
        input: params.input,
        fragment_count: records.len(),
        records,
    })
}

fn source_intervals(record: &SequenceRecord) -> Result<Vec<Interval>, ToolExecutionError> {
    let mut intervals = Vec::new();
    for feature in record.features() {
        if !matches_source_feature(&feature.kind) {
            continue;
        }

        let span = feature.location.single_span().ok_or_else(|| {
            PlatformError::new(
                ErrorCategory::Validation,
                "splitsource currently supports only simple single-span source features",
            )
            .with_code("tools.splitsource.source_features.unsupported_complex_location")
        })?;
        intervals.push(span.interval());
    }

    Ok(intervals)
}

fn matches_source_feature(kind: &FeatureKind) -> bool {
    matches!(kind, FeatureKind::Other(label) if label.eq_ignore_ascii_case("source"))
}

fn source_fragment_metadata(
    metadata: &SequenceMetadata,
    ordinal: usize,
    interval: Interval,
) -> SequenceMetadata {
    let description = format!(
        "source fragment {} {}..{}",
        ordinal,
        interval.start() + 1,
        interval.end()
    );
    metadata.clone().with_description(description)
}

#[cfg(test)]
mod tests {
    use super::{SplitsourceParams, run_splitsource};
    use crate::sequence_stream::SequenceInput;

    #[test]
    fn emits_one_record_per_source_feature() {
        let outcome = run_splitsource(SplitsourceParams {
            input: SequenceInput::new(
                std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join("../../crates/epithema-tools/tests/fixtures/splitsource_annotated.gbk"),
            ),
        })
        .expect("splitsource should execute");

        assert_eq!(outcome.fragment_count, 2);
        assert_eq!(outcome.records[0].residues(), "AAAT");
        assert_eq!(outcome.records[1].residues(), "GGGC");
    }

    #[test]
    fn rejects_records_without_multiple_source_features() {
        let error = run_splitsource(SplitsourceParams {
            input: SequenceInput::new(
                std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join("../../crates/epithema-tools/tests/fixtures/annotated_feature.gbk"),
            ),
        })
        .expect_err("splitsource should reject non-synthetic record");

        assert_eq!(
            error.code(),
            Some("tools.splitsource.source_features.too_few")
        );
    }
}
