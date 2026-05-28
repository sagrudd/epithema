//! Internal `wobble` implementation under staged plotting rollout.

use emboss_core::{NucleotideWobbleError, NucleotideWobbleProfile, nucleotide_wobble_profile};
use emboss_diagnostics::{ErrorCategory, PlatformError};
use emboss_plot_contract::{
    AxisScaleHint, DataVector, GeometryHint, PlotAxis, PlotKind, PlotMetadata, PlotPayload,
    PlotProvenance, PlotSeries, PlotSpec, SeriesStyle,
};

use crate::sequence_stream::{SequenceInput, ToolExecutionError, load_sequence_records};

/// Typed parameters for the staged `wobble` tool path.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WobbleParams {
    /// Local coding nucleotide sequence input path.
    pub input: SequenceInput,
    /// Codon-window length.
    pub codon_window: usize,
    /// Codon-step size.
    pub codon_step: usize,
}

/// Structured staged `wobble` outcome.
#[derive(Clone, Debug, PartialEq)]
pub struct WobbleOutcome {
    /// Source input.
    pub input: SequenceInput,
    /// Computed profile.
    pub profile: NucleotideWobbleProfile,
    /// Plot-ready single-series line contract.
    pub plot: PlotPayload,
}

/// Returns staged `wobble` help text.
#[must_use]
pub fn wobble_help() -> &'static str {
    "Usage: emboss-rs wobble <input> [--codon-window <length>] [--codon-step <length>] [--plot-contract-out <path>]\n\nCompute a bounded third-base-position variability table for exactly one coding nucleotide record. The staged v1 path keeps the analytical table explicit, derives a single-series wobble-variability line contract from the same computation path, and can serialize that typed plot-contract JSON."
}

/// Executes the staged `wobble` path.
pub fn run_wobble(params: WobbleParams) -> Result<WobbleOutcome, ToolExecutionError> {
    let records = load_sequence_records(&params.input)?;
    let [record]: [_; 1] = records.try_into().map_err(|records: Vec<_>| {
        PlatformError::new(
            ErrorCategory::Validation,
            "wobble requires exactly one coding nucleotide sequence record in the bounded continuation rollout",
        )
        .with_code("tools.wobble.input.record_count_invalid")
        .with_detail(format!("observed {} records", records.len()))
    })?;

    let profile = nucleotide_wobble_profile(&record, params.codon_window, params.codon_step)
        .map_err(map_error)?;
    let plot = build_wobble_plot(&profile)?;

    Ok(WobbleOutcome {
        input: params.input,
        profile,
        plot,
    })
}

fn build_wobble_plot(profile: &NucleotideWobbleProfile) -> Result<PlotPayload, ToolExecutionError> {
    let plot = PlotSpec::new(
        PlotKind::Line,
        PlotMetadata::new(
            format!("wobble_{}", profile.identifier),
            format!("Wobble variability profile for {}", profile.identifier),
        )
        .with_subtitle(format!(
            "Codon window {} step {} (single-series bounded wobble variability line)",
            profile.codon_window, profile.codon_step
        ))
        .with_provenance(PlotProvenance {
            tool: Some("wobble".to_owned()),
            method: Some("nucleotide_wobble_profile".to_owned()),
            source_artifact_ids: vec!["table:wobble-profile".to_owned()],
        }),
        PlotAxis::new("Window start").with_scale_hint(AxisScaleHint::Linear),
        PlotAxis::new("Wobble variability").with_scale_hint(AxisScaleHint::Linear),
        vec![
            PlotSeries::new(
                "wobble_variability",
                "Wobble variability",
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
                    .map(|window| window.wobble_variability)
                    .collect(),
            )
            .with_legend_label("Wobble variability")
            .with_semantic_group("wobble_variability")
            .with_style(
                SeriesStyle::empty()
                    .with_geometry_hint(GeometryHint::Line)
                    .with_color_role("primary"),
            ),
        ],
    );
    plot.validate().map_err(|error| {
        PlatformError::new(ErrorCategory::Validation, error.to_string())
            .with_code("tools.wobble.plot.invalid")
    })?;
    Ok(plot)
}

fn map_error(error: NucleotideWobbleError) -> ToolExecutionError {
    let code = match error {
        NucleotideWobbleError::NonNucleotideSequence => "tools.wobble.input.non_nucleotide",
        NucleotideWobbleError::InvalidCodingSequence(ref cause) => match cause {
            emboss_core::CodonUsageError::NonCodingLength { .. } => {
                "tools.wobble.coding.non_coding_length"
            }
            emboss_core::CodonUsageError::InvalidCodon(_) => "tools.wobble.codon.invalid",
            emboss_core::CodonUsageError::AmbiguousCodon(_) => "tools.wobble.codon.ambiguous",
            emboss_core::CodonUsageError::InternalStopCodon(_) => {
                "tools.wobble.codon.internal_stop"
            }
        },
        NucleotideWobbleError::InvalidCodonWindow { .. } => "tools.wobble.codon_window.invalid",
        NucleotideWobbleError::InvalidCodonStep { .. } => "tools.wobble.codon_step.invalid",
        NucleotideWobbleError::SequenceShorterThanWindow { .. } => {
            "tools.wobble.codon_window.sequence_too_short"
        }
    };
    PlatformError::new(ErrorCategory::Validation, error.to_string()).with_code(code)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use emboss_diagnostics::PlatformError;
    use emboss_plot_contract::PlotKind;

    use super::{WobbleParams, build_wobble_plot, map_error, run_wobble};
    use crate::sequence_stream::SequenceInput;
    use emboss_core::{
        CodonUsageError, NucleotideWobbleError, NucleotideWobbleProfile, WobbleWindow,
    };

    fn fixture_path(name: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("fixtures")
            .join(name)
    }

    #[test]
    fn computes_profile_and_plot_contract() {
        let outcome = run_wobble(WobbleParams {
            input: SequenceInput::new(fixture_path("wobble_coding_nucleotide.fasta")),
            codon_window: 3,
            codon_step: 1,
        })
        .expect("wobble should execute");

        assert_eq!(outcome.profile.identifier, "wobble_example");
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
            Some("wobble")
        );
    }

    #[test]
    fn rejects_invalid_record_count() {
        let error = run_wobble(WobbleParams {
            input: SequenceInput::new(fixture_path("three_records.fasta")),
            codon_window: 3,
            codon_step: 1,
        })
        .expect_err("multiple records should fail");

        assert_eq!(
            error.code().as_deref(),
            Some("tools.wobble.input.record_count_invalid")
        );
    }

    #[test]
    fn maps_invalid_coding_error() {
        let mapped: PlatformError = map_error(NucleotideWobbleError::InvalidCodingSequence(
            CodonUsageError::AmbiguousCodon("NCC".to_owned()),
        ));
        assert_eq!(
            mapped.code().as_deref(),
            Some("tools.wobble.codon.ambiguous")
        );
    }

    #[test]
    fn validates_plot_shape() {
        let plot = build_wobble_plot(&NucleotideWobbleProfile {
            identifier: "wobble_profile".to_owned(),
            sequence_length: 15,
            codon_window: 3,
            codon_step: 1,
            windows: vec![
                WobbleWindow {
                    window_start: 1,
                    window_end: 9,
                    window_length: 9,
                    codon_window_length: 3,
                    wobble_positions: 3,
                    adenine_fraction: 0.0,
                    cytosine_fraction: 1.0 / 3.0,
                    guanine_fraction: 0.0,
                    thymine_fraction: 2.0 / 3.0,
                    dominant_wobble_fraction: 2.0 / 3.0,
                    wobble_variability: 1.0 / 3.0,
                },
                WobbleWindow {
                    window_start: 4,
                    window_end: 12,
                    window_length: 9,
                    codon_window_length: 3,
                    wobble_positions: 3,
                    adenine_fraction: 1.0 / 3.0,
                    cytosine_fraction: 1.0 / 3.0,
                    guanine_fraction: 0.0,
                    thymine_fraction: 1.0 / 3.0,
                    dominant_wobble_fraction: 1.0 / 3.0,
                    wobble_variability: 2.0 / 3.0,
                },
            ],
        })
        .expect("plot should build");

        assert_eq!(
            plot.series[0].semantic_group.as_deref(),
            Some("wobble_variability")
        );
    }
}
