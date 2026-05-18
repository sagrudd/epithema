//! Translation-adjacent tool cohort.

mod backtranambig;
mod backtranseq;
mod checktrans;
mod getorf;
mod prettyseq;
mod shared;
mod tranalign;
mod transeq;

use crate::ToolDescriptor;

const FAMILY: &str = "translation_tools";

pub use backtranambig::{
    BacktranambigOutcome, BacktranambigParams, backtranambig_help, run_backtranambig,
};
pub use backtranseq::{BacktranseqOutcome, BacktranseqParams, backtranseq_help, run_backtranseq};
pub use getorf::{GetorfOutcome, GetorfParams, getorf_help, run_getorf};
pub use prettyseq::{PrettyseqOutcome, PrettyseqParams, prettyseq_help, run_prettyseq};
pub use checktrans::{ChecktransOutcome, ChecktransParams, checktrans_help, run_checktrans};
pub use shared::TranslationFrameSelection;
pub use tranalign::{TranalignOutcome, TranalignParams, tranalign_help, run_tranalign};
pub use transeq::{TranseqOutcome, TranseqParams, transeq_help, run_transeq};

/// `backtranseq` descriptor.
pub const BACKTRANSEQ_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "backtranseq",
    "back-translate protein sequences to deterministic representative DNA codons",
)
.with_family(FAMILY);
/// `backtranambig` descriptor.
pub const BACKTRANAMBIG_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "backtranambig",
    "back-translate protein sequences to ambiguous DNA codon representations",
)
.with_family(FAMILY);
/// `checktrans` descriptor.
pub const CHECKTRANS_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "checktrans",
    "strictly compare frame-1 DNA translation against expected protein sequences",
)
.with_family(FAMILY);
/// `transeq` descriptor.
pub const TRANSEQ_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "transeq",
    "translate nucleotide sequences in forward reading frames",
)
.with_family(FAMILY);
/// `getorf` descriptor.
pub const GETORF_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "getorf",
    "extract stop-bounded forward ORFs from nucleotide sequences",
)
.with_family(FAMILY);
/// `prettyseq` descriptor.
pub const PRETTYSEQ_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "prettyseq",
    "render nucleotide sequences with a translated frame-oriented text view",
)
.with_family(FAMILY);
/// `tranalign` descriptor.
pub const TRANALIGN_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "tranalign",
    "project aligned proteins onto matching coding-sequence codon alignments",
)
.with_family(FAMILY);
