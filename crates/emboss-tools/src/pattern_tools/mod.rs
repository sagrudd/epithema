//! Simple deterministic pattern-search tool cohort.

mod dreg;
mod exact_words;
mod einverted;
mod fuzznuc;
mod fuzzpro;
mod fuzztran;
mod inverted_repeats;
mod nucleotide_regex;
mod palindrome;
mod patmatdb;
mod preg;
mod protein_regex;
mod seqmatchall;
mod wordfinder;
mod wordmatch;

use crate::ToolDescriptor;

const FAMILY: &str = "pattern_tools";

pub use dreg::{DregOutcome, DregParams, dreg_help, run_dreg};
pub use einverted::{EinvertedOutcome, EinvertedParams, einverted_help, run_einverted};
pub use fuzznuc::{FuzznucOutcome, FuzznucParams, fuzznuc_help, run_fuzznuc};
pub use fuzzpro::{FuzzproOutcome, FuzzproParams, fuzzpro_help, run_fuzzpro};
pub use fuzztran::{FuzztranOutcome, FuzztranParams, fuzztran_help, run_fuzztran};
pub use nucleotide_regex::CompiledNucleotideRegex;
pub use palindrome::{PalindromeOutcome, PalindromeParams, palindrome_help, run_palindrome};
pub use patmatdb::{PatmatdbOutcome, PatmatdbParams, patmatdb_help, run_patmatdb};
pub use preg::{PregOutcome, PregParams, preg_help, run_preg};
pub use protein_regex::CompiledProteinRegex;
pub use seqmatchall::{SeqmatchallOutcome, SeqmatchallParams, run_seqmatchall, seqmatchall_help};
pub use wordfinder::{WordfinderOutcome, WordfinderParams, run_wordfinder, wordfinder_help};
pub use wordmatch::{WordmatchOutcome, WordmatchParams, run_wordmatch, wordmatch_help};

/// `dreg` descriptor.
pub const DREG_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "dreg",
    "scan nucleotide sequences with deterministic bounded regular expressions",
)
.with_family(FAMILY);
/// `einverted` descriptor.
pub const EINVERTED_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "einverted",
    "report exact inverted-repeat arms with bounded spacer length in nucleotide sequences",
)
.with_family(FAMILY);
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
/// `palindrome` descriptor.
pub const PALINDROME_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "palindrome",
    "report exact reverse-complement palindromic regions in nucleotide sequences",
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
/// `seqmatchall` descriptor.
pub const SEQMATCHALL_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "seqmatchall",
    "report all-against-all maximal exact shared regions across a sequence set",
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
