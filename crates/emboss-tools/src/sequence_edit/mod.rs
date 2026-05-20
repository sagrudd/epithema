//! Sequence cleanup and editing tool cohort.

mod biosed;
mod degapseq;
mod descseq;
mod msbar;
mod revseq;
mod shared;
mod trimest;
mod trimseq;
mod vectorstrip;

use crate::ToolDescriptor;

const FAMILY: &str = "sequence_edit";

pub use biosed::{BiosedOutcome, BiosedParams, biosed_help, run_biosed};
pub use degapseq::{DegapseqOutcome, DegapseqParams, degapseq_help, run_degapseq};
pub use descseq::{DescseqOutcome, DescseqParams, descseq_help, run_descseq};
pub use msbar::{MsbarMutation, MsbarOutcome, MsbarParams, msbar_help, run_msbar};
pub use revseq::{RevseqOutcome, RevseqParams, revseq_help, run_revseq};
pub use trimest::{TrimestOutcome, TrimestParams, run_trimest, trimest_help};
pub use trimseq::{TrimseqOutcome, TrimseqParams, run_trimseq, trimseq_help};
pub use vectorstrip::{VectorstripOutcome, VectorstripParams, run_vectorstrip, vectorstrip_help};

/// `biosed` descriptor.
pub const BIOSED_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "biosed",
    "replace or delete explicit sequence intervals record by record",
)
.with_family(FAMILY);

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
/// `trimest` descriptor.
pub const TRIMEST_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "trimest",
    "remove terminal poly-A tails from nucleotide sequence records",
)
.with_family(FAMILY);
/// `descseq` descriptor.
pub const DESCSEQ_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "descseq",
    "report stable sequence-record descriptions and metadata",
)
.with_family(FAMILY);
/// `msbar` descriptor.
pub const MSBAR_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "msbar",
    "apply explicit point mutations to sequence records",
)
.with_family(FAMILY);
/// `vectorstrip` descriptor.
pub const VECTORSTRIP_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "vectorstrip",
    "strip exact vector sequences from the ends of nucleotide records",
)
.with_family(FAMILY);
