//! Sequence-stream and sequence-selection tool cohort.
//!
//! This family provides a first real shipped slice for EMBOSS-RS:
//! `newseq`, `seqcount`, `notseq`, `nthseq`, and `skipseq`.

mod newseq;
mod notseq;
mod nthseq;
mod seqcount;
mod shared;
mod skipseq;

use crate::ToolDescriptor;

pub use newseq::{NewseqOutcome, NewseqParams, newseq_help, run_newseq};
pub use notseq::{NotseqOutcome, NotseqParams, notseq_help, run_notseq};
pub use nthseq::{NthseqOutcome, NthseqParams, nthseq_help, run_nthseq};
pub use seqcount::{SeqcountOutcome, SeqcountParams, run_seqcount, seqcount_help};
pub use shared::{SequenceInput, ToolExecutionError, load_sequence_records};
pub use skipseq::{SkipseqOutcome, SkipseqParams, run_skipseq, skipseq_help};

/// `newseq` descriptor.
pub const NEWSEQ_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "newseq",
    "create a new sequence record from supplied residues",
);
/// `seqcount` descriptor.
pub const SEQCOUNT_DESCRIPTOR: ToolDescriptor =
    ToolDescriptor::new("seqcount", "count sequence records in an input stream");
/// `notseq` descriptor.
pub const NOTSEQ_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "notseq",
    "return all sequence records except the excluded index",
);
/// `nthseq` descriptor.
pub const NTHSEQ_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "nthseq",
    "select the 1-based Nth sequence record from an input set",
);
/// `skipseq` descriptor.
pub const SKIPSEQ_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "skipseq",
    "skip the first N sequence records and return the rest",
);

/// Implemented sequence-stream cohort descriptors in stable listing order.
pub const TOOL_DESCRIPTORS: &[ToolDescriptor] = &[
    NEWSEQ_DESCRIPTOR,
    SEQCOUNT_DESCRIPTOR,
    NOTSEQ_DESCRIPTOR,
    NTHSEQ_DESCRIPTOR,
    SKIPSEQ_DESCRIPTOR,
];
