//! Modernized user-facing retrieval and normalization tools.

mod refseqget;
mod seqret;
mod seqretsetall;

pub use refseqget::{RefseqgetOutcome, RefseqgetParams, refseqget_help, run_refseqget};
pub use seqret::{SeqretOutcome, SeqretParams, SeqretSource, run_seqret, seqret_help};
pub use seqretsetall::{
    SeqretsetallInputSet, SeqretsetallOutcome, SeqretsetallParams, run_seqretsetall,
    seqretsetall_help,
};

use crate::ToolDescriptor;

const FAMILY: &str = "retrieval_tools";

/// `seqret` descriptor.
pub const SEQRET_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "seqret",
    "normalize local sequence inputs or retrieve one accession-backed sequence",
)
.with_family(FAMILY);
/// `seqretsetall` descriptor.
pub const SEQRETSETALL_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "seqretsetall",
    "normalize multiple local or provider-backed sequence inputs into ordered output record sets",
)
.with_family(FAMILY);
/// `refseqget` descriptor.
pub const REFSEQGET_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "refseqget",
    "retrieve one provider-backed reference sequence through the governed acquisition seam",
)
.with_family(FAMILY);
