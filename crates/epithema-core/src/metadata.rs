//! Shared sequence metadata types.

/// Coarse topology classification for a sequence record.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum SequenceTopology {
    /// The sequence is linear.
    Linear,
    /// The sequence is circular.
    Circular,
}

/// Lightweight metadata attached to a biological sequence record.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SequenceMetadata {
    /// Free-text description for human-readable contexts.
    pub description: Option<String>,
    /// High-level source label, provider name, or provenance hint.
    pub source: Option<String>,
    /// Optional organism or taxon label.
    pub organism: Option<String>,
    /// Optional topology hint.
    pub topology: Option<SequenceTopology>,
    /// Free-form comments preserved with the record.
    pub comments: Vec<String>,
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

    /// Sets an organism label.
    #[must_use]
    pub fn with_organism(mut self, organism: impl Into<String>) -> Self {
        self.organism = Some(organism.into());
        self
    }

    /// Sets a topology hint.
    #[must_use]
    pub fn with_topology(mut self, topology: SequenceTopology) -> Self {
        self.topology = Some(topology);
        self
    }

    /// Adds a free-form comment.
    #[must_use]
    pub fn with_comment(mut self, comment: impl Into<String>) -> Self {
        self.comments.push(comment.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::{SequenceMetadata, SequenceTopology};

    #[test]
    fn preserves_metadata_fields() {
        let metadata = SequenceMetadata::new()
            .with_description("example record")
            .with_source("fixtures")
            .with_organism("Homo sapiens")
            .with_topology(SequenceTopology::Linear)
            .with_comment("example");

        assert_eq!(metadata.description.as_deref(), Some("example record"));
        assert_eq!(metadata.source.as_deref(), Some("fixtures"));
        assert_eq!(metadata.organism.as_deref(), Some("Homo sapiens"));
        assert_eq!(metadata.topology, Some(SequenceTopology::Linear));
        assert_eq!(metadata.comments, vec!["example"]);
    }
}
