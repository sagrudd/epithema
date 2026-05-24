//! Nucleotide plot-producing tools under staged rollout.

mod density;

use crate::ToolDescriptor;

const FAMILY: &str = "nucleotide_plots";

pub use density::{DensityOutcome, DensityParams, density_help, run_density};

/// `density` descriptor.
pub const DENSITY_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "density",
    "report a sliding-window nucleotide density profile and emit a line-plot contract",
)
.with_family(FAMILY);
