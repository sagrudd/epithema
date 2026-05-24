//! Internal `density` implementation under staged plotting rollout.

use emboss_core::{NucleotideDensityError, NucleotideDensityProfile, nucleotide_density_profile};
use emboss_diagnostics::{ErrorCategory, PlatformError};
use emboss_plot_contract::{
    AxisScaleHint, DataVector, GeometryHint, PlotAxis, PlotKind, PlotMetadata, PlotPayload,
    PlotProvenance, PlotSeries, PlotSpec, SeriesStyle,
};

use crate::sequence_stream::{SequenceInput, ToolExecutionError, load_sequence_records};

/// Typed parameters for the staged `density` tool path.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DensityParams {
    /// Local nucleotide sequence input path.
    pub input: SequenceInput,
    /// Sliding-window length.
    pub window: usize,
    /// Sliding-window step size.
    pub step: usize,
}

/// Structured staged `density` outcome.
#[derive(Clone, Debug, PartialEq)]
pub struct DensityOutcome {
    /// Source input.
    pub input: SequenceInput,
    /// Computed profile.
    pub profile: NucleotideDensityProfile,
    /// Plot-ready single-series line contract.
    pub plot: PlotPayload,
}

/// Returns staged `density` help text.
#[must_use]
pub fn density_help() -> &'static str {
    "Usage: emboss-rs density <input> [--window <length>] [--step <length>] [--plot-contract-out <path>]\n\nCompute a bounded sliding-window nucleotide-density table for exactly one nucleotide record. The staged v1 path keeps the analytical table explicit, derives a single-series GC-fraction line contract from the same computation path, and can serialize that typed plot-contract JSON."
}

/// Executes the staged `density` path.
pub fn run_density(params: DensityParams) -> Result<DensityOutcome, ToolExecutionError> {
    let records = load_sequence_records(&params.input)?;
    let [record]: [_; 1] = records.try_into().map_err(|records: Vec<_>| {
        PlatformError::new(
            ErrorCategory::Validation,
            "density requires exactly one nucleotide sequence record in the bounded Phase 2 rollout",
        )
        .with_code("tools.density.input.record_count_invalid")
        .with_detail(format!("observed {} records", records.len()))
    })?;

    let profile =
        nucleotide_density_profile(&record, params.window, params.step).map_err(map_error)?;
    let plot = build_density_plot(&profile)?;

    Ok(DensityOutcome {
        input: params.input,
        profile,
        plot,
    })
}

fn build_density_plot(
    profile: &NucleotideDensityProfile,
) -> Result<PlotPayload, ToolExecutionError> {
    let plot = PlotSpec::new(
        PlotKind::Line,
        PlotMetadata::new(
            format!("density_{}", profile.identifier),
            format!("GC density profile for {}", profile.identifier),
        )
        .with_subtitle(format!(
            "Window {} step {} (single-series bounded v1 GC line)",
            profile.window, profile.step
        ))
        .with_provenance(PlotProvenance {
            tool: Some("density".to_owned()),
            method: Some("nucleotide_density_profile".to_owned()),
            source_artifact_ids: vec!["table:density-profile".to_owned()],
        }),
        PlotAxis::new("Window start").with_scale_hint(AxisScaleHint::Linear),
        PlotAxis::new("GC fraction").with_scale_hint(AxisScaleHint::Linear),
        vec![
            PlotSeries::new(
                "density_gc_fraction",
                "GC fraction",
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
                    .map(|window| window.gc_fraction)
                    .collect(),
            )
            .with_legend_label("GC fraction")
            .with_semantic_group("gc_fraction")
            .with_style(
                SeriesStyle::empty()
                    .with_geometry_hint(GeometryHint::Line)
                    .with_color_role("primary"),
            ),
        ],
    );
    plot.validate().map_err(|error| {
        PlatformError::new(ErrorCategory::Validation, error.to_string())
            .with_code("tools.density.plot.invalid")
    })?;
    Ok(plot)
}

fn map_error(error: NucleotideDensityError) -> ToolExecutionError {
    let code = match error {
        NucleotideDensityError::NonNucleotideSequence => "tools.density.input.non_nucleotide",
        NucleotideDensityError::InvalidWindow { .. } => "tools.density.window.invalid",
        NucleotideDensityError::InvalidStep { .. } => "tools.density.step.invalid",
        NucleotideDensityError::SequenceShorterThanWindow { .. } => {
            "tools.density.window.sequence_too_short"
        }
    };
    PlatformError::new(ErrorCategory::Validation, error.to_string()).with_code(code)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use emboss_diagnostics::PlatformError;
    use emboss_plot_contract::PlotKind;

    use super::{DensityParams, build_density_plot, map_error, run_density};
    use crate::sequence_stream::SequenceInput;
    use emboss_core::{DensityWindow, NucleotideDensityError, NucleotideDensityProfile};

    fn fixture_path(name: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("fixtures")
            .join(name)
    }

    #[test]
    fn computes_profile_and_plot_contract() {
        let outcome = run_density(DensityParams {
            input: SequenceInput::new(fixture_path("density_nucleotide.fasta")),
            window: 4,
            step: 1,
        })
        .expect("density should execute");

        assert_eq!(outcome.profile.identifier, "density_example");
        assert_eq!(outcome.profile.windows.len(), 4);
        assert_eq!(outcome.plot.kind, PlotKind::Line);
        assert_eq!(outcome.plot.series.len(), 1);
        assert_eq!(
            outcome
                .plot
                .metadata
                .provenance
                .as_ref()
                .and_then(|p| p.tool.as_deref()),
            Some("density")
        );
    }

    #[test]
    fn rejects_invalid_record_count() {
        let error = run_density(DensityParams {
            input: SequenceInput::new(fixture_path("three_records.fasta")),
            window: 4,
            step: 1,
        })
        .expect_err("multiple records should fail");

        assert_eq!(
            error.code().as_deref(),
            Some("tools.density.input.record_count_invalid")
        );
    }

    #[test]
    fn maps_invalid_window_error() {
        let mapped: PlatformError = map_error(NucleotideDensityError::InvalidWindow { window: 0 });
        assert_eq!(
            mapped.code().as_deref(),
            Some("tools.density.window.invalid")
        );
    }

    #[test]
    fn validates_plot_shape() {
        let plot = build_density_plot(&NucleotideDensityProfile {
            identifier: "density_profile".to_owned(),
            sequence_length: 5,
            window: 4,
            step: 1,
            windows: vec![
                DensityWindow {
                    window_start: 1,
                    window_end: 4,
                    window_length: 4,
                    canonical_symbols: 4,
                    ambiguous_symbols: 0,
                    ignored_gap_symbols: 0,
                    adenine_fraction: 0.5,
                    cytosine_fraction: 0.25,
                    guanine_fraction: 0.25,
                    thymine_or_uracil_fraction: 0.0,
                    at_fraction: 0.5,
                    gc_fraction: 0.5,
                },
                DensityWindow {
                    window_start: 2,
                    window_end: 5,
                    window_length: 4,
                    canonical_symbols: 4,
                    ambiguous_symbols: 0,
                    ignored_gap_symbols: 0,
                    adenine_fraction: 0.25,
                    cytosine_fraction: 0.25,
                    guanine_fraction: 0.25,
                    thymine_or_uracil_fraction: 0.25,
                    at_fraction: 0.5,
                    gc_fraction: 0.5,
                },
            ],
        })
        .expect("plot should build");

        assert_eq!(
            plot.series[0].semantic_group.as_deref(),
            Some("gc_fraction")
        );
    }
}
