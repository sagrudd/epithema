//! Alignment-summary, distance, and consensus tool implementations.

mod cons;
mod consambig;
mod distmat;
mod matcher;
mod shared;

pub use cons::{ConsOutcome, ConsParams, cons_help, run_cons};
pub use consambig::{ConsambigOutcome, ConsambigParams, consambig_help, run_consambig};
pub use distmat::{DistmatOutcome, DistmatParams, distmat_help, run_distmat};
pub use matcher::{MatcherOutcome, MatcherParams, matcher_help, run_matcher};

use crate::ToolDescriptor;

const FAMILY: &str = "alignment_analysis";

/// `matcher` descriptor.
pub const MATCHER_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "matcher",
    "deterministic ungapped pairwise similarity summary",
)
.with_family(FAMILY);
/// `distmat` descriptor.
pub const DISTMAT_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "distmat",
    "pairwise p-distance matrix for equal-length sequence sets",
)
.with_family(FAMILY);
/// `cons` descriptor.
pub const CONS_DESCRIPTOR: ToolDescriptor =
    ToolDescriptor::new("cons", "simple majority consensus from an alignment").with_family(FAMILY);
/// `consambig` descriptor.
pub const CONSAMBIG_DESCRIPTOR: ToolDescriptor =
    ToolDescriptor::new("consambig", "ambiguity-aware consensus from an alignment")
        .with_family(FAMILY);
