//! Command-discovery and help-navigation bounded helpers.

mod wossname;

use crate::ToolDescriptor;

const FAMILY: &str = "command_tools";

pub use wossname::{run_wossname, wossname_help, WossnameOutcome, WossnameParams, WossnameRow};

/// `wossname` descriptor.
pub const WOSSNAME_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "wossname",
    "report deterministic local keyword matches against governed tool metadata",
)
.with_family(FAMILY);
