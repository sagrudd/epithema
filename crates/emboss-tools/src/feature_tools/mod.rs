//! Feature-driven masking, extraction, and annotation-copy tool cohort.

mod extractfeat;
mod coderet;
mod featcopy;
mod featmerge;
mod featreport;
mod feattext;
mod maskfeat;
mod maskseq;
mod render;
mod shared;

use crate::ToolDescriptor;

const FAMILY: &str = "feature_tools";

pub use coderet::{CoderetOutcome, CoderetParams, coderet_help, run_coderet};
pub use extractfeat::{ExtractfeatOutcome, ExtractfeatParams, extractfeat_help, run_extractfeat};
pub use featcopy::{FeatcopyOutcome, FeatcopyParams, featcopy_help, run_featcopy};
pub use featmerge::{FeatmergeOutcome, FeatmergeParams, featmerge_help, run_featmerge};
pub use featreport::{FeatreportOutcome, FeatreportParams, featreport_help, run_featreport};
pub use feattext::{FeattextOutcome, FeattextParams, feattext_help, run_feattext};
pub use maskfeat::{MaskfeatOutcome, MaskfeatParams, maskfeat_help, run_maskfeat};
pub use maskseq::{MaskseqOutcome, MaskseqParams, maskseq_help, run_maskseq};

/// `maskseq` descriptor.
pub const MASKSEQ_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "maskseq",
    "mask explicit 1-based inclusive sequence intervals with a configurable symbol",
)
.with_family(FAMILY);
/// `maskfeat` descriptor.
pub const MASKFEAT_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "maskfeat",
    "mask selected simple feature spans while preserving annotations in the result payload",
)
.with_family(FAMILY);
/// `extractfeat` descriptor.
pub const EXTRACTFEAT_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "extractfeat",
    "extract selected simple feature spans into rebased sequence records",
)
.with_family(FAMILY);
/// `featcopy` descriptor.
pub const FEATCOPY_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "featcopy",
    "copy selected feature annotations between identifier-matched compatible records",
)
.with_family(FAMILY);
/// `coderet` descriptor.
pub const CODERET_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "coderet",
    "extract selected simple coding features and optionally translate them",
)
.with_family(FAMILY);
/// `featmerge` descriptor.
pub const FEATMERGE_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "featmerge",
    "merge selected feature annotations between identifier-matched annotated records",
)
.with_family(FAMILY);
/// `featreport` descriptor.
pub const FEATREPORT_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "featreport",
    "report selected features in a stable tabular summary",
)
.with_family(FAMILY);
/// `feattext` descriptor.
pub const FEATTEXT_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "feattext",
    "render selected features as normalized feature-table text",
)
.with_family(FAMILY);
