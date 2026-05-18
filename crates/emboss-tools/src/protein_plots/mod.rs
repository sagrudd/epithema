//! Protein plot-producing tools.

mod charge;
mod pepwindow;

use crate::ToolDescriptor;

const FAMILY: &str = "protein_plots";

pub use charge::{ChargeOutcome, ChargeParams, charge_help, run_charge};
pub use pepwindow::{PepwindowOutcome, PepwindowParams, pepwindow_help, run_pepwindow};

/// `charge` descriptor.
pub const CHARGE_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "charge",
    "report a sliding-window protein charge profile and emit a line-plot contract",
)
.with_family(FAMILY);

/// `pepwindow` descriptor.
pub const PEPWINDOW_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "pepwindow",
    "report a sliding-window protein hydropathy profile and emit a line-plot contract",
)
.with_family(FAMILY);
