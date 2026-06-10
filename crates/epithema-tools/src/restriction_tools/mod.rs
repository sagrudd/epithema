//! Restriction-site design and recoding tool cohort.

mod recoder;
mod shared;
mod silent;

use crate::ToolDescriptor;

const FAMILY: &str = "restriction_tools";

pub use recoder::{RecoderCandidate, RecoderOutcome, RecoderParams, recoder_help, run_recoder};
pub use silent::{SilentCandidate, SilentOutcome, SilentParams, run_silent, silent_help};

/// `recoder` descriptor.
pub const RECODER_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "recoder",
    "report synonymous single-codon edits that remove exact forward-strand restriction sites",
)
.with_family(FAMILY);

/// `silent` descriptor.
pub const SILENT_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "silent",
    "report synonymous single-codon edits that create exact forward-strand restriction sites",
)
.with_family(FAMILY);

/// Implemented restriction-tool cohort descriptors in stable listing order.
pub const TOOL_DESCRIPTORS: &[ToolDescriptor] = &[RECODER_DESCRIPTOR, SILENT_DESCRIPTOR];
