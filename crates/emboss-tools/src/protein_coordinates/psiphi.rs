//! `psiphi` implementation.

use std::fs;
use std::path::PathBuf;

use emboss_core::{protein_psiphi_profile, ProteinPsiphiProfile};
use emboss_diagnostics::{ErrorCategory, PlatformError};

/// Local protein-coordinate input handled by the bounded `psiphi` seam.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PsiphiInput {
    /// Canonical local path to the coordinate input.
    pub path: PathBuf,
}

impl PsiphiInput {
    /// Creates a new local `psiphi` coordinate input wrapper.
    #[must_use]
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }
}

/// Shared execution error for bounded protein-coordinate tools.
pub type ToolExecutionError = PlatformError;

/// Typed parameters for `psiphi`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PsiphiParams {
    /// Local coordinate input path.
    pub input: PsiphiInput,
}

/// Structured `psiphi` outcome.
#[derive(Clone, Debug, PartialEq)]
pub struct PsiphiOutcome {
    /// Source input.
    pub input: PsiphiInput,
    /// Deterministic torsion-angle profile.
    pub profile: ProteinPsiphiProfile,
}

/// Returns the `psiphi` help text.
#[must_use]
pub fn psiphi_help() -> &'static str {
    "Usage: emboss-rs psiphi <protein-coordinate-input>\n\nReport deterministic per-residue phi/psi torsion angles from bounded PDB ATOM backbone coordinates. The v1 seam accepts local coordinate files only, retains only backbone N/CA/C atoms, ignores unsupported alternate locations, and leaves torsions absent when continuity or backbone atoms are missing."
}

/// Executes `psiphi`.
pub fn run_psiphi(params: PsiphiParams) -> Result<PsiphiOutcome, ToolExecutionError> {
    let coordinate_text = fs::read_to_string(&params.input.path).map_err(|error| {
        PlatformError::new(
            ErrorCategory::Validation,
            "failed to read protein coordinate input",
        )
        .with_code("tools.psiphi.input.read_failed")
        .with_detail(format!("{}: {error}", params.input.path.display()))
    })?;
    let profile = protein_psiphi_profile(&coordinate_text).map_err(|error| {
        PlatformError::new(ErrorCategory::Validation, error.to_string())
            .with_code("tools.psiphi.profile.invalid")
    })?;

    Ok(PsiphiOutcome {
        input: params.input,
        profile,
    })
}

#[cfg(test)]
mod tests {
    use super::{run_psiphi, PsiphiInput, PsiphiParams};

    fn fixture(name: &str) -> std::path::PathBuf {
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join(format!("../../crates/emboss-tools/tests/fixtures/{name}"))
    }

    #[test]
    fn reports_expected_torsion_rows_for_fixture() {
        let outcome = run_psiphi(PsiphiParams {
            input: PsiphiInput::new(fixture("psiphi_backbone.txt")),
        })
        .expect("psiphi should succeed");

        assert_eq!(outcome.profile.residue_count, 3);
        assert_eq!(outcome.profile.phi_count, 2);
        assert_eq!(outcome.profile.psi_count, 2);
        assert_eq!(outcome.profile.residues[1].residue_name, "ALA");
        assert!(
            (outcome.profile.residues[1]
                .phi_degrees
                .expect("phi should exist")
                - 143.70145061638846)
                .abs()
                < 1e-9
        );
    }

    #[test]
    fn rejects_coordinate_text_without_backbone_atoms() {
        let error = run_psiphi(PsiphiParams {
            input: PsiphiInput::new(fixture("psiphi_no_backbone.txt")),
        })
        .expect_err("missing backbone should fail");

        assert!(error.to_string().contains("requires PDB ATOM input"));
    }
}
