//! Internal `pepinfo` implementation under staged plotting rollout.

use emboss_core::{ProteinPepinfoError, ProteinPepinfoProfile, protein_pepinfo_profile};
use emboss_diagnostics::{ErrorCategory, PlatformError};
use emboss_plot_contract::{
    AxisScaleHint, DataVector, GeometryHint, PlotAxis, PlotKind, PlotMetadata, PlotPayload,
    PlotProvenance, PlotSeries, PlotSpec, SeriesStyle,
};

use crate::sequence_stream::{SequenceInput, ToolExecutionError, load_sequence_records};

/// Typed parameters for the staged `pepinfo` tool path.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PepinfoParams {
    /// Local protein sequence input path.
    pub input: SequenceInput,
    /// Sliding-window length.
    pub window: usize,
    /// Sliding-window step size.
    pub step: usize,
}

/// Structured staged `pepinfo` outcome.
#[derive(Clone, Debug, PartialEq)]
pub struct PepinfoOutcome {
    /// Source input.
    pub input: SequenceInput,
    /// Computed profile.
    pub profile: ProteinPepinfoProfile,
    /// Plot-ready multi-series line contract.
    pub plot: PlotPayload,
}

/// Returns staged `pepinfo` help text.
#[must_use]
#[allow(dead_code)]
pub fn pepinfo_help() -> &'static str {
    "Usage: emboss-rs pepinfo <input> [--window <length>] [--step <length>] [--plot-contract-out <path>]\n\nCompute a bounded sliding-window multi-property protein profile for exactly one protein record. The staged v1 path derives hydropathy, residue-mass, charged-fraction, and polar-fraction series from the same analytical table and can serialize a typed multi-series line-plot contract."
}

/// Executes the staged `pepinfo` path.
pub fn run_pepinfo(params: PepinfoParams) -> Result<PepinfoOutcome, ToolExecutionError> {
    let records = load_sequence_records(&params.input)?;
    let [record]: [_; 1] = records.try_into().map_err(|records: Vec<_>| {
        PlatformError::new(
            ErrorCategory::Validation,
            "pepinfo requires exactly one protein sequence record in the bounded Phase 1 rollout",
        )
        .with_code("tools.pepinfo.input.record_count_invalid")
        .with_detail(format!("observed {} records", records.len()))
    })?;

    let profile =
        protein_pepinfo_profile(&record, params.window, params.step).map_err(map_error)?;
    let plot = build_pepinfo_plot(&profile)?;

    Ok(PepinfoOutcome {
        input: params.input,
        profile,
        plot,
    })
}

fn build_pepinfo_plot(profile: &ProteinPepinfoProfile) -> Result<PlotPayload, ToolExecutionError> {
    let x = profile
        .windows
        .iter()
        .map(|window| window.window_start as f64)
        .collect::<Vec<_>>();
    let plot = PlotSpec::new(
        PlotKind::Line,
        PlotMetadata::new(
            format!("pepinfo_{}", profile.identifier),
            format!("Pepinfo profile for {}", profile.identifier),
        )
        .with_subtitle(format!("Window {} step {}", profile.window, profile.step))
        .with_provenance(PlotProvenance {
            tool: Some("pepinfo".to_owned()),
            method: Some("protein_pepinfo_profile".to_owned()),
            source_artifact_ids: vec!["table:pepinfo-profile".to_owned()],
        }),
        PlotAxis::new("Window start").with_scale_hint(AxisScaleHint::Linear),
        PlotAxis::new("Profile value").with_scale_hint(AxisScaleHint::Linear),
        vec![
            PlotSeries::new(
                "pepinfo_hydropathy",
                "Mean hydropathy",
                DataVector::Numeric(x.clone()),
                profile
                    .windows
                    .iter()
                    .map(|window| window.mean_hydropathy)
                    .collect(),
            )
            .with_legend_label("Mean hydropathy")
            .with_semantic_group("hydropathy")
            .with_style(
                SeriesStyle::empty()
                    .with_geometry_hint(GeometryHint::Line)
                    .with_color_role("primary"),
            ),
            PlotSeries::new(
                "pepinfo_residue_mass",
                "Mean residue mass",
                DataVector::Numeric(x.clone()),
                profile
                    .windows
                    .iter()
                    .map(|window| window.mean_residue_mass)
                    .collect(),
            )
            .with_legend_label("Mean residue mass")
            .with_semantic_group("residue_mass")
            .with_style(
                SeriesStyle::empty()
                    .with_geometry_hint(GeometryHint::Line)
                    .with_color_role("secondary"),
            ),
            PlotSeries::new(
                "pepinfo_charged_fraction",
                "Charged fraction",
                DataVector::Numeric(x.clone()),
                profile
                    .windows
                    .iter()
                    .map(|window| window.charged_fraction)
                    .collect(),
            )
            .with_legend_label("Charged fraction")
            .with_semantic_group("charged_fraction")
            .with_style(
                SeriesStyle::empty()
                    .with_geometry_hint(GeometryHint::Line)
                    .with_color_role("tertiary"),
            ),
            PlotSeries::new(
                "pepinfo_polar_fraction",
                "Polar fraction",
                DataVector::Numeric(x),
                profile
                    .windows
                    .iter()
                    .map(|window| window.polar_fraction)
                    .collect(),
            )
            .with_legend_label("Polar fraction")
            .with_semantic_group("polar_fraction")
            .with_style(
                SeriesStyle::empty()
                    .with_geometry_hint(GeometryHint::Line)
                    .with_color_role("quaternary"),
            ),
        ],
    );
    plot.validate().map_err(|error| {
        PlatformError::new(ErrorCategory::Validation, error.to_string())
            .with_code("tools.pepinfo.plot.invalid")
    })?;
    Ok(plot)
}

fn map_error(error: ProteinPepinfoError) -> ToolExecutionError {
    let code = match error {
        ProteinPepinfoError::NonProteinSequence => "tools.pepinfo.input.non_protein",
        ProteinPepinfoError::UnsupportedResidue { .. } => "tools.pepinfo.input.unsupported_residue",
        ProteinPepinfoError::InvalidWindow { .. } => "tools.pepinfo.window.invalid",
        ProteinPepinfoError::InvalidStep { .. } => "tools.pepinfo.step.invalid",
        ProteinPepinfoError::SequenceShorterThanWindow { .. } => {
            "tools.pepinfo.window.sequence_too_short"
        }
    };
    PlatformError::new(ErrorCategory::Validation, error.to_string()).with_code(code)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use emboss_diagnostics::PlatformError;
    use emboss_plot_contract::PlotKind;

    use super::{PepinfoParams, build_pepinfo_plot, map_error, run_pepinfo};
    use crate::sequence_stream::SequenceInput;
    use emboss_core::{PepinfoWindow, ProteinPepinfoError, ProteinPepinfoProfile};

    fn fixture_path(name: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("fixtures")
            .join(name)
    }

    #[test]
    fn computes_profile_and_plot_contract() {
        let outcome = run_pepinfo(PepinfoParams {
            input: SequenceInput::new(fixture_path("pepinfo_protein.fasta")),
            window: 3,
            step: 1,
        })
        .expect("pepinfo should execute");

        assert_eq!(outcome.profile.identifier, "pepinfo_example");
        assert_eq!(outcome.profile.windows.len(), 7);
        assert_eq!(outcome.plot.kind, PlotKind::Line);
        assert_eq!(outcome.plot.series.len(), 4);
        assert_eq!(
            outcome
                .plot
                .metadata
                .provenance
                .as_ref()
                .and_then(|p| p.tool.as_deref()),
            Some("pepinfo")
        );
    }

    #[test]
    fn rejects_invalid_record_count() {
        let error = run_pepinfo(PepinfoParams {
            input: SequenceInput::new(fixture_path("three_records.fasta")),
            window: 3,
            step: 1,
        })
        .expect_err("multiple records should fail");

        assert_eq!(
            error.code().as_deref(),
            Some("tools.pepinfo.input.record_count_invalid")
        );
    }

    #[test]
    fn maps_invalid_window_error() {
        let mapped: PlatformError = map_error(ProteinPepinfoError::InvalidWindow { window: 0 });
        assert_eq!(
            mapped.code().as_deref(),
            Some("tools.pepinfo.window.invalid")
        );
    }

    #[test]
    fn validates_multi_series_plot_shape() {
        let plot = build_pepinfo_plot(&ProteinPepinfoProfile {
            identifier: "pepinfo_profile".to_owned(),
            sequence_length: 4,
            window: 2,
            step: 1,
            windows: vec![
                PepinfoWindow {
                    window_start: 1,
                    window_end: 2,
                    window_length: 2,
                    mean_hydropathy: -1.05,
                    mean_residue_mass: 99.626_45,
                    charged_fraction: 0.5,
                    polar_fraction: 0.5,
                },
                PepinfoWindow {
                    window_start: 2,
                    window_end: 3,
                    window_length: 2,
                    mean_hydropathy: -3.7,
                    mean_residue_mass: 121.631_35,
                    charged_fraction: 1.0,
                    polar_fraction: 1.0,
                },
            ],
        })
        .expect("plot should build");

        assert_eq!(plot.series.len(), 4);
        assert_eq!(
            plot.series
                .iter()
                .map(|series| series.semantic_group.as_deref())
                .collect::<Vec<_>>(),
            vec![
                Some("hydropathy"),
                Some("residue_mass"),
                Some("charged_fraction"),
                Some("polar_fraction"),
            ]
        );
    }
}
