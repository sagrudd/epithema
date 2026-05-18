//! Sequence extraction and partitioning tool cohort.

mod cutseq;
mod extractseq;
mod megamerger;
mod merger;
mod pasteseq;
mod shared;
mod shuffleseq;
mod sizeseq;
mod splitter;
mod union;

use crate::ToolDescriptor;

const FAMILY: &str = "sequence_transform";

pub use cutseq::{CutseqOutcome, CutseqParams, cutseq_help, run_cutseq};
pub use extractseq::{ExtractseqOutcome, ExtractseqParams, extractseq_help, run_extractseq};
pub use megamerger::{MegamergerOutcome, MegamergerParams, megamerger_help, run_megamerger};
pub use merger::{MergerOutcome, MergerParams, merger_help, run_merger};
pub use pasteseq::{PasteseqOutcome, PasteseqParams, pasteseq_help, run_pasteseq};
pub use shuffleseq::{ShuffleseqOutcome, ShuffleseqParams, run_shuffleseq, shuffleseq_help};
pub use sizeseq::{SizeseqOutcome, SizeseqParams, run_sizeseq, sizeseq_help};
pub use splitter::{SplitterOutcome, SplitterParams, run_splitter, splitter_help};
pub use union::{UnionOutcome, UnionParams, run_union, union_help};

/// `extractseq` descriptor.
pub const EXTRACTSEQ_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "extractseq",
    "extract a 1-based inclusive region from each input sequence record",
)
.with_family(FAMILY);
/// `cutseq` descriptor.
pub const CUTSEQ_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "cutseq",
    "cut each input sequence record into left and right fragments at a position",
)
.with_family(FAMILY);
/// `union` descriptor.
pub const UNION_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "union",
    "concatenate multiple sequence inputs into one output stream",
)
.with_family(FAMILY);
/// `pasteseq` descriptor.
pub const PASTESEQ_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "pasteseq",
    "insert one sequence into another at a deterministic 1-based position",
)
.with_family(FAMILY);
/// `splitter` descriptor.
pub const SPLITTER_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "splitter",
    "partition an input sequence stream into deterministic fixed-size chunks",
)
.with_family(FAMILY);
/// `merger` descriptor.
pub const MERGER_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "merger",
    "merge two overlapping sequences by longest exact suffix/prefix overlap",
)
.with_family(FAMILY);
/// `megamerger` descriptor.
pub const MEGAMERGER_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "megamerger",
    "merge two overlapping DNA sequences by longest exact suffix/prefix overlap",
)
.with_family(FAMILY);
/// `sizeseq` descriptor.
pub const SIZESEQ_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "sizeseq",
    "sort sequence records by size in deterministic order",
)
.with_family(FAMILY);
/// `shuffleseq` descriptor.
pub const SHUFFLESEQ_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "shuffleseq",
    "shuffle sequence residues deterministically while preserving composition",
)
.with_family(FAMILY);
