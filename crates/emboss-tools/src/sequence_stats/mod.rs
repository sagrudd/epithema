//! Composition and summary-statistics tool cohort.

mod complex;
mod compseq;
mod geecee;
mod pepstats;

use crate::ToolDescriptor;

const FAMILY: &str = "sequence_stats";

pub use complex::{ComplexOutcome, ComplexParams, complex_help, run_complex};
pub use compseq::{CompseqOutcome, CompseqParams, compseq_help, run_compseq};
pub use geecee::{GeeceeOutcome, GeeceeParams, geecee_help, run_geecee};
pub use pepstats::{PepstatsOutcome, PepstatsParams, pepstats_help, run_pepstats};

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
/// `geecee` descriptor.
pub const GEECEE_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "geecee",
    "report per-record and aggregate GC statistics for nucleotide sequences",
)
.with_family(FAMILY);
/// `pepstats` descriptor.
pub const PEPSTATS_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "pepstats",
    "report basic protein composition, length, and molecular-weight statistics",
)
.with_family(FAMILY);
