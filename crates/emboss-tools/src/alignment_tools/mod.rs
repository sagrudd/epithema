//! Alignment utility tool cohort.

mod aligncopy;
mod aligncopypair;
mod extractalign;
mod infoalign;
mod shared;

use crate::ToolDescriptor;

const FAMILY: &str = "alignment_tools";

pub use aligncopy::{AligncopyOutcome, AligncopyParams, aligncopy_help, run_aligncopy};
pub use aligncopypair::{
    AligncopypairOutcome, AligncopypairParams, aligncopypair_help, run_aligncopypair,
};
pub use extractalign::{
    ExtractalignOutcome, ExtractalignParams, extractalign_help, run_extractalign,
};
pub use infoalign::{InfoalignOutcome, InfoalignParams, infoalign_help, run_infoalign};
pub use shared::{AlignmentInput, AlignmentToolError, load_alignment};

/// `aligncopy` descriptor.
pub const ALIGNCOPY_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "aligncopy",
    "copy a single alignment unchanged through the shared alignment IO path",
)
.with_family(FAMILY);
/// `aligncopypair` descriptor.
pub const ALIGNCOPYPAIR_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "aligncopypair",
    "copy a single pairwise alignment unchanged and reject non-pairwise inputs",
)
.with_family(FAMILY);
/// `infoalign` descriptor.
pub const INFOALIGN_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "infoalign",
    "report row counts, column counts, and per-row gap statistics for an alignment",
)
.with_family(FAMILY);
/// `extractalign` descriptor.
pub const EXTRACTALIGN_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "extractalign",
    "extract rows and an optional 1-based inclusive column range from an alignment",
)
.with_family(FAMILY);
