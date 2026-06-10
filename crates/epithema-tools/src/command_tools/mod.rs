//! Command-discovery and help-navigation bounded helpers.

mod seealso;
mod wossname;

use crate::ToolDescriptor;

const FAMILY: &str = "command_tools";

pub use seealso::{SeealsoOutcome, SeealsoParams, SeealsoRow, run_seealso, seealso_help};
pub use wossname::{WossnameOutcome, WossnameParams, WossnameRow, run_wossname, wossname_help};

/// `seealso` descriptor.
pub const SEEALSO_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "seealso",
    "report deterministic local related-program rows from governed tool metadata",
)
.with_family(FAMILY);

/// `wossname` descriptor.
pub const WOSSNAME_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "wossname",
    "report deterministic local keyword matches against governed tool metadata",
)
.with_family(FAMILY);
