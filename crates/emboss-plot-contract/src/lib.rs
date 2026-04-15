//! Typed plot-contract layer for handing plot-ready analytical payloads from
//! Rust to the R-owned rendering backend in the sister `emboss-r` project.
//!
//! Rendering does not happen in Rust. Rust owns scientific computation and the
//! generation of plot-ready semantic structures; R owns rendering choices and
//! graphics backends. This crate defines the stable JSON-serializable contract
//! between those two layers.

/// Axis metadata and scale hints.
pub mod axis;
/// Validation and serialization errors.
pub mod error;
/// Optional faceting metadata.
pub mod facet;
/// Plot display metadata and provenance.
pub mod metadata;
/// Data-series and style hint types.
pub mod series;
/// Top-level plot specification and validation.
pub mod spec;

pub use axis::{AxisScaleHint, PlotAxis};
pub use error::PlotContractError;
pub use facet::{FacetScaleMode, PlotFacet};
pub use metadata::{PlotMetadata, PlotProvenance};
pub use series::{DataVector, GeometryHint, PlotSeries, SeriesStyle};
pub use spec::{PlotAnnotation, PlotKind, PlotReferenceAxis, PlotReferenceLine, PlotSpec};

/// Backwards-compatible alias for the governed plot payload type.
pub type PlotPayload = PlotSpec;
