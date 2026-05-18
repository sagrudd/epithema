//! Simple deterministic pattern-search tool cohort.

mod exact_words;
mod fuzznuc;
mod fuzzpro;
mod fuzztran;
mod patmatdb;
mod preg;
mod protein_regex;
mod wordfinder;
mod wordmatch;

use crate::ToolDescriptor;

const FAMILY: &str = "pattern_tools";

pub use fuzznuc::{FuzznucOutcome, FuzznucParams, fuzznuc_help, run_fuzznuc};
pub use fuzzpro::{FuzzproOutcome, FuzzproParams, fuzzpro_help, run_fuzzpro};
pub use fuzztran::{FuzztranOutcome, FuzztranParams, fuzztran_help, run_fuzztran};
pub use patmatdb::{PatmatdbOutcome, PatmatdbParams, patmatdb_help, run_patmatdb};
pub use preg::{PregOutcome, PregParams, preg_help, run_preg};
pub use protein_regex::CompiledProteinRegex;
pub use wordfinder::{WordfinderOutcome, WordfinderParams, run_wordfinder, wordfinder_help};
pub use wordmatch::{WordmatchOutcome, WordmatchParams, run_wordmatch, wordmatch_help};

/// `fuzznuc` descriptor.
pub const FUZZNUC_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "fuzznuc",
    "scan nucleotide sequences for deterministic literal or IUPAC-ambiguous motifs",
)
.with_family(FAMILY);
/// `fuzzpro` descriptor.
pub const FUZZPRO_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "fuzzpro",
    "scan protein sequences for deterministic literal motifs with X wildcard support",
)
.with_family(FAMILY);
/// `fuzztran` descriptor.
pub const FUZZTRAN_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "fuzztran",
    "scan forward translated nucleotide frames for deterministic protein motifs",
)
.with_family(FAMILY);
/// `preg` descriptor.
pub const PREG_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "preg",
    "scan protein sequences with deterministic bounded regular expressions",
)
.with_family(FAMILY);
/// `patmatdb` descriptor.
pub const PATMATDB_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "patmatdb",
    "scan protein sequences with a local deterministic motif database",
)
.with_family(FAMILY);
/// `wordmatch` descriptor.
pub const WORDMATCH_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "wordmatch",
    "report maximal exact shared regions between two singleton sequences",
)
.with_family(FAMILY);
/// `wordfinder` descriptor.
pub const WORDFINDER_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "wordfinder",
    "report maximal exact shared regions between one query and multiple targets",
)
.with_family(FAMILY);
