//! Shared sequence metadata types.

/// Lightweight metadata attached to a biological sequence record.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SequenceMetadata {
    /// Free-text description for human-readable contexts.
    pub description: Option<String>,
    /// High-level source label, provider name, or provenance hint.
    pub source: Option<String>,
}

impl SequenceMetadata {
    /// Creates empty metadata.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets a description.
    #[must_use]
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets a source label.
    #[must_use]
    pub fn with_source(mut self, source: impl Into<String>) -> Self {
        self.source = Some(source.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::SequenceMetadata;

    #[test]
    fn preserves_metadata_fields() {
        let metadata = SequenceMetadata::new()
            .with_description("example record")
            .with_source("fixtures");

        assert_eq!(metadata.description.as_deref(), Some("example record"));
        assert_eq!(metadata.source.as_deref(), Some("fixtures"));
    }
}
