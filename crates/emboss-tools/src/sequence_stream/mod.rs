//! Sequence-stream and sequence-selection tool cohort.
//!
//! This family provides a first real shipped slice for EMBOSS-RS:
//! `newseq`, `seqcount`, `notseq`, `nthseq`, `skipseq`, and deterministic set operations.

mod generation;
mod listor;
mod makenucseq;
mod makeprotseq;
mod newseq;
mod notseq;
mod nthseq;
mod seqcount;
mod set_ops;
mod shared;
mod skipredundant;
mod skipseq;

use crate::ToolDescriptor;

const FAMILY: &str = "sequence_stream";

pub use listor::{ListorOutcome, ListorParams, listor_help, run_listor};
pub use makenucseq::{MakenucseqOutcome, MakenucseqParams, makenucseq_help, run_makenucseq};
pub use makeprotseq::{MakeprotseqOutcome, MakeprotseqParams, makeprotseq_help, run_makeprotseq};
pub use newseq::{NewseqOutcome, NewseqParams, newseq_help, run_newseq};
pub use notseq::{NotseqOutcome, NotseqParams, notseq_help, run_notseq};
pub use nthseq::{NthseqOutcome, NthseqParams, nthseq_help, run_nthseq};
pub use seqcount::{SeqcountOutcome, SeqcountParams, run_seqcount, seqcount_help};
pub use set_ops::SequenceSetOperator;
pub use shared::{SequenceInput, ToolExecutionError, load_sequence_records};
pub use skipredundant::{
    SkipredundantOutcome, SkipredundantParams, run_skipredundant, skipredundant_help,
};
pub use skipseq::{SkipseqOutcome, SkipseqParams, run_skipseq, skipseq_help};

/// `newseq` descriptor.
pub const NEWSEQ_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "newseq",
    "create a new sequence record from supplied residues",
)
.with_family(FAMILY);
/// `makenucseq` descriptor.
pub const MAKENUCSEQ_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "makenucseq",
    "create deterministic nucleotide sequence records from a bounded random generator",
)
.with_family(FAMILY);
/// `makeprotseq` descriptor.
pub const MAKEPROTSEQ_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "makeprotseq",
    "create deterministic protein sequence records from a bounded random generator",
)
.with_family(FAMILY);
/// `seqcount` descriptor.
pub const SEQCOUNT_DESCRIPTOR: ToolDescriptor =
    ToolDescriptor::new("seqcount", "count sequence records in an input stream")
        .with_family(FAMILY);
/// `notseq` descriptor.
pub const NOTSEQ_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "notseq",
    "return all sequence records except the excluded index",
)
.with_family(FAMILY);
/// `nthseq` descriptor.
pub const NTHSEQ_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "nthseq",
    "select the 1-based Nth sequence record from an input set",
)
.with_family(FAMILY);
/// `skipseq` descriptor.
pub const SKIPSEQ_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "skipseq",
    "skip the first N sequence records and return the rest",
)
.with_family(FAMILY);
/// `listor` descriptor.
pub const LISTOR_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "listor",
    "combine two sequence sets with deterministic logical set operators over exact sequence identity",
)
.with_family(FAMILY);
/// `skipredundant` descriptor.
pub const SKIPREDUNDANT_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "skipredundant",
    "remove exact redundant sequences from one input while preserving first-seen order",
)
.with_family(FAMILY);

/// Implemented sequence-stream cohort descriptors in stable listing order.
pub const TOOL_DESCRIPTORS: &[ToolDescriptor] = &[
    NEWSEQ_DESCRIPTOR,
    MAKENUCSEQ_DESCRIPTOR,
    MAKEPROTSEQ_DESCRIPTOR,
    SEQCOUNT_DESCRIPTOR,
    NOTSEQ_DESCRIPTOR,
    NTHSEQ_DESCRIPTOR,
    SKIPSEQ_DESCRIPTOR,
    LISTOR_DESCRIPTOR,
    SKIPREDUNDANT_DESCRIPTOR,
];
