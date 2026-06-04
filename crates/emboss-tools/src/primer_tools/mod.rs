//! Primer and assay-oriented bounded search helpers.

mod eprimer3;
mod primersearch;
mod sirna;

use crate::ToolDescriptor;

const FAMILY: &str = "primer_tools";

pub use primersearch::{
    PrimersearchOutcome, PrimersearchPairInput, PrimersearchParams, PrimersearchRow,
    primersearch_help, run_primersearch,
};
pub use eprimer3::{Eprimer3Outcome, Eprimer3Params, Eprimer3Row, eprimer3_help, run_eprimer3};
pub use sirna::{SirnaOutcome, SirnaParams, SirnaRow, run_sirna, sirna_help};

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

/// `sirna` descriptor.
pub const SIRNA_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "sirna",
    "report deterministic bounded sirna candidate rows against local nucleotide sequence inputs",
)
.with_family(FAMILY);
