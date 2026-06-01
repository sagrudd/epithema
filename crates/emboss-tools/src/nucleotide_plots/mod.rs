//! Nucleotide plot-producing tools under staged rollout.

mod density;
mod isochore;
mod wobble;

use crate::ToolDescriptor;

const FAMILY: &str = "nucleotide_plots";

pub use density::{DensityOutcome, DensityParams, density_help, run_density};
pub use isochore::{IsochoreOutcome, IsochoreParams, isochore_help, run_isochore};
pub use wobble::{WobbleOutcome, WobbleParams, run_wobble, wobble_help};

/// `density` descriptor.
pub const DENSITY_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "density",
    "report a sliding-window nucleotide density profile and emit a line-plot contract",
)
.with_family(FAMILY);

/// `wobble` descriptor.
pub const WOBBLE_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "wobble",
    "report a bounded third-base-position variability profile and emit a line-plot contract",
)
.with_family(FAMILY);

/// `isochore` descriptor.
pub const ISOCHORE_DESCRIPTOR: ToolDescriptor = ToolDescriptor::new(
    "isochore",
    "report a bounded isochore profile and emit a line-plot contract",
)
.with_family(FAMILY);
