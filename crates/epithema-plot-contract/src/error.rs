use std::fmt;

/// Validation and serialization errors for the typed plot contract.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PlotContractError {
    /// Plot title must be present.
    EmptyTitle,
    /// At least one series must be present.
    EmptySeriesSet,
    /// A required axis label is missing.
    MissingAxisLabel {
        /// Axis name.
        axis: &'static str,
    },
    /// Mixed x-vector types across series are not supported in v1.
    MixedXValueKinds,
    /// A series contains no points.
    EmptySeriesData {
        /// Stable series identifier.
        series_id: String,
    },
    /// A series contains mismatched x/y vector lengths.
    InconsistentSeriesLengths {
        /// Stable series identifier.
        series_id: String,
        /// Length of the x vector.
        x_len: usize,
        /// Length of the y vector.
        y_len: usize,
    },
    /// A series uses a geometry hint that is incompatible with the plot kind.
    GeometryKindMismatch {
        /// Stable series identifier.
        series_id: String,
        /// Plot kind name.
        plot_kind: &'static str,
        /// Geometry hint name.
        geometry_hint: &'static str,
    },
    /// JSON serialization failed.
    Serialization(String),
    /// JSON deserialization failed.
    Deserialization(String),
}

impl fmt::Display for PlotContractError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyTitle => write!(f, "plot metadata.title must not be empty"),
            Self::EmptySeriesSet => write!(f, "plot must contain at least one series"),
            Self::MissingAxisLabel { axis } => {
                write!(f, "plot {axis}_axis.label must not be empty")
            }
            Self::MixedXValueKinds => write!(f, "all series must use the same x vector kind in v1"),
            Self::EmptySeriesData { series_id } => {
                write!(f, "series '{series_id}' must contain at least one point")
            }
            Self::InconsistentSeriesLengths {
                series_id,
                x_len,
                y_len,
            } => write!(
                f,
                "series '{series_id}' has inconsistent x/y lengths ({x_len} vs {y_len})"
            ),
            Self::GeometryKindMismatch {
                series_id,
                plot_kind,
                geometry_hint,
            } => write!(
                f,
                "series '{series_id}' uses geometry hint '{geometry_hint}' incompatible with plot kind '{plot_kind}'"
            ),
            Self::Serialization(message) => {
                write!(f, "plot contract serialization failed: {message}")
            }
            Self::Deserialization(message) => {
                write!(f, "plot contract deserialization failed: {message}")
            }
        }
    }
}

impl std::error::Error for PlotContractError {}
