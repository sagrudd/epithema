//! Global pairwise-alignment tool cohort.

mod needle;
mod needleall;
mod shared;
mod water;

use crate::ToolDescriptor;

const FAMILY: &str = "pairwise_alignment";

pub use needle::{NeedleOutcome, NeedleParams, needle_help, run_needle};
pub use needleall::{
    NeedleallCase, NeedleallOutcome, NeedleallParams, needleall_help, run_needleall,
};
pub use water::{WaterOutcome, WaterParams, run_water, water_help};

/// `needle` descriptor.
pub const NEEDLE_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "needle",
    "perform deterministic global pairwise alignment between exactly one query and one target",
)
.with_family(FAMILY);
/// `needleall` descriptor.
pub const NEEDLEALL_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "needleall",
    "perform deterministic many-vs-many global pairwise alignment and report comparison summaries",
)
.with_family(FAMILY);
/// `water` descriptor.
pub const WATER_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "water",
    "perform deterministic local pairwise alignment between exactly one query and one target",
)
.with_family(FAMILY);
