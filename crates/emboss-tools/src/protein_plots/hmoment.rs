//! Internal `hmoment` implementation under staged plotting rollout.

use emboss_core::{
    ProteinHydrophobicMomentError, ProteinHydrophobicMomentProfile,
    protein_hydrophobic_moment_profile,
};
use emboss_diagnostics::{ErrorCategory, PlatformError};
use emboss_plot_contract::{
    AxisScaleHint, DataVector, GeometryHint, PlotAxis, PlotKind, PlotMetadata, PlotPayload,
    PlotProvenance, PlotSeries, PlotSpec, SeriesStyle,
};

use crate::sequence_stream::{SequenceInput, ToolExecutionError, load_sequence_records};

/// Typed parameters for the staged `hmoment` tool path.
#[derive(Clone, Debug, PartialEq)]
pub struct HmomentParams {
    /// Local protein sequence input path.
    pub input: SequenceInput,
    /// Sliding-window length.
    pub window: usize,
    /// Sliding-window step size.
    pub step: usize,
    /// Per-residue turn angle in degrees.
    pub angle_degrees: f64,
}

impl Eq for HmomentParams {}

/// Structured staged `hmoment` outcome.
#[derive(Clone, Debug, PartialEq)]
pub struct HmomentOutcome {
    /// Source input.
    pub input: SequenceInput,
    /// Computed profile.
    pub profile: ProteinHydrophobicMomentProfile,
    /// Plot-ready line contract.
    pub plot: PlotPayload,
}

/// Returns staged `hmoment` help text.
#[must_use]
pub fn hmoment_help() -> &'static str {
    "Usage: emboss-rs hmoment <input> [--window <length>] [--step <length>] [--angle-degrees <degrees>] [--plot-contract-out <path>]\n\nCompute a sliding-window protein hydrophobic-moment profile for exactly one protein record. The bounded v1 model uses a deterministic residue hydrophobicity table with a default turn angle of 100 degrees, emits a table-first single-series profile, and can serialize a typed line-plot contract."
}

/// Executes the staged `hmoment` path.
pub fn run_hmoment(params: HmomentParams) -> Result<HmomentOutcome, ToolExecutionError> {
    let records = load_sequence_records(&params.input)?;
    let [record]: [_; 1] = records.try_into().map_err(|records: Vec<_>| {
        PlatformError::new(
            ErrorCategory::Validation,
            "hmoment requires exactly one protein sequence record in the bounded Phase 1 rollout",
        )
        .with_code("tools.hmoment.input.record_count_invalid")
        .with_detail(format!("observed {} records", records.len()))
    })?;

    let profile = protein_hydrophobic_moment_profile(
        &record,
        params.window,
        params.step,
        params.angle_degrees,
    )
    .map_err(map_error)?;

    let plot = build_hmoment_plot(&profile)?;

    Ok(HmomentOutcome {
        input: params.input,
        profile,
        plot,
    })
}

fn build_hmoment_plot(
    profile: &ProteinHydrophobicMomentProfile,
) -> Result<PlotPayload, ToolExecutionError> {
    let plot = PlotSpec::new(
        PlotKind::Line,
        PlotMetadata::new(
            format!("hmoment_{}", profile.identifier),
            format!("Hydrophobic moment profile for {}", profile.identifier),
        )
        .with_subtitle(format!(
            "Window {} step {} angle {}",
            profile.window, profile.step, profile.angle_degrees
        ))
        .with_provenance(PlotProvenance {
            tool: Some("hmoment".to_owned()),
            method: Some("protein_hydrophobic_moment_profile".to_owned()),
            source_artifact_ids: vec!["table:hmoment-profile".to_owned()],
        }),
        PlotAxis::new("Window start").with_scale_hint(AxisScaleHint::Linear),
        PlotAxis::new("Hydrophobic moment").with_scale_hint(AxisScaleHint::Linear),
        vec![
            PlotSeries::new(
                "hmoment_profile",
                "Hydrophobic moment profile",
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
                    .map(|window| window.hydrophobic_moment)
                    .collect(),
            )
            .with_legend_label("Hydrophobic moment")
            .with_semantic_group("hydrophobic_moment")
            .with_style(
                SeriesStyle::empty()
                    .with_geometry_hint(GeometryHint::Line)
                    .with_color_role("primary"),
            ),
        ],
    );
    plot.validate().map_err(|error| {
        PlatformError::new(ErrorCategory::Validation, error.to_string())
            .with_code("tools.hmoment.plot.invalid")
    })?;
    Ok(plot)
}

fn map_error(error: ProteinHydrophobicMomentError) -> ToolExecutionError {
    let code = match error {
        ProteinHydrophobicMomentError::NonProteinSequence => "tools.hmoment.input.non_protein",
        ProteinHydrophobicMomentError::UnsupportedResidue { .. } => {
            "tools.hmoment.input.unsupported_residue"
        }
        ProteinHydrophobicMomentError::InvalidWindow { .. } => "tools.hmoment.window.invalid",
        ProteinHydrophobicMomentError::InvalidStep { .. } => "tools.hmoment.step.invalid",
        ProteinHydrophobicMomentError::InvalidAngle { .. } => "tools.hmoment.angle.invalid",
        ProteinHydrophobicMomentError::SequenceShorterThanWindow { .. } => {
            "tools.hmoment.window.sequence_too_short"
        }
    };
    PlatformError::new(ErrorCategory::Validation, error.to_string()).with_code(code)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use emboss_diagnostics::PlatformError;
    use emboss_plot_contract::PlotKind;

    use super::{HmomentParams, build_hmoment_plot, map_error, run_hmoment};
    use crate::sequence_stream::SequenceInput;
    use emboss_core::{
        DEFAULT_HYDROPHOBIC_MOMENT_ANGLE_DEGREES, HydrophobicMomentWindow,
        ProteinHydrophobicMomentError, ProteinHydrophobicMomentProfile,
    };

    fn fixture_path(name: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("fixtures")
            .join(name)
    }

    #[test]
    fn computes_profile_and_plot_contract() {
        let outcome = run_hmoment(HmomentParams {
            input: SequenceInput::new(fixture_path("hmoment_protein.fasta")),
            window: 4,
            step: 1,
            angle_degrees: DEFAULT_HYDROPHOBIC_MOMENT_ANGLE_DEGREES,
        })
        .expect("hmoment should execute");

        assert_eq!(outcome.profile.identifier, "hmoment_example");
        assert_eq!(outcome.profile.windows.len(), 2);
        assert_eq!(outcome.plot.kind, PlotKind::Line);
        assert_eq!(outcome.plot.series.len(), 1);
        assert_eq!(
            outcome
                .plot
                .metadata
                .provenance
                .as_ref()
                .and_then(|p| p.tool.as_deref()),
            Some("hmoment")
        );
    }

    #[test]
    fn rejects_invalid_record_count() {
        let error = run_hmoment(HmomentParams {
            input: SequenceInput::new(fixture_path("three_records.fasta")),
            window: 4,
            step: 1,
            angle_degrees: DEFAULT_HYDROPHOBIC_MOMENT_ANGLE_DEGREES,
        })
        .expect_err("multiple records should fail");

        assert_eq!(
            error.code().as_deref(),
            Some("tools.hmoment.input.record_count_invalid")
        );
    }

    #[test]
    fn maps_invalid_angle_error() {
        let mapped: PlatformError =
            map_error(ProteinHydrophobicMomentError::InvalidAngle { angle_degrees: 0.0 });
        assert_eq!(
            mapped.code().as_deref(),
            Some("tools.hmoment.angle.invalid")
        );
    }

    #[test]
    fn validates_plot_shape() {
        let plot = build_hmoment_plot(&ProteinHydrophobicMomentProfile {
            identifier: "hmoment_profile".to_owned(),
            sequence_length: 5,
            window: 4,
            step: 1,
            angle_degrees: DEFAULT_HYDROPHOBIC_MOMENT_ANGLE_DEGREES,
            windows: vec![
                HydrophobicMomentWindow {
                    window_start: 1,
                    window_end: 4,
                    window_length: 4,
                    hydrophobic_moment: 0.701_831_267_935_911_7,
                },
                HydrophobicMomentWindow {
                    window_start: 2,
                    window_end: 5,
                    window_length: 4,
                    hydrophobic_moment: 1.222_809_478_488_536_8,
                },
            ],
        })
        .expect("plot should build");

        assert_eq!(
            plot.series[0].semantic_group.as_deref(),
            Some("hydrophobic_moment")
        );
    }
}
