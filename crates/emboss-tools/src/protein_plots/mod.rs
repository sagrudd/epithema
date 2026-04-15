//! Protein plot-producing tools.

mod charge;

use crate::ToolDescriptor;

pub use charge::{ChargeOutcome, ChargeParams, charge_help, run_charge};

/// `charge` descriptor.
pub const CHARGE_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "charge",
    "report a sliding-window protein charge profile and emit a line-plot contract",
);
