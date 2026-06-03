//! Primer and assay-oriented bounded search helpers.

mod primersearch;

use crate::ToolDescriptor;

const FAMILY: &str = "primer_tools";

pub use primersearch::{
    PrimersearchOutcome, PrimersearchPairInput, PrimersearchParams, PrimersearchRow,
    primersearch_help, run_primersearch,
};

/// `primersearch` descriptor.
pub const PRIMERSEARCH_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "primersearch",
    "report deterministic complete primer-pair hits against local nucleotide sequence inputs",
)
.with_family(FAMILY);
