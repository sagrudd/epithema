//! Global pairwise-alignment tool cohort.

mod needle;
mod needleall;
mod shared;

use crate::ToolDescriptor;

pub use needle::{NeedleOutcome, NeedleParams, needle_help, run_needle};
pub use needleall::{
    NeedleallCase, NeedleallOutcome, NeedleallParams, needleall_help, run_needleall,
};

/// `needle` descriptor.
pub const NEEDLE_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "needle",
    "perform deterministic global pairwise alignment between exactly one query and one target",
);
/// `needleall` descriptor.
pub const NEEDLEALL_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "needleall",
    "perform deterministic many-vs-many global pairwise alignment and report comparison summaries",
);
