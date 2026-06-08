//! Internal `syco` implementation under staged plotting rollout.

use std::path::PathBuf;

use emboss_core::{
    CodonUsageError, NucleotideSycoError, NucleotideSycoProfile, nucleotide_syco_profile,
};
use emboss_diagnostics::{ErrorCategory, PlatformError};
use emboss_plot_contract::{
    AxisScaleHint, DataVector, GeometryHint, PlotAxis, PlotKind, PlotMetadata, PlotPayload,
    PlotProvenance, PlotSeries, PlotSpec, SeriesStyle,
};

use crate::codon_tools::load_profile_source;
use crate::sequence_stream::{SequenceInput, ToolExecutionError, load_sequence_records};

/// Typed parameters for the staged `syco` tool path.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SycoParams {
    /// Local coding nucleotide sequence input path.
    pub input: SequenceInput,
    /// Reference codon profile source.
    pub reference: PathBuf,
    /// Codon-window length.
    pub codon_window: usize,
    /// Codon-step size.
    pub codon_step: usize,
}

/// Structured staged `syco` outcome.
#[derive(Clone, Debug, PartialEq)]
pub struct SycoOutcome {
    /// Source input.
    pub input: SequenceInput,
    /// Reference codon profile source.
    pub reference: PathBuf,
    /// Computed profile.
    pub profile: NucleotideSycoProfile,
    /// Plot-ready single-series line contract.
    pub plot: PlotPayload,
}

/// Returns staged `syco` help text.
#[must_use]
pub fn syco_help() -> &'static str {
    "Usage: emboss-rs syco <input> <reference> [--codon-window <length>] [--codon-step <length>] [--plot-contract-out <path>]\n\nCompute a bounded synonymous-codon-preference table for exactly one coding nucleotide record against a reference codon-usage profile. The staged v1 path keeps the analytical table explicit, derives a single-series syco-score line contract from the same computation path, and can serialize that typed plot-contract JSON."
}

/// Executes the staged `syco` path.
pub fn run_syco(params: SycoParams) -> Result<SycoOutcome, ToolExecutionError> {
    let records = load_sequence_records(&params.input)?;
    let [record]: [_; 1] = records.try_into().map_err(|records: Vec<_>| {
        PlatformError::new(
            ErrorCategory::Validation,
            "syco requires exactly one coding nucleotide sequence record in the bounded continuation rollout",
        )
        .with_code("tools.syco.input.record_count_invalid")
        .with_detail(format!("observed {} records", records.len()))
    })?;

    let reference = load_profile_source(&params.reference)?;
    let profile =
        nucleotide_syco_profile(&record, &reference, params.codon_window, params.codon_step)
            .map_err(map_error)?;
    let plot = build_syco_plot(&profile)?;

    Ok(SycoOutcome {
        input: params.input,
        reference: params.reference,
        profile,
        plot,
    })
}

fn build_syco_plot(profile: &NucleotideSycoProfile) -> Result<PlotPayload, ToolExecutionError> {
    let plot = PlotSpec::new(
        PlotKind::Line,
        PlotMetadata::new(
            format!("syco_{}", profile.identifier),
            format!("Syco preference profile for {}", profile.identifier),
        )
        .with_subtitle(format!(
            "Codon window {} step {} (single-series bounded synonymous codon preference line)",
            profile.codon_window, profile.codon_step
        ))
        .with_provenance(PlotProvenance {
            tool: Some("syco".to_owned()),
            method: Some("nucleotide_syco_profile".to_owned()),
            source_artifact_ids: vec!["table:syco-profile".to_owned()],
        }),
        PlotAxis::new("Window start").with_scale_hint(AxisScaleHint::Linear),
        PlotAxis::new("Syco score").with_scale_hint(AxisScaleHint::Linear),
        vec![
            PlotSeries::new(
                "syco_score",
                "Syco score",
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
                    .map(|window| window.syco_score)
                    .collect(),
            )
            .with_legend_label("Syco score")
            .with_semantic_group("syco_score")
            .with_style(
                SeriesStyle::empty()
                    .with_geometry_hint(GeometryHint::Line)
                    .with_color_role("primary"),
            ),
        ],
    );
    plot.validate().map_err(|error| {
        PlatformError::new(ErrorCategory::Validation, error.to_string())
            .with_code("tools.syco.plot.invalid")
    })?;
    Ok(plot)
}

fn map_error(error: NucleotideSycoError) -> ToolExecutionError {
    let code = match error {
        NucleotideSycoError::NonNucleotideSequence => "tools.syco.input.non_nucleotide",
        NucleotideSycoError::InvalidCodingSequence(ref cause) => match cause {
            CodonUsageError::NonCodingLength { .. } => "tools.syco.coding.non_coding_length",
            CodonUsageError::InvalidCodon(_) => "tools.syco.codon.invalid",
            CodonUsageError::AmbiguousCodon(_) => "tools.syco.codon.ambiguous",
            CodonUsageError::InternalStopCodon(_) => "tools.syco.codon.internal_stop",
        },
        NucleotideSycoError::EmptyReferenceProfile => "tools.syco.reference.empty",
        NucleotideSycoError::InvalidCodonWindow { .. } => "tools.syco.codon_window.invalid",
        NucleotideSycoError::InvalidCodonStep { .. } => "tools.syco.codon_step.invalid",
        NucleotideSycoError::SequenceShorterThanWindow { .. } => {
            "tools.syco.codon_window.sequence_too_short"
        }
    };
    PlatformError::new(ErrorCategory::Validation, error.to_string()).with_code(code)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use emboss_diagnostics::PlatformError;
    use emboss_plot_contract::PlotKind;

    use super::{SycoParams, build_syco_plot, map_error, run_syco};
    use crate::sequence_stream::SequenceInput;
    use emboss_core::{CodonUsageError, NucleotideSycoError, NucleotideSycoProfile, SycoWindow};

    fn fixture_path(name: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("fixtures")
            .join(name)
    }

    #[test]
    fn computes_profile_and_plot_contract() {
        let outcome = run_syco(SycoParams {
            input: SequenceInput::new(fixture_path("syco_coding_nucleotide.fasta")),
            reference: fixture_path("codon_reference.fasta"),
            codon_window: 2,
            codon_step: 1,
        })
        .expect("syco should execute");

        assert_eq!(outcome.profile.identifier, "syco_example");
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
            Some("syco")
        );
    }

    #[test]
    fn rejects_invalid_record_count() {
        let error = run_syco(SycoParams {
            input: SequenceInput::new(fixture_path("three_records.fasta")),
            reference: fixture_path("codon_reference.fasta"),
            codon_window: 2,
            codon_step: 1,
        })
        .expect_err("multiple records should fail");

        assert_eq!(
            error.code().as_deref(),
            Some("tools.syco.input.record_count_invalid")
        );
    }

    #[test]
    fn maps_invalid_coding_error() {
        let mapped: PlatformError = map_error(NucleotideSycoError::InvalidCodingSequence(
            CodonUsageError::AmbiguousCodon("NCC".to_owned()),
        ));
        assert_eq!(mapped.code().as_deref(), Some("tools.syco.codon.ambiguous"));
    }

    #[test]
    fn validates_plot_shape() {
        let plot = build_syco_plot(&NucleotideSycoProfile {
            identifier: "syco_profile".to_owned(),
            sequence_length: 12,
            terminal_stop: None,
            codon_window: 2,
            codon_step: 1,
            sense_codon_count: 4,
            reference_sense_codon_count: 8,
            windows: vec![
                SycoWindow {
                    window_start: 1,
                    window_end: 6,
                    window_length: 6,
                    codon_window_length: 2,
                    sense_codon_count: 2,
                    syco_score: 0.5,
                },
                SycoWindow {
                    window_start: 4,
                    window_end: 9,
                    window_length: 6,
                    codon_window_length: 2,
                    sense_codon_count: 2,
                    syco_score: 0.75,
                },
            ],
        })
        .expect("plot should validate");

        assert_eq!(plot.series.len(), 1);
        assert_eq!(plot.series[0].y, vec![0.5, 0.75]);
    }
}
