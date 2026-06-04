//! Primer and assay-oriented bounded search helpers.

mod eprimer3;
mod primersearch;

use crate::ToolDescriptor;

const FAMILY: &str = "primer_tools";

pub use primersearch::{
    PrimersearchOutcome, PrimersearchPairInput, PrimersearchParams, PrimersearchRow,
    primersearch_help, run_primersearch,
};
pub use eprimer3::{Eprimer3Outcome, Eprimer3Params, Eprimer3Row, eprimer3_help, run_eprimer3};

/// `eprimer3` descriptor.
pub const EPRIMER3_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "eprimer3",
    "report deterministic bounded primer-and-oligo design candidates against local nucleotide sequence inputs",
)
.with_family(FAMILY);

/// `primersearch` descriptor.
pub const PRIMERSEARCH_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "primersearch",
    "report deterministic complete primer-pair hits against local nucleotide sequence inputs",
)
.with_family(FAMILY);
