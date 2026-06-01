//! Protein-coordinate analytical helpers that remain local to bounded structural methods.

mod psiphi;

pub use psiphi::{psiphi_help, run_psiphi, PsiphiInput, PsiphiOutcome, PsiphiParams};

use crate::ToolDescriptor;

const FAMILY: &str = "protein_coordinates";

/// `psiphi` descriptor.
pub const PSIPHI_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "psiphi",
    "report deterministic per-residue phi/psi torsion angles from bounded protein backbone coordinates",
)
.with_family(FAMILY);
