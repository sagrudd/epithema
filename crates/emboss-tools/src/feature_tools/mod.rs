//! Feature-driven masking, extraction, and annotation-copy tool cohort.

mod extractfeat;
mod featcopy;
mod maskfeat;
mod maskseq;
mod shared;

use crate::ToolDescriptor;

pub use extractfeat::{ExtractfeatOutcome, ExtractfeatParams, extractfeat_help, run_extractfeat};
pub use featcopy::{FeatcopyOutcome, FeatcopyParams, featcopy_help, run_featcopy};
pub use maskfeat::{MaskfeatOutcome, MaskfeatParams, maskfeat_help, run_maskfeat};
pub use maskseq::{MaskseqOutcome, MaskseqParams, maskseq_help, run_maskseq};

/// `maskseq` descriptor.
pub const MASKSEQ_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "maskseq",
    "mask explicit 1-based inclusive sequence intervals with a configurable symbol",
);
/// `maskfeat` descriptor.
pub const MASKFEAT_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "maskfeat",
    "mask selected simple feature spans while preserving annotations in the result payload",
);
/// `extractfeat` descriptor.
pub const EXTRACTFEAT_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "extractfeat",
    "extract selected simple feature spans into rebased sequence records",
);
/// `featcopy` descriptor.
pub const FEATCOPY_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "featcopy",
    "copy selected feature annotations between identifier-matched compatible records",
);
