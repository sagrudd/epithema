//! Internal `banana` implementation under staged plotting rollout.

use emboss_core::{NucleotideBananaError, NucleotideBananaProfile, nucleotide_banana_profile};
use emboss_diagnostics::{ErrorCategory, PlatformError};
use emboss_plot_contract::{
    AxisScaleHint, DataVector, GeometryHint, PlotAxis, PlotKind, PlotMetadata, PlotPayload,
    PlotProvenance, PlotSeries, PlotSpec, SeriesStyle,
};

use crate::sequence_stream::{SequenceInput, ToolExecutionError, load_sequence_records};

/// Typed parameters for the staged `banana` tool path.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BananaParams {
    /// Local nucleotide sequence input path.
    pub input: SequenceInput,
}

/// Structured staged `banana` outcome.
#[derive(Clone, Debug, PartialEq)]
pub struct BananaOutcome {
    /// Source input.
    pub input: SequenceInput,
    /// Computed profile.
    pub profile: NucleotideBananaProfile,
    /// Plot-ready single-series line contract.
    pub plot: PlotPayload,
}

/// Returns staged `banana` help text.
#[must_use]
pub fn banana_help() -> &'static str {
    "Usage: emboss-rs banana <input> [--plot-contract-out <path>]\n\nCompute a bounded per-base B-DNA bendability and curvature table for exactly one nucleotide record. The staged v1 path keeps the analytical table explicit, derives a single-series curvature line contract from the same computation path, and can serialize that typed plot-contract JSON."
}

/// Executes the staged `banana` path.
pub fn run_banana(params: BananaParams) -> Result<BananaOutcome, ToolExecutionError> {
    let records = load_sequence_records(&params.input)?;
    let [record]: [_; 1] = records.try_into().map_err(|records: Vec<_>| {
        PlatformError::new(
            ErrorCategory::Validation,
            "banana requires exactly one nucleotide sequence record in the bounded continuation rollout",
        )
        .with_code("tools.banana.input.record_count_invalid")
        .with_detail(format!("observed {} records", records.len()))
    })?;

    let profile = nucleotide_banana_profile(&record).map_err(map_error)?;
    let plot = build_banana_plot(&profile)?;

    Ok(BananaOutcome {
        input: params.input,
        profile,
        plot,
    })
}

fn build_banana_plot(profile: &NucleotideBananaProfile) -> Result<PlotPayload, ToolExecutionError> {
    let defined_points = profile
        .points
        .iter()
        .filter_map(|point| point.curvature.map(|curvature| (point.position as f64, curvature)))
        .collect::<Vec<_>>();

    let plot = PlotSpec::new(
        PlotKind::Line,
        PlotMetadata::new(
            format!("banana_{}", profile.identifier),
            format!("Banana profile for {}", profile.identifier),
        )
        .with_subtitle(
            "Bounded v1 curvature continuation line derived from the analytical table".to_owned(),
        )
        .with_provenance(PlotProvenance {
            tool: Some("banana".to_owned()),
            method: Some("nucleotide_banana_profile".to_owned()),
            source_artifact_ids: vec!["table:banana-profile".to_owned()],
        }),
        PlotAxis::new("Position").with_scale_hint(AxisScaleHint::Linear),
        PlotAxis::new("Curvature").with_scale_hint(AxisScaleHint::Linear),
        vec![
            PlotSeries::new(
                "banana_curvature",
                "Curvature",
                DataVector::Numeric(defined_points.iter().map(|(x, _)| *x).collect()),
                defined_points.iter().map(|(_, y)| *y).collect(),
            )
            .with_legend_label("Curvature")
            .with_semantic_group("curvature")
            .with_style(
                SeriesStyle::empty()
                    .with_geometry_hint(GeometryHint::Line)
                    .with_color_role("primary"),
            ),
        ],
    );
    plot.validate().map_err(|error| {
        PlatformError::new(ErrorCategory::Validation, error.to_string())
            .with_code("tools.banana.plot.invalid")
    })?;
    Ok(plot)
}

fn map_error(error: NucleotideBananaError) -> ToolExecutionError {
    let code = match error {
        NucleotideBananaError::NonNucleotideSequence => "tools.banana.input.non_nucleotide",
        NucleotideBananaError::SequenceTooShort { .. } => "tools.banana.sequence.too_short",
        NucleotideBananaError::UnsupportedResidue { .. } => {
            "tools.banana.input.unsupported_residue"
        }
    };
    PlatformError::new(ErrorCategory::Validation, error.to_string()).with_code(code)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use emboss_diagnostics::PlatformError;
    use emboss_plot_contract::PlotKind;

    use super::{BananaParams, build_banana_plot, map_error, run_banana};
    use crate::sequence_stream::SequenceInput;
    use emboss_core::{BananaPoint, NucleotideBananaError, NucleotideBananaProfile};

    fn fixture_path(name: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("fixtures")
            .join(name)
    }

    #[test]
    fn computes_profile_and_plot_contract() {
        let outcome = run_banana(BananaParams {
            input: SequenceInput::new(fixture_path("banana_nucleotide.fasta")),
        })
        .expect("banana should execute");

        assert_eq!(outcome.profile.identifier, "banana_example");
        assert_eq!(outcome.profile.points.len(), 45);
        assert_eq!(outcome.plot.kind, PlotKind::Line);
        assert_eq!(outcome.plot.series.len(), 1);
        assert_eq!(
            outcome
                .plot
                .metadata
                .provenance
                .as_ref()
                .and_then(|p| p.tool.as_deref()),
            Some("banana")
        );
    }

    #[test]
    fn rejects_invalid_record_count() {
        let error = run_banana(BananaParams {
            input: SequenceInput::new(fixture_path("three_records.fasta")),
        })
        .expect_err("multiple records should fail");

        assert_eq!(
            error.code().as_deref(),
            Some("tools.banana.input.record_count_invalid")
        );
    }

    #[test]
    fn maps_unsupported_residue_error() {
        let mapped: PlatformError = map_error(NucleotideBananaError::UnsupportedResidue {
            position: 4,
            residue: 'N',
        });
        assert_eq!(
            mapped.code().as_deref(),
            Some("tools.banana.input.unsupported_residue")
        );
    }

    #[test]
    fn validates_plot_shape() {
        let plot = build_banana_plot(&NucleotideBananaProfile {
            identifier: "banana_profile".to_owned(),
            sequence_length: 40,
            local_bend_bracket: 1,
            curvature_bracket: 15,
            smoothing_radius: 5,
            points: vec![
                BananaPoint {
                    position: 20,
                    residue: 'A',
                    local_bend: Some(3.36),
                    curvature: Some(10.48310492061),
                },
                BananaPoint {
                    position: 21,
                    residue: 'A',
                    local_bend: Some(0.0),
                    curvature: Some(9.8845),
                },
            ],
        })
        .expect("plot should build");

        assert_eq!(plot.series[0].semantic_group.as_deref(), Some("curvature"));
    }
}
