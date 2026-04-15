//! Modernized user-facing archive metadata and public-run acquisition tools.

mod runget;
mod runinfo;

pub use runget::{RungetOutcome, RungetParams, run_runget, runget_help};
pub use runinfo::{RuninfoOutcome, RuninfoParams, run_runinfo, runinfo_help};

use crate::ToolDescriptor;

/// `runinfo` descriptor.
pub const RUNINFO_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "runinfo",
    "normalize ENA or SRA archive metadata for one accession-backed archive object",
);

/// `runget` descriptor.
pub const RUNGET_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "runget",
    "discover a normalized public-run manifest through the governed archive acquisition seam",
);
