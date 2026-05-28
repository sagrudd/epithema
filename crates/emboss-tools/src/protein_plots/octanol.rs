//! Internal `octanol` implementation under staged plotting rollout.

use emboss_core::{ProteinOctanolError, ProteinOctanolProfile, protein_octanol_profile};
use emboss_diagnostics::{ErrorCategory, PlatformError};
use emboss_plot_contract::{
    AxisScaleHint, DataVector, GeometryHint, PlotAxis, PlotKind, PlotMetadata, PlotPayload,
    PlotProvenance, PlotSeries, PlotSpec, SeriesStyle,
};

use crate::sequence_stream::{SequenceInput, ToolExecutionError, load_sequence_records};

/// Typed parameters for the staged `octanol` tool path.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OctanolParams {
    /// Local protein sequence input path.
    pub input: SequenceInput,
    /// Sliding-window length.
    pub window: usize,
    /// Sliding-window step size.
    pub step: usize,
}

/// Structured staged `octanol` outcome.
#[derive(Clone, Debug, PartialEq)]
pub struct OctanolOutcome {
    /// Source input.
    pub input: SequenceInput,
    /// Computed profile.
    pub profile: ProteinOctanolProfile,
    /// Plot-ready line contract.
    pub plot: PlotPayload,
}

/// Returns staged `octanol` help text.
#[must_use]
pub fn octanol_help() -> &'static str {
    "Usage: emboss-rs octanol <input> [--window <length>] [--step <length>] [--plot-contract-out <path>]\n\nCompute a sliding-window White-Wimley interface-minus-octanol profile for exactly one protein record. The bounded v1 rollout keeps `octanol` single-series, emits a table-first profile, and can serialize a typed line-plot contract from the same computation path."
}

/// Executes the staged `octanol` path.
pub fn run_octanol(params: OctanolParams) -> Result<OctanolOutcome, ToolExecutionError> {
    let records = load_sequence_records(&params.input)?;
    let [record]: [_; 1] = records.try_into().map_err(|records: Vec<_>| {
        PlatformError::new(
            ErrorCategory::Validation,
            "octanol requires exactly one protein sequence record in the bounded Phase 1 rollout",
        )
        .with_code("tools.octanol.input.record_count_invalid")
        .with_detail(format!("observed {} records", records.len()))
    })?;

    let profile =
        protein_octanol_profile(&record, params.window, params.step).map_err(map_error)?;
    let plot = build_octanol_plot(&profile)?;

    Ok(OctanolOutcome {
        input: params.input,
        profile,
        plot,
    })
}

fn build_octanol_plot(profile: &ProteinOctanolProfile) -> Result<PlotPayload, ToolExecutionError> {
    let plot = PlotSpec::new(
        PlotKind::Line,
        PlotMetadata::new(
            format!("octanol_{}", profile.identifier),
            format!("White-Wimley profile for {}", profile.identifier),
        )
        .with_subtitle(format!("Window {} step {}", profile.window, profile.step))
        .with_provenance(PlotProvenance {
            tool: Some("octanol".to_owned()),
            method: Some("protein_octanol_profile".to_owned()),
            source_artifact_ids: vec!["table:octanol-profile".to_owned()],
        }),
        PlotAxis::new("Window start").with_scale_hint(AxisScaleHint::Linear),
        PlotAxis::new("Interface minus octanol").with_scale_hint(AxisScaleHint::Linear),
        vec![
            PlotSeries::new(
                "octanol_profile",
                "Interface minus octanol profile",
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
                    .map(|window| window.interface_minus_octanol)
                    .collect(),
            )
            .with_legend_label("Interface minus octanol")
            .with_semantic_group("white_wimley_difference")
            .with_style(
                SeriesStyle::empty()
                    .with_geometry_hint(GeometryHint::Line)
                    .with_color_role("primary"),
            ),
        ],
    );
    plot.validate().map_err(|error| {
        PlatformError::new(ErrorCategory::Validation, error.to_string())
            .with_code("tools.octanol.plot.invalid")
    })?;
    Ok(plot)
}

fn map_error(error: ProteinOctanolError) -> ToolExecutionError {
    let code = match error {
        ProteinOctanolError::NonProteinSequence => "tools.octanol.input.non_protein",
        ProteinOctanolError::UnsupportedResidue { .. } => "tools.octanol.input.unsupported_residue",
        ProteinOctanolError::InvalidWindow { .. } => "tools.octanol.window.invalid",
        ProteinOctanolError::InvalidStep { .. } => "tools.octanol.step.invalid",
        ProteinOctanolError::SequenceShorterThanWindow { .. } => {
            "tools.octanol.window.sequence_too_short"
        }
    };
    PlatformError::new(ErrorCategory::Validation, error.to_string()).with_code(code)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use emboss_diagnostics::PlatformError;
    use emboss_plot_contract::PlotKind;

    use super::{OctanolParams, build_octanol_plot, map_error, run_octanol};
    use crate::sequence_stream::SequenceInput;
    use emboss_core::{OctanolWindow, ProteinOctanolError, ProteinOctanolProfile};

    fn fixture_path(name: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("fixtures")
            .join(name)
    }

    #[test]
    fn computes_profile_and_plot_contract() {
        let outcome = run_octanol(OctanolParams {
            input: SequenceInput::new(fixture_path("octanol_protein.fasta")),
            window: 3,
            step: 1,
        })
        .expect("octanol should execute");

        assert_eq!(outcome.profile.identifier, "octanol_example");
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
            Some("octanol")
        );
    }

    #[test]
    fn rejects_invalid_record_count() {
        let error = run_octanol(OctanolParams {
            input: SequenceInput::new(fixture_path("three_records.fasta")),
            window: 3,
            step: 1,
        })
        .expect_err("multiple records should fail");

        assert_eq!(
            error.code().as_deref(),
            Some("tools.octanol.input.record_count_invalid")
        );
    }

    #[test]
    fn maps_invalid_window_error() {
        let mapped: PlatformError = map_error(ProteinOctanolError::InvalidWindow { window: 0 });
        assert_eq!(
            mapped.code().as_deref(),
            Some("tools.octanol.window.invalid")
        );
    }

    #[test]
    fn validates_plot_shape() {
        let plot = build_octanol_plot(&ProteinOctanolProfile {
            identifier: "octanol_profile".to_owned(),
            sequence_length: 6,
            window: 3,
            step: 1,
            windows: vec![
                OctanolWindow {
                    window_start: 1,
                    window_end: 3,
                    window_length: 3,
                    interface_minus_octanol: -2.96,
                },
                OctanolWindow {
                    window_start: 2,
                    window_end: 4,
                    window_length: 3,
                    interface_minus_octanol: -4.24,
                },
            ],
        })
        .expect("plot should build");

        assert_eq!(
            plot.series[0].semantic_group.as_deref(),
            Some("white_wimley_difference")
        );
    }
}
