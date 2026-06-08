//! Internal `isochore` implementation under staged plotting rollout.

use emboss_core::{
    NucleotideIsochoreError, NucleotideIsochoreProfile, nucleotide_isochore_profile,
};
use emboss_diagnostics::{ErrorCategory, PlatformError};
use emboss_plot_contract::{
    AxisScaleHint, DataVector, GeometryHint, PlotAxis, PlotKind, PlotMetadata, PlotPayload,
    PlotProvenance, PlotSeries, PlotSpec, SeriesStyle,
};

use crate::sequence_stream::{SequenceInput, ToolExecutionError, load_sequence_records};

/// Typed parameters for the staged `isochore` tool path.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IsochoreParams {
    /// Local nucleotide sequence input path.
    pub input: SequenceInput,
    /// Sliding-window length.
    pub window: usize,
    /// Sliding-window step size.
    pub step: usize,
}

/// Structured staged `isochore` outcome.
#[derive(Clone, Debug, PartialEq)]
pub struct IsochoreOutcome {
    /// Source input.
    pub input: SequenceInput,
    /// Computed profile.
    pub profile: NucleotideIsochoreProfile,
    /// Plot-ready single-series line contract.
    pub plot: PlotPayload,
}

/// Returns staged `isochore` help text.
#[must_use]
pub fn isochore_help() -> &'static str {
    "Usage: emboss-rs isochore <input> [--window <length>] [--step <length>] [--plot-contract-out <path>]\n\nCompute a bounded sliding-window isochore table for exactly one nucleotide record. The staged v1 path keeps the analytical table explicit, derives a single-series GC-percent line contract from the same computation path, and can serialize that typed plot-contract JSON."
}

/// Executes the staged `isochore` path.
pub fn run_isochore(params: IsochoreParams) -> Result<IsochoreOutcome, ToolExecutionError> {
    let records = load_sequence_records(&params.input)?;
    let [record]: [_; 1] = records.try_into().map_err(|records: Vec<_>| {
        PlatformError::new(
            ErrorCategory::Validation,
            "isochore requires exactly one nucleotide sequence record in the bounded continuation rollout",
        )
        .with_code("tools.isochore.input.record_count_invalid")
        .with_detail(format!("observed {} records", records.len()))
    })?;

    let profile =
        nucleotide_isochore_profile(&record, params.window, params.step).map_err(map_error)?;
    let plot = build_isochore_plot(&profile)?;

    Ok(IsochoreOutcome {
        input: params.input,
        profile,
        plot,
    })
}

fn build_isochore_plot(
    profile: &NucleotideIsochoreProfile,
) -> Result<PlotPayload, ToolExecutionError> {
    let plot = PlotSpec::new(
        PlotKind::Line,
        PlotMetadata::new(
            format!("isochore_{}", profile.identifier),
            format!("Isochore profile for {}", profile.identifier),
        )
        .with_subtitle(format!(
            "Window {} step {} (single-series bounded v1 GC-percent line)",
            profile.window, profile.step
        ))
        .with_provenance(PlotProvenance {
            tool: Some("isochore".to_owned()),
            method: Some("nucleotide_isochore_profile".to_owned()),
            source_artifact_ids: vec!["table:isochore-profile".to_owned()],
        }),
        PlotAxis::new("Window start").with_scale_hint(AxisScaleHint::Linear),
        PlotAxis::new("GC percent").with_scale_hint(AxisScaleHint::Linear),
        vec![
            PlotSeries::new(
                "isochore_gc_percent",
                "GC percent",
                DataVector::Numeric(
                    profile
                        .windows
                        .iter()
                        .map(|window| window.window_start as f64)
                        .collect(),
                ),
                profile
                    .windows
                    .iter()
                    .map(|window| window.gc_percent)
                    .collect(),
            )
            .with_legend_label("GC percent")
            .with_semantic_group("gc_percent")
            .with_style(
                SeriesStyle::empty()
                    .with_geometry_hint(GeometryHint::Line)
                    .with_color_role("primary"),
            ),
        ],
    );
    plot.validate().map_err(|error| {
        PlatformError::new(ErrorCategory::Validation, error.to_string())
            .with_code("tools.isochore.plot.invalid")
    })?;
    Ok(plot)
}

fn map_error(error: NucleotideIsochoreError) -> ToolExecutionError {
    let code = match error {
        NucleotideIsochoreError::NonNucleotideSequence => "tools.isochore.input.non_nucleotide",
        NucleotideIsochoreError::InvalidWindow { .. } => "tools.isochore.window.invalid",
        NucleotideIsochoreError::InvalidStep { .. } => "tools.isochore.step.invalid",
        NucleotideIsochoreError::SequenceShorterThanWindow { .. } => {
            "tools.isochore.window.sequence_too_short"
        }
    };
    PlatformError::new(ErrorCategory::Validation, error.to_string()).with_code(code)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use emboss_diagnostics::PlatformError;
    use emboss_plot_contract::PlotKind;

    use super::{IsochoreParams, build_isochore_plot, map_error, run_isochore};
    use crate::sequence_stream::SequenceInput;
    use emboss_core::{
        IsochoreBand, IsochoreWindow, NucleotideIsochoreError, NucleotideIsochoreProfile,
    };

    fn fixture_path(name: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("fixtures")
            .join(name)
    }

    #[test]
    fn computes_profile_and_plot_contract() {
        let outcome = run_isochore(IsochoreParams {
            input: SequenceInput::new(fixture_path("isochore_nucleotide.fasta")),
            window: 4,
            step: 4,
        })
        .expect("isochore should execute");

        assert_eq!(outcome.profile.identifier, "isochore_example");
        assert_eq!(outcome.profile.windows.len(), 3);
        assert_eq!(outcome.plot.kind, PlotKind::Line);
        assert_eq!(outcome.plot.series.len(), 1);
        assert_eq!(
            outcome
                .plot
                .metadata
                .provenance
                .as_ref()
                .and_then(|p| p.tool.as_deref()),
            Some("isochore")
        );
    }

    #[test]
    fn rejects_invalid_record_count() {
        let error = run_isochore(IsochoreParams {
            input: SequenceInput::new(fixture_path("three_records.fasta")),
            window: 4,
            step: 1,
        })
        .expect_err("multiple records should fail");

        assert_eq!(
            error.code().as_deref(),
            Some("tools.isochore.input.record_count_invalid")
        );
    }

    #[test]
    fn maps_invalid_window_error() {
        let mapped: PlatformError = map_error(NucleotideIsochoreError::InvalidWindow { window: 0 });
        assert_eq!(
            mapped.code().as_deref(),
            Some("tools.isochore.window.invalid")
        );
    }

    #[test]
    fn validates_plot_shape() {
        let plot = build_isochore_plot(&NucleotideIsochoreProfile {
            identifier: "isochore_profile".to_owned(),
            sequence_length: 12,
            window: 4,
            step: 4,
            windows: vec![
                IsochoreWindow {
                    window_start: 1,
                    window_end: 4,
                    window_length: 4,
                    canonical_symbols: 4,
                    ambiguous_symbols: 0,
                    ignored_gap_symbols: 0,
                    at_fraction: 1.0,
                    gc_fraction: 0.0,
                    gc_percent: 0.0,
                    isochore_band: IsochoreBand::L1,
                },
                IsochoreWindow {
                    window_start: 5,
                    window_end: 8,
                    window_length: 4,
                    canonical_symbols: 4,
                    ambiguous_symbols: 0,
                    ignored_gap_symbols: 0,
                    at_fraction: 0.0,
                    gc_fraction: 1.0,
                    gc_percent: 100.0,
                    isochore_band: IsochoreBand::H3,
                },
            ],
        })
        .expect("plot should build");

        assert_eq!(plot.series[0].semantic_group.as_deref(), Some("gc_percent"));
    }
}
