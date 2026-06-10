//! Modernized user-facing archive metadata and public-run acquisition tools.

mod assemblyget;
mod infoassembly;
mod runget;
mod runinfo;

pub use assemblyget::{
    ASSEMBLYGET_REPORT_COLUMNS, AssemblygetMaterializationStatus, AssemblygetOutcome,
    AssemblygetParams, assemblyget_help, run_assemblyget,
};
pub use infoassembly::{
    InfoassemblyOutcome, InfoassemblyParams, infoassembly_help, run_infoassembly,
};
pub use runget::{RungetOutcome, RungetParams, run_runget, runget_help};
pub use runinfo::{RuninfoOutcome, RuninfoParams, run_runinfo, runinfo_help};

use crate::ToolDescriptor;

const FAMILY: &str = "archive_tools";

/// `infoassembly` descriptor.
pub const INFOASSEMBLY_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "infoassembly",
    "normalize provider-backed archive metadata into a bounded assembly-first report",
)
.with_family(FAMILY);

/// `assemblyget` descriptor.
pub const ASSEMBLYGET_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "assemblyget",
    "report bounded assembly-level manifest intent for one provider-qualified archive accession",
)
.with_family(FAMILY);

/// `runinfo` descriptor.
pub const RUNINFO_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "runinfo",
    "normalize ENA or SRA archive metadata for one accession-backed archive object",
)
.with_family(FAMILY);

/// `runget` descriptor.
pub const RUNGET_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "runget",
    "discover a normalized public-run manifest through the governed archive acquisition seam",
)
.with_family(FAMILY);
