//! Modernized user-facing retrieval and normalization tools.

mod refseqget;
mod seqret;

pub use refseqget::{RefseqgetOutcome, RefseqgetParams, refseqget_help, run_refseqget};
pub use seqret::{SeqretOutcome, SeqretParams, SeqretSource, run_seqret, seqret_help};

use crate::ToolDescriptor;

/// `seqret` descriptor.
pub const SEQRET_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "seqret",
    "normalize local sequence inputs or retrieve one accession-backed sequence",
);
/// `refseqget` descriptor.
pub const REFSEQGET_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "refseqget",
    "retrieve one provider-backed reference sequence through the governed acquisition seam",
);
