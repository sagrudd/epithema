//! `inforesidue` implementation.

use epithema_core::{ProteinResidueProperty, protein_residue_property};
use epithema_diagnostics::{ErrorCategory, PlatformError};

use crate::sequence_stream::ToolExecutionError;

/// Typed parameters for `inforesidue`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct InforesidueParams {
    /// Queried amino-acid residue.
    pub residue: char,
}

/// Structured `inforesidue` outcome.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InforesidueOutcome {
    /// Queried amino-acid residue.
    pub residue: char,
    /// Stable residue-property row.
    pub property: ProteinResidueProperty,
}

/// Returns `inforesidue` help text.
#[must_use]
pub fn inforesidue_help() -> &'static str {
    "Usage: epithema inforesidue <residue>\n\nReport deterministic metadata for one canonical amino-acid residue. The v1 implementation reports one stable row with residue naming, average residue mass, Kyte-Doolittle hydropathy score, and coarse charge/polarity classes."
}

/// Executes `inforesidue`.
pub fn run_inforesidue(
    params: InforesidueParams,
) -> Result<InforesidueOutcome, ToolExecutionError> {
    let property = protein_residue_property(params.residue).ok_or_else(|| {
        ToolExecutionError::from(
            PlatformError::new(
                ErrorCategory::Validation,
                format!("unsupported amino-acid residue '{}'", params.residue),
            )
            .with_code("tools.inforesidue.residue.unsupported"),
        )
    })?;

    Ok(InforesidueOutcome {
        residue: params.residue.to_ascii_uppercase(),
        property,
    })
}

#[cfg(test)]
mod tests {
    use super::{InforesidueParams, run_inforesidue};

    #[test]
    fn reports_residue_property_info() {
        let outcome = run_inforesidue(InforesidueParams { residue: 'k' })
            .expect("inforesidue should execute");
        assert_eq!(outcome.property.residue, 'K');
        assert_eq!(outcome.property.three_letter, "Lys");
        assert_eq!(outcome.property.charge_class, "positive");
    }

    #[test]
    fn rejects_unknown_residue() {
        let error = run_inforesidue(InforesidueParams { residue: 'X' })
            .expect_err("unknown residue should fail");
        assert!(error.to_string().contains("unsupported amino-acid residue"));
    }
}
