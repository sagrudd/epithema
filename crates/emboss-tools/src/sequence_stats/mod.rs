//! Composition and summary-statistics tool cohort.

mod complex;
mod compseq;
mod dan;
mod geecee;
mod infoseq;
mod pepstats;
mod wordcount;

use crate::ToolDescriptor;

const FAMILY: &str = "sequence_stats";

pub use complex::{ComplexOutcome, ComplexParams, complex_help, run_complex};
pub use compseq::{CompseqOutcome, CompseqParams, compseq_help, run_compseq};
pub use dan::{DanOutcome, DanParams, dan_help, run_dan};
pub use geecee::{GeeceeOutcome, GeeceeParams, geecee_help, run_geecee};
pub use infoseq::{InfoseqOutcome, InfoseqParams, infoseq_help, run_infoseq};
pub use pepstats::{PepstatsOutcome, PepstatsParams, pepstats_help, run_pepstats};
pub use wordcount::{
    WordcountOutcome, WordcountParams, run_wordcount, word_frequency, wordcount_help,
};

/// `complex` descriptor.
pub const COMPLEX_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "complex",
    "report whole-sequence and sliding-window nucleotide linguistic complexity",
)
.with_family(FAMILY);
/// `compseq` descriptor.
pub const COMPSEQ_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "compseq",
    "report per-record and aggregate residue composition counts and frequencies",
)
.with_family(FAMILY);
/// `dan` descriptor.
pub const DAN_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "dan",
    "report conservative whole-sequence or sliding-window melting estimates",
)
.with_family(FAMILY);
/// `geecee` descriptor.
pub const GEECEE_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "geecee",
    "report per-record and aggregate GC statistics for nucleotide sequences",
)
.with_family(FAMILY);
/// `infoseq` descriptor.
pub const INFOSEQ_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "infoseq",
    "report stable basic metadata and length summaries for sequence records",
)
.with_family(FAMILY);
/// `pepstats` descriptor.
pub const PEPSTATS_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "pepstats",
    "report basic protein composition, length, and molecular-weight statistics",
)
.with_family(FAMILY);
/// `wordcount` descriptor.
pub const WORDCOUNT_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "wordcount",
    "count overlapping normalized sequence words with stable per-record and aggregate reporting",
)
.with_family(FAMILY);
