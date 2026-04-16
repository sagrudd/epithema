//! Sequence cleanup and editing tool cohort.

mod degapseq;
mod descseq;
mod revseq;
mod trimseq;

use crate::ToolDescriptor;

const FAMILY: &str = "sequence_edit";

pub use degapseq::{DegapseqOutcome, DegapseqParams, degapseq_help, run_degapseq};
pub use descseq::{DescseqOutcome, DescseqParams, descseq_help, run_descseq};
pub use revseq::{RevseqOutcome, RevseqParams, revseq_help, run_revseq};
pub use trimseq::{TrimseqOutcome, TrimseqParams, run_trimseq, trimseq_help};

/// `degapseq` descriptor.
pub const DEGAPSEQ_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "degapseq",
    "remove '-' and '.' gap characters from sequence records",
)
.with_family(FAMILY);
/// `revseq` descriptor.
pub const REVSEQ_DESCRIPTOR: ToolDescriptor =
    ToolDescriptor::new("revseq", "reverse sequence content record by record").with_family(FAMILY);
/// `trimseq` descriptor.
pub const TRIMSEQ_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "trimseq",
    "trim explicit residue counts from the left and right ends of sequence records",
)
.with_family(FAMILY);
/// `descseq` descriptor.
pub const DESCSEQ_DESCRIPTOR: ToolDescriptor =
    ToolDescriptor::new("descseq", "replace or clear sequence record descriptions")
        .with_family(FAMILY);
