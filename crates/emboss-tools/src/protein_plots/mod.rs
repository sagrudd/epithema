//! Protein plot-producing tools.

mod charge;
mod hmoment;
mod octanol;
mod pepinfo;
mod pepwindow;

use crate::ToolDescriptor;

const FAMILY: &str = "protein_plots";

pub use charge::{ChargeOutcome, ChargeParams, charge_help, run_charge};
pub use hmoment::{HmomentOutcome, HmomentParams, hmoment_help, run_hmoment};
pub use octanol::{OctanolOutcome, OctanolParams, octanol_help, run_octanol};
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

/// `hmoment` descriptor.
pub const HMOMENT_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "hmoment",
    "report a sliding-window protein hydrophobic-moment profile and emit a line-plot contract",
)
.with_family(FAMILY);

/// `octanol` descriptor.
pub const OCTANOL_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "octanol",
    "report a sliding-window White-Wimley interface-minus-octanol profile and emit a line-plot contract",
)
.with_family(FAMILY);
