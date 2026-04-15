//! Input and output format support scaffolding for EMBOSS-RS.

/// Minimal description of a supported data format.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DataFormat {
    /// Stable format identifier.
    pub name: &'static str,
    /// Short operational summary.
    pub summary: &'static str,
}

/// Registry of known formats in the current workspace state.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct FormatCatalog {
    formats: Vec<DataFormat>,
}

impl FormatCatalog {
    /// Creates an empty format catalog for the initial workspace skeleton.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the currently registered formats.
    #[must_use]
    pub fn formats(&self) -> &[DataFormat] {
        &self.formats
    }
}

#[cfg(test)]
mod tests {
    use super::FormatCatalog;

    #[test]
    fn starts_empty() {
        assert!(FormatCatalog::new().formats().is_empty());
    }
}
