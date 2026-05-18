//! Composition and summary-statistics tool cohort.

mod aaindexextract;
mod complex;
mod compseq;
mod dan;
mod geecee;
mod infobase;
mod infoseq;
mod inforesidue;
mod oddcomp;
mod pepstats;
mod wordcount;

use crate::ToolDescriptor;

const FAMILY: &str = "sequence_stats";

pub use aaindexextract::{
    AaindexBuiltIn, AaindexextractOutcome, AaindexextractParams, aaindexextract_help,
    parse_aaindexextract_index, run_aaindexextract,
};
pub use complex::{ComplexOutcome, ComplexParams, complex_help, run_complex};
pub use compseq::{CompseqOutcome, CompseqParams, compseq_help, run_compseq};
pub use dan::{DanOutcome, DanParams, dan_help, run_dan};
pub use geecee::{GeeceeOutcome, GeeceeParams, geecee_help, run_geecee};
pub use infobase::{InfobaseOutcome, InfobaseParams, infobase_help, run_infobase};
pub use infoseq::{InfoseqOutcome, InfoseqParams, infoseq_help, run_infoseq};
pub use inforesidue::{InforesidueOutcome, InforesidueParams, inforesidue_help, run_inforesidue};
pub use oddcomp::{OddcompOutcome, OddcompParams, oddcomp_help, run_oddcomp};
pub use pepstats::{PepstatsOutcome, PepstatsParams, pepstats_help, run_pepstats};
pub use wordcount::{
    WordcountOutcome, WordcountParams, run_wordcount, word_frequency, wordcount_help,
};

/// `aaindexextract` descriptor.
pub const AAINDEXEXTRACT_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "aaindexextract",
    "report one governed built-in amino-acid property table",
)
.with_family(FAMILY);
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
/// `infobase` descriptor.
pub const INFOBASE_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "infobase",
    "report deterministic metadata for one nucleotide base or ambiguity symbol",
)
.with_family(FAMILY);
/// `infoseq` descriptor.
pub const INFOSEQ_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "infoseq",
    "report stable basic metadata and length summaries for sequence records",
)
.with_family(FAMILY);
/// `inforesidue` descriptor.
pub const INFORESIDUE_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "inforesidue",
    "report deterministic metadata for one canonical amino-acid residue",
)
.with_family(FAMILY);
/// `oddcomp` descriptor.
pub const ODDCOMP_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "oddcomp",
    "report deterministic exact protein word-composition counts for query words",
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
