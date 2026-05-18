//! `wordcount` implementation.

use std::collections::BTreeMap;

use emboss_core::MoleculeKind;
use emboss_diagnostics::{ErrorCategory, PlatformError};
use emboss_plot_contract::{
    AxisScaleHint, DataVector, GeometryHint, PlotAxis, PlotKind, PlotMetadata, PlotPayload,
    PlotProvenance, PlotSeries, PlotSpec, SeriesStyle,
};

use crate::sequence_stream::{SequenceInput, ToolExecutionError, load_sequence_records};

/// Typed parameters for `wordcount`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WordcountParams {
    /// Local sequence input path.
    pub input: SequenceInput,
    /// Word size in normalized residues.
    pub word_size: usize,
    /// Minimum count threshold for output rows.
    pub min_count: usize,
}

/// Per-record overlapping word counts.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WordcountRecord {
    /// Stable record identifier.
    pub record_id: String,
    /// Molecule classification.
    pub molecule: MoleculeKind,
    /// Counted overlapping windows excluding skipped windows.
    pub counted_windows: usize,
    /// Windows skipped because they contained gaps.
    pub skipped_gap_windows: usize,
    /// Stable lexicographic word counts.
    pub counts: BTreeMap<String, usize>,
}

/// Structured `wordcount` outcome.
#[derive(Clone, Debug, PartialEq)]
pub struct WordcountOutcome {
    /// Source input.
    pub input: SequenceInput,
    /// Requested word size.
    pub word_size: usize,
    /// Minimum count threshold for report rows.
    pub min_count: usize,
    /// Per-record counts.
    pub records: Vec<WordcountRecord>,
    /// Aggregate counts across all records.
    pub aggregate: WordcountRecord,
    /// Optional aggregate plot contract when there are reportable rows.
    pub plot: Option<PlotPayload>,
}

/// Returns `wordcount` help text.
#[must_use]
pub fn wordcount_help() -> &'static str {
    "Usage: emboss-rs wordcount <input> --word-size <count> [--min-count <count>] [--plot-contract-out <path>]\n\nCount overlapping normalized sequence words in one or more records. v1 reports both per-record and aggregate counts, treats words as exact normalized substrings, skips windows containing gap symbols '-', outputs rows in stable lexicographic word order, and emits an aggregate categorical bar-plot contract when the filtered aggregate is nonempty."
}

/// Executes `wordcount`.
pub fn run_wordcount(params: WordcountParams) -> Result<WordcountOutcome, ToolExecutionError> {
    let records = load_sequence_records(&params.input)?
        .into_iter()
        .map(|record| {
            let (counted_windows, skipped_gap_windows, counts) =
                count_windows(record.residues(), params.word_size);
            WordcountRecord {
                record_id: record.identifier().accession().to_owned(),
                molecule: record.molecule(),
                counted_windows,
                skipped_gap_windows,
                counts,
            }
        })
        .collect::<Vec<_>>();

    let mut aggregate = WordcountRecord {
        record_id: "ALL".to_owned(),
        molecule: MoleculeKind::Unknown,
        counted_windows: 0,
        skipped_gap_windows: 0,
        counts: BTreeMap::new(),
    };
    for record in &records {
        aggregate.counted_windows += record.counted_windows;
        aggregate.skipped_gap_windows += record.skipped_gap_windows;
        for (word, count) in &record.counts {
            *aggregate.counts.entry(word.clone()).or_insert(0) += count;
        }
    }

    let plot = build_wordcount_plot(params.word_size, params.min_count, &aggregate)?;

    Ok(WordcountOutcome {
        input: params.input,
        word_size: params.word_size,
        min_count: params.min_count,
        records,
        aggregate,
        plot,
    })
}

fn build_wordcount_plot(
    word_size: usize,
    min_count: usize,
    aggregate: &WordcountRecord,
) -> Result<Option<PlotPayload>, ToolExecutionError> {
    let words = aggregate
        .counts
        .iter()
        .filter(|(_, count)| **count >= min_count)
        .map(|(word, _)| word.clone())
        .collect::<Vec<_>>();
    if words.is_empty() {
        return Ok(None);
    }

    let counts = words
        .iter()
        .map(|word| aggregate.counts.get(word).copied().unwrap_or_default() as f64)
        .collect::<Vec<_>>();

    let plot = PlotSpec::new(
        PlotKind::Bar,
        PlotMetadata::new(
            format!("wordcount_aggregate_{word_size}"),
            "Aggregate word counts",
        )
        .with_subtitle(format!(
            "Word size {word_size}, minimum count {min_count}, counted windows {}",
            aggregate.counted_windows
        ))
        .with_provenance(PlotProvenance {
            tool: Some("wordcount".to_owned()),
            method: Some("run_wordcount".to_owned()),
            source_artifact_ids: vec!["table:wordcount-aggregate".to_owned()],
        }),
        PlotAxis::new("Word").with_scale_hint(AxisScaleHint::Categorical),
        PlotAxis::new("Count").with_scale_hint(AxisScaleHint::Linear),
        vec![
            PlotSeries::new(
                "aggregate_counts",
                "Aggregate counts",
                DataVector::Text(words),
                counts,
            )
            .with_legend_label("Aggregate counts")
            .with_semantic_group("wordcount")
            .with_style(
                SeriesStyle::empty()
                    .with_geometry_hint(GeometryHint::Bar)
                    .with_color_role("primary"),
            ),
        ],
    );
    plot.validate().map_err(|error| {
        PlatformError::new(ErrorCategory::Validation, error.to_string())
            .with_code("tools.wordcount.plot.invalid")
    })?;
    Ok(Some(plot))
}

fn count_windows(sequence: &str, word_size: usize) -> (usize, usize, BTreeMap<String, usize>) {
    let bytes = sequence.as_bytes();
    if bytes.len() < word_size {
        return (0, 0, BTreeMap::new());
    }

    let mut counted_windows = 0usize;
    let mut skipped_gap_windows = 0usize;
    let mut counts = BTreeMap::new();

    for start in 0..=bytes.len() - word_size {
        let word = &sequence[start..start + word_size];
        if word.contains('-') {
            skipped_gap_windows += 1;
            continue;
        }
        counted_windows += 1;
        *counts.entry(word.to_owned()).or_insert(0) += 1;
    }

    (counted_windows, skipped_gap_windows, counts)
}

/// Returns the frequency for a counted word.
#[must_use]
pub fn word_frequency(record: &WordcountRecord, word: &str) -> f64 {
    if record.counted_windows == 0 {
        return 0.0;
    }
    record.counts.get(word).copied().unwrap_or_default() as f64 / record.counted_windows as f64
}

#[cfg(test)]
mod tests {
    use super::{WordcountParams, run_wordcount, word_frequency};
    use crate::sequence_stream::SequenceInput;

    #[test]
    fn counts_overlapping_words_per_record_and_aggregate() {
        let outcome = run_wordcount(WordcountParams {
            input: SequenceInput::new(
                "/Users/stephen/Projects/emboss-rs/crates/emboss-tools/tests/fixtures/three_records.fasta",
            ),
            word_size: 2,
            min_count: 1,
        })
        .expect("wordcount should execute");

        assert_eq!(outcome.records.len(), 3);
        assert_eq!(outcome.records[0].counts.get("AC"), Some(&1));
        assert_eq!(outcome.records[0].counted_windows, 3);
        assert_eq!(outcome.aggregate.counts.get("TT"), Some(&3));
        assert!((word_frequency(&outcome.records[1], "TT") - 1.0).abs() < 1e-9);
        assert!(outcome.plot.is_some());
    }

    #[test]
    fn skips_gap_windows() {
        let outcome = run_wordcount(WordcountParams {
            input: SequenceInput::new(
                "/Users/stephen/Projects/emboss-rs/crates/emboss-tools/tests/fixtures/gapped_records.fasta",
            ),
            word_size: 2,
            min_count: 1,
        })
        .expect("wordcount should execute");

        assert_eq!(outcome.records.len(), 2);
        assert_eq!(outcome.records[0].skipped_gap_windows, 2);
        assert_eq!(outcome.records[0].counted_windows, 3);
        assert_eq!(outcome.records[0].counts.get("G."), Some(&1));
    }

    #[test]
    fn returns_empty_counts_when_word_is_longer_than_sequence() {
        let outcome = run_wordcount(WordcountParams {
            input: SequenceInput::new(
                "/Users/stephen/Projects/emboss-rs/crates/emboss-tools/tests/fixtures/three_records.fasta",
            ),
            word_size: 10,
            min_count: 1,
        })
        .expect("wordcount should execute");

        assert!(outcome.records.iter().all(|record| record.counts.is_empty()));
        assert!(outcome.aggregate.counts.is_empty());
        assert!(outcome.plot.is_none());
    }
}
