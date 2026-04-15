//! Lightweight feature and annotation primitives.

use crate::interval::Interval;
use crate::strand::Strand;

/// Broad feature categories for sequence annotation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum FeatureKind {
    /// A named gene region.
    Gene,
    /// A coding sequence.
    CodingSequence,
    /// A translated or translated-adjacent region.
    Region,
    /// A motif or site annotation.
    Motif,
    /// A catch-all annotation type.
    MiscFeature,
}

/// Feature location expressed as an interval and strand.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct FeatureLocation {
    /// Feature span on the associated sequence.
    pub interval: Interval,
    /// Strand associated with the feature.
    pub strand: Strand,
}

impl FeatureLocation {
    /// Creates a feature location.
    #[must_use]
    pub fn new(interval: Interval, strand: Strand) -> Self {
        Self { interval, strand }
    }
}

/// Lightweight sequence feature annotation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Feature {
    /// Feature classification.
    pub kind: FeatureKind,
    /// Optional feature label or name.
    pub name: Option<String>,
    /// Genomic or sequence-local feature location.
    pub location: FeatureLocation,
    /// Optional free-text note.
    pub note: Option<String>,
}

impl Feature {
    /// Creates a feature with the supplied kind and location.
    #[must_use]
    pub fn new(kind: FeatureKind, location: FeatureLocation) -> Self {
        Self {
            kind,
            name: None,
            location,
            note: None,
        }
    }

    /// Sets a feature name.
    #[must_use]
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Sets a free-text feature note.
    #[must_use]
    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.note = Some(note.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::{Feature, FeatureKind, FeatureLocation};
    use crate::{Interval, Strand};

    #[test]
    fn stores_feature_metadata() {
        let feature = Feature::new(
            FeatureKind::Gene,
            FeatureLocation::new(
                Interval::new(4, 12).expect("valid interval"),
                Strand::Forward,
            ),
        )
        .with_name("geneX")
        .with_note("example");

        assert_eq!(feature.name.as_deref(), Some("geneX"));
        assert_eq!(feature.note.as_deref(), Some("example"));
    }
}
