//! `pepwindow` implementation.

use emboss_core::{
    ProteinHydropathyError, ProteinHydropathyProfile, protein_hydropathy_profile,
};
use emboss_diagnostics::{ErrorCategory, PlatformError};
use emboss_plot_contract::{
    AxisScaleHint, DataVector, GeometryHint, PlotAxis, PlotKind, PlotMetadata, PlotPayload,
    PlotProvenance, PlotSeries, PlotSpec, SeriesStyle,
};

use crate::sequence_stream::{SequenceInput, ToolExecutionError, load_sequence_records};

/// Typed parameters for `pepwindow`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PepwindowParams {
    /// Local protein sequence input path.
    pub input: SequenceInput,
    /// Sliding-window length.
    pub window: usize,
    /// Sliding-window step size.
    pub step: usize,
}

/// Structured `pepwindow` outcome.
#[derive(Clone, Debug, PartialEq)]
pub struct PepwindowOutcome {
    /// Source input.
    pub input: SequenceInput,
    /// Computed profile.
    pub profile: ProteinHydropathyProfile,
    /// Plot-ready line contract.
    pub plot: PlotPayload,
}

/// Returns `pepwindow` help text.
#[must_use]
pub fn pepwindow_help() -> &'static str {
    "Usage: emboss-rs pepwindow <input> [--window <length>] [--step <length>] [--plot-contract-out <path>]\n\nCompute a sliding-window Kyte-Doolittle hydropathy profile for exactly one protein record. v1 uses the standard Kyte-Doolittle residue scale, defaults to --window 19 and --step 1, uses 1-based window-start positions on the x axis, and can write the typed line-plot contract JSON with --plot-contract-out."
}

/// Executes `pepwindow`.
pub fn run_pepwindow(params: PepwindowParams) -> Result<PepwindowOutcome, ToolExecutionError> {
    let records = load_sequence_records(&params.input)?;
    let [record]: [_; 1] = records.try_into().map_err(|records: Vec<_>| {
        PlatformError::new(
            ErrorCategory::Validation,
            "pepwindow requires exactly one protein sequence record in v1",
        )
        .with_code("tools.pepwindow.input.record_count_invalid")
        .with_detail(format!("observed {} records", records.len()))
    })?;

    let profile =
        protein_hydropathy_profile(&record, params.window, params.step).map_err(map_error)?;

    let plot = PlotSpec::new(
        PlotKind::Line,
        PlotMetadata::new(
            format!("pepwindow_{}", profile.identifier),
            format!("Hydropathy profile for {}", profile.identifier),
        )
        .with_subtitle(format!("Window {} step {}", profile.window, profile.step))
        .with_provenance(PlotProvenance {
            tool: Some("pepwindow".to_owned()),
            method: Some("protein_hydropathy_profile".to_owned()),
            source_artifact_ids: vec!["table:pepwindow-profile".to_owned()],
        }),
        PlotAxis::new("Window start").with_scale_hint(AxisScaleHint::Linear),
        PlotAxis::new("Mean hydropathy").with_scale_hint(AxisScaleHint::Linear),
        vec![
            PlotSeries::new(
                "pepwindow_profile",
                "Hydropathy profile",
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
                    .map(|window| window.mean_hydropathy)
                    .collect(),
            )
            .with_legend_label("Hydropathy profile")
            .with_semantic_group("hydropathy")
            .with_style(
                SeriesStyle::empty()
                    .with_geometry_hint(GeometryHint::Line)
                    .with_color_role("primary"),
            ),
        ],
    );
    plot.validate().map_err(|error| {
        PlatformError::new(ErrorCategory::Validation, error.to_string())
            .with_code("tools.pepwindow.plot.invalid")
    })?;

    Ok(PepwindowOutcome {
        input: params.input,
        profile,
        plot,
    })
}

fn map_error(error: ProteinHydropathyError) -> ToolExecutionError {
    let code = match error {
        ProteinHydropathyError::NonProteinSequence => "tools.pepwindow.input.non_protein",
        ProteinHydropathyError::UnsupportedResidue { .. } => {
            "tools.pepwindow.input.unsupported_residue"
        }
        ProteinHydropathyError::InvalidWindow { .. } => "tools.pepwindow.window.invalid",
        ProteinHydropathyError::InvalidStep { .. } => "tools.pepwindow.step.invalid",
        ProteinHydropathyError::SequenceShorterThanWindow { .. } => {
            "tools.pepwindow.window.sequence_too_short"
        }
    };
    PlatformError::new(ErrorCategory::Validation, error.to_string()).with_code(code)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::{PepwindowParams, run_pepwindow};
    use crate::sequence_stream::SequenceInput;

    fn fixture_path(name: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("fixtures")
            .join(name)
    }

    #[test]
    fn computes_hydropathy_profile_and_plot() {
        let outcome = run_pepwindow(PepwindowParams {
            input: SequenceInput::new(fixture_path("pepwindow_protein.fasta")),
            window: 5,
            step: 2,
        })
        .expect("pepwindow should execute");

        assert_eq!(outcome.profile.identifier, "pepwindow_example");
        assert_eq!(outcome.profile.windows.len(), 10);
        assert_eq!(outcome.profile.windows[0].window_start, 1);
        assert_eq!(outcome.profile.windows[0].window_end, 5);
        assert_eq!(outcome.plot.kind.as_str(), "line");
    }
}
