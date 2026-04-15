//! `charge` implementation.

use emboss_core::{ProteinChargeError, ProteinChargeProfile, protein_charge_profile};
use emboss_diagnostics::{ErrorCategory, PlatformError};
use emboss_plot_contract::{
    AxisScaleHint, DataVector, GeometryHint, PlotAxis, PlotKind, PlotMetadata, PlotPayload,
    PlotProvenance, PlotSeries, PlotSpec, SeriesStyle,
};

use crate::sequence_stream::{SequenceInput, ToolExecutionError, load_sequence_records};

/// Typed parameters for `charge`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChargeParams {
    /// Local protein sequence input path.
    pub input: SequenceInput,
    /// Sliding-window length.
    pub window: usize,
    /// Sliding-window step size.
    pub step: usize,
}

/// Structured `charge` outcome.
#[derive(Clone, Debug, PartialEq)]
pub struct ChargeOutcome {
    /// Source input.
    pub input: SequenceInput,
    /// Computed profile.
    pub profile: ProteinChargeProfile,
    /// Plot-ready line contract.
    pub plot: PlotPayload,
}

/// Returns `charge` help text.
#[must_use]
pub fn charge_help() -> &'static str {
    "Usage: emboss-rs charge <input> [--window <length>] [--step <length>] [--plot-contract-out <path>]\n\nCompute a sliding-window mean protein charge profile for exactly one protein record. v1 uses D/E=-1.0, K/R=+1.0, H=+0.5, all other standard amino acids=0.0; defaults to --window 5 and --step 1; uses 1-based window-start positions on the x axis; and can write the typed line-plot contract JSON with --plot-contract-out."
}

/// Executes `charge`.
pub fn run_charge(params: ChargeParams) -> Result<ChargeOutcome, ToolExecutionError> {
    let records = load_sequence_records(&params.input)?;
    let [record]: [_; 1] = records.try_into().map_err(|records: Vec<_>| {
        PlatformError::new(
            ErrorCategory::Validation,
            "charge requires exactly one protein sequence record in v1",
        )
        .with_code("tools.charge.input.record_count_invalid")
        .with_detail(format!("observed {} records", records.len()))
    })?;

    let profile =
        protein_charge_profile(&record, params.window, params.step).map_err(map_charge_error)?;

    let plot = PlotSpec::new(
        PlotKind::Line,
        PlotMetadata::new(
            format!("charge_{}", profile.identifier),
            format!("Charge profile for {}", profile.identifier),
        )
        .with_subtitle(format!("Window {} step {}", profile.window, profile.step))
        .with_provenance(PlotProvenance {
            tool: Some("charge".to_owned()),
            method: Some("protein_charge_profile".to_owned()),
            source_artifact_ids: vec!["table:charge-profile".to_owned()],
        }),
        PlotAxis::new("Window start").with_scale_hint(AxisScaleHint::Linear),
        PlotAxis::new("Mean charge").with_scale_hint(AxisScaleHint::Linear),
        vec![
            PlotSeries::new(
                "charge_profile",
                "Charge profile",
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
                    .map(|window| window.mean_charge)
                    .collect(),
            )
            .with_legend_label("Charge profile")
            .with_semantic_group("charge")
            .with_style(
                SeriesStyle::empty()
                    .with_geometry_hint(GeometryHint::Line)
                    .with_color_role("primary"),
            ),
        ],
    );
    plot.validate().map_err(|error| {
        PlatformError::new(ErrorCategory::Validation, error.to_string())
            .with_code("tools.charge.plot.invalid")
    })?;

    Ok(ChargeOutcome {
        input: params.input,
        profile,
        plot,
    })
}

fn map_charge_error(error: ProteinChargeError) -> ToolExecutionError {
    let code = match error {
        ProteinChargeError::NonProteinSequence => "tools.charge.input.non_protein",
        ProteinChargeError::UnsupportedResidue { .. } => "tools.charge.input.unsupported_residue",
        ProteinChargeError::InvalidWindow { .. } => "tools.charge.window.invalid",
        ProteinChargeError::InvalidStep { .. } => "tools.charge.step.invalid",
        ProteinChargeError::SequenceShorterThanWindow { .. } => {
            "tools.charge.window.sequence_too_short"
        }
    };
    PlatformError::new(ErrorCategory::Validation, error.to_string()).with_code(code)
}
