//! Sequence extraction and partitioning tool cohort.

mod cutseq;
mod extractseq;
mod splitter;
mod union;

use crate::ToolDescriptor;

pub use cutseq::{CutseqOutcome, CutseqParams, cutseq_help, run_cutseq};
pub use extractseq::{ExtractseqOutcome, ExtractseqParams, extractseq_help, run_extractseq};
pub use splitter::{SplitterOutcome, SplitterParams, run_splitter, splitter_help};
pub use union::{UnionOutcome, UnionParams, run_union, union_help};

/// `extractseq` descriptor.
pub const EXTRACTSEQ_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "extractseq",
    "extract a 1-based inclusive region from each input sequence record",
);
/// `cutseq` descriptor.
pub const CUTSEQ_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "cutseq",
    "cut each input sequence record into left and right fragments at a position",
);
/// `union` descriptor.
pub const UNION_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "union",
    "concatenate multiple sequence inputs into one output stream",
);
/// `splitter` descriptor.
pub const SPLITTER_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "splitter",
    "partition an input sequence stream into deterministic fixed-size chunks",
);
