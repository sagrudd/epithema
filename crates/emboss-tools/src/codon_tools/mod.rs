//! Codon-usage and coding-bias tool cohort.

mod cai;
mod chips;
mod codcmp;
mod codcopy;
mod shared;

use crate::ToolDescriptor;

const FAMILY: &str = "codon_tools";

pub use cai::{CaiOutcome, CaiParams, cai_help, run_cai};
pub use chips::{ChipsOutcome, ChipsParams, chips_help, run_chips};
pub use codcmp::{CodcmpOutcome, CodcmpParams, codcmp_help, run_codcmp};
pub use codcopy::{CodcopyOutcome, CodcopyParams, codcopy_help, run_codcopy};
pub use shared::render_profile_rows;

/// `cai` descriptor.
pub const CAI_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "cai",
    "report deterministic codon adaptation index values against a reference profile",
)
.with_family(FAMILY);
/// `chips` descriptor.
pub const CHIPS_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "chips",
    "report per-record and aggregate codon usage counts and frequencies",
)
.with_family(FAMILY);
/// `codcmp` descriptor.
pub const CODCMP_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "codcmp",
    "compare codon usage between two coding-sequence or codon-profile sources",
)
.with_family(FAMILY);
/// `codcopy` descriptor.
pub const CODCOPY_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "codcopy",
    "normalize coding-sequence or codon-profile input into a reusable codon profile",
)
.with_family(FAMILY);
