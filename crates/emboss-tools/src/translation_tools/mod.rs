//! Translation-adjacent tool cohort.

mod backtranambig;
mod backtranseq;
mod checktrans;

use crate::ToolDescriptor;

pub use backtranambig::{
    BacktranambigOutcome, BacktranambigParams, backtranambig_help, run_backtranambig,
};
pub use backtranseq::{BacktranseqOutcome, BacktranseqParams, backtranseq_help, run_backtranseq};
pub use checktrans::{ChecktransOutcome, ChecktransParams, checktrans_help, run_checktrans};

/// `backtranseq` descriptor.
pub const BACKTRANSEQ_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "backtranseq",
    "back-translate protein sequences to deterministic representative DNA codons",
);
/// `backtranambig` descriptor.
pub const BACKTRANAMBIG_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "backtranambig",
    "back-translate protein sequences to ambiguous DNA codon representations",
);
/// `checktrans` descriptor.
pub const CHECKTRANS_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "checktrans",
    "strictly compare frame-1 DNA translation against expected protein sequences",
);
