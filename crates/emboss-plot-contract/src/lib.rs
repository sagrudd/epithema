//! Plot payload contracts for handing graphical data from Rust to the R surface.

/// A named numeric series in a plot payload.
#[derive(Clone, Debug, PartialEq)]
pub struct PlotSeries {
    /// Series label.
    pub name: String,
    /// Y values for the series.
    pub values: Vec<f64>,
}

/// Plot-ready payload emitted by Rust for rendering elsewhere.
#[derive(Clone, Debug, PartialEq)]
pub struct PlotPayload {
    /// Plot title.
    pub title: String,
    /// Included numeric series.
    pub series: Vec<PlotSeries>,
}

impl PlotPayload {
    /// Creates an empty payload with the supplied title.
    #[must_use]
    pub fn empty(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            series: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PlotPayload;

    #[test]
    fn creates_empty_payload() {
        assert!(PlotPayload::empty("example").series.is_empty());
    }
}
