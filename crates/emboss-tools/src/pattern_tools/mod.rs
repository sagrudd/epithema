//! Simple deterministic pattern-search tool cohort.

mod fuzznuc;
mod fuzzpro;
mod fuzztran;

use crate::ToolDescriptor;

pub use fuzznuc::{FuzznucOutcome, FuzznucParams, fuzznuc_help, run_fuzznuc};
pub use fuzzpro::{FuzzproOutcome, FuzzproParams, fuzzpro_help, run_fuzzpro};
pub use fuzztran::{FuzztranOutcome, FuzztranParams, fuzztran_help, run_fuzztran};

/// `fuzznuc` descriptor.
pub const FUZZNUC_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "fuzznuc",
    "scan nucleotide sequences for deterministic literal or IUPAC-ambiguous motifs",
);
/// `fuzzpro` descriptor.
pub const FUZZPRO_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "fuzzpro",
    "scan protein sequences for deterministic literal motifs with X wildcard support",
);
/// `fuzztran` descriptor.
pub const FUZZTRAN_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "fuzztran",
    "scan forward translated nucleotide frames for deterministic protein motifs",
);
