//! Composition and summary-statistics tool cohort.

mod compseq;
mod geecee;
mod pepstats;

use crate::ToolDescriptor;

pub use compseq::{CompseqOutcome, CompseqParams, compseq_help, run_compseq};
pub use geecee::{GeeceeOutcome, GeeceeParams, geecee_help, run_geecee};
pub use pepstats::{PepstatsOutcome, PepstatsParams, pepstats_help, run_pepstats};

/// `compseq` descriptor.
pub const COMPSEQ_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "compseq",
    "report per-record and aggregate residue composition counts and frequencies",
);
/// `geecee` descriptor.
pub const GEECEE_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "geecee",
    "report per-record and aggregate GC statistics for nucleotide sequences",
);
/// `pepstats` descriptor.
pub const PEPSTATS_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "pepstats",
    "report basic protein composition, length, and molecular-weight statistics",
);
