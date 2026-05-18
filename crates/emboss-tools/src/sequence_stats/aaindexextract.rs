//! `aaindexextract` implementation.

use emboss_core::{ProteinResidueProperty, protein_residue_properties};
use emboss_diagnostics::{ErrorCategory, PlatformError};

use crate::sequence_stream::ToolExecutionError;

/// Supported built-in property indices for `aaindexextract`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AaindexBuiltIn {
    /// Kyte-Doolittle hydropathy score.
    HydropathyKyteDoolittle,
    /// Average residue mass.
    AverageMass,
    /// Coarse charge class.
    ChargeClass,
    /// Coarse polarity class.
    PolarityClass,
}

impl AaindexBuiltIn {
    /// Stable index name.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::HydropathyKyteDoolittle => "hydropathy_kyte_doolittle",
            Self::AverageMass => "average_mass",
            Self::ChargeClass => "charge_class",
            Self::PolarityClass => "polarity_class",
        }
    }

    /// Human-readable description.
    #[must_use]
    pub const fn description(self) -> &'static str {
        match self {
            Self::HydropathyKyteDoolittle => "Kyte-Doolittle residue hydropathy scores",
            Self::AverageMass => "Average residue masses in Daltons",
            Self::ChargeClass => "Coarse residue charge classes",
            Self::PolarityClass => "Coarse residue polarity classes",
        }
    }

    /// Units or category label for the emitted values.
    #[must_use]
    pub const fn units(self) -> &'static str {
        match self {
            Self::HydropathyKyteDoolittle => "hydropathy_units",
            Self::AverageMass => "daltons",
            Self::ChargeClass | Self::PolarityClass => "class",
        }
    }

    /// Stable notes for the built-in subset.
    #[must_use]
    pub const fn notes(self) -> &'static str {
        match self {
            Self::HydropathyKyteDoolittle => {
                "governed built-in subset; not full historical AAINDEX coverage"
            }
            Self::AverageMass => {
                "same residue-mass table used by pepstats molecular-weight estimation"
            }
            Self::ChargeClass => "v1 reports coarse positive/negative/neutral classes only",
            Self::PolarityClass => {
                "v1 reports coarse nonpolar/polar/basic/acidic/aromatic classes only"
            }
        }
    }

    /// Parses a user-facing built-in index name.
    pub fn parse(raw: &str) -> Option<Self> {
        match raw.to_ascii_lowercase().as_str() {
            "hydropathy" | "hydropathy_kyte_doolittle" => Some(Self::HydropathyKyteDoolittle),
            "mass" | "average_mass" => Some(Self::AverageMass),
            "charge" | "charge_class" => Some(Self::ChargeClass),
            "polarity" | "polarity_class" => Some(Self::PolarityClass),
            _ => None,
        }
    }
}

/// Typed parameters for `aaindexextract`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AaindexextractParams {
    /// Requested built-in property index.
    pub index: AaindexBuiltIn,
}

/// One output row from `aaindexextract`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AaindexextractRow {
    /// Index name.
    pub index: String,
    /// One-letter amino-acid code.
    pub residue: char,
    /// Three-letter amino-acid code.
    pub three_letter: String,
    /// Residue name.
    pub name: String,
    /// Property value rendered as a stable string.
    pub value: String,
    /// Units or value class.
    pub units: String,
    /// Stable notes.
    pub notes: String,
}

/// Structured `aaindexextract` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AaindexextractOutcome {
    /// Requested built-in index.
    pub index: AaindexBuiltIn,
    /// Stable rows in residue order.
    pub rows: Vec<AaindexextractRow>,
}

/// Returns `aaindexextract` help text.
#[must_use]
pub fn aaindexextract_help() -> &'static str {
    "Usage: emboss-rs aaindexextract <index>\n\nReport one governed built-in amino-acid property table. The v1 implementation exposes a small typed subset rather than the full historical AAINDEX corpus. Supported indices: hydropathy_kyte_doolittle, average_mass, charge_class, polarity_class."
}

/// Executes `aaindexextract`.
pub fn run_aaindexextract(
    params: AaindexextractParams,
) -> Result<AaindexextractOutcome, ToolExecutionError> {
    let rows = protein_residue_properties()
        .iter()
        .map(|property| render_row(params.index, property))
        .collect();

    Ok(AaindexextractOutcome {
        index: params.index,
        rows,
    })
}

fn render_row(index: AaindexBuiltIn, property: &ProteinResidueProperty) -> AaindexextractRow {
    AaindexextractRow {
        index: index.name().to_owned(),
        residue: property.residue,
        three_letter: property.three_letter.to_owned(),
        name: property.name.to_owned(),
        value: match index {
            AaindexBuiltIn::HydropathyKyteDoolittle => format!("{:.3}", property.hydropathy),
            AaindexBuiltIn::AverageMass => format!("{:.4}", property.average_mass),
            AaindexBuiltIn::ChargeClass => property.charge_class.to_owned(),
            AaindexBuiltIn::PolarityClass => property.polarity_class.to_owned(),
        },
        units: index.units().to_owned(),
        notes: index.notes().to_owned(),
    }
}

/// Parses a built-in index name into typed parameters.
pub fn parse_aaindexextract_index(raw: &str) -> Result<AaindexBuiltIn, ToolExecutionError> {
    AaindexBuiltIn::parse(raw).ok_or_else(|| {
        PlatformError::new(
            ErrorCategory::Validation,
            format!("unsupported aaindexextract index '{raw}'"),
        )
        .with_code("tools.aaindexextract.index.unsupported")
        .with_detail(
            "supported indices: hydropathy_kyte_doolittle, average_mass, charge_class, polarity_class",
        )
        .into()
    })
}

#[cfg(test)]
mod tests {
    use super::{
        AaindexBuiltIn, AaindexextractParams, parse_aaindexextract_index, run_aaindexextract,
    };

    #[test]
    fn reports_hydropathy_rows() {
        let outcome = run_aaindexextract(AaindexextractParams {
            index: AaindexBuiltIn::HydropathyKyteDoolittle,
        })
        .expect("aaindexextract should execute");

        assert_eq!(outcome.rows.len(), 20);
        assert_eq!(outcome.rows[0].residue, 'A');
        assert_eq!(outcome.rows[0].value, "1.800");
        assert_eq!(outcome.rows[1].residue, 'R');
        assert_eq!(outcome.rows[1].value, "-4.500");
    }

    #[test]
    fn parses_supported_aliases() {
        assert_eq!(
            parse_aaindexextract_index("hydropathy").expect("alias should parse"),
            AaindexBuiltIn::HydropathyKyteDoolittle
        );
        assert_eq!(
            parse_aaindexextract_index("mass").expect("alias should parse"),
            AaindexBuiltIn::AverageMass
        );
    }

    #[test]
    fn rejects_unknown_indices() {
        let error = parse_aaindexextract_index("mystery").expect_err("unknown index should fail");
        assert!(error.to_string().contains("unsupported aaindexextract index"));
    }
}
