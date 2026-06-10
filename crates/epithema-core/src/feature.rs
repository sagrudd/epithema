//! Lightweight feature and annotation primitives.

use std::collections::BTreeMap;

use crate::error::DomainError;
use crate::interval::Interval;
use crate::strand::Strand;

/// Broad feature categories for sequence annotation.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum FeatureKind {
    /// A named gene region.
    Gene,
    /// A coding sequence.
    CodingSequence,
    /// An exon span.
    Exon,
    /// An intron span.
    Intron,
    /// A translated or translated-adjacent region.
    Region,
    /// A motif or site annotation.
    Motif,
    /// A repeat or low-complexity region.
    RepeatRegion,
    /// A catch-all annotation type.
    MiscFeature,
    /// A user-defined or not-yet-modelled feature label.
    Other(String),
}

/// Single span within a feature location.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct FeatureSpan {
    interval: Interval,
    strand: Strand,
}

impl FeatureSpan {
    /// Creates a feature span.
    #[must_use]
    pub fn new(interval: Interval, strand: Strand) -> Self {
        Self { interval, strand }
    }

    /// Returns the span interval.
    #[must_use]
    pub fn interval(self) -> Interval {
        self.interval
    }

    /// Returns the span strand.
    #[must_use]
    pub fn strand(self) -> Strand {
        self.strand
    }
}

/// Feature location expressed as one or more ordered spans.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct FeatureLocation {
    spans: Vec<FeatureSpan>,
}

impl FeatureLocation {
    /// Creates a feature location from a single span.
    #[must_use]
    pub fn new(interval: Interval, strand: Strand) -> Self {
        Self {
            spans: vec![FeatureSpan::new(interval, strand)],
        }
    }

    /// Creates a validated feature location from multiple spans.
    pub fn from_spans(spans: Vec<FeatureSpan>) -> Result<Self, DomainError> {
        if spans.is_empty() {
            return Err(DomainError::EmptyFeatureLocation);
        }

        for window in spans.windows(2) {
            let previous = window[0].interval();
            let next = window[1].interval();
            if previous.end() > next.start() {
                return Err(DomainError::OverlappingFeatureSpans {
                    previous_end: previous.end(),
                    next_start: next.start(),
                });
            }
        }

        Ok(Self { spans })
    }

    /// Returns the ordered spans for the location.
    #[must_use]
    pub fn spans(&self) -> &[FeatureSpan] {
        &self.spans
    }

    /// Returns the total covered length across all spans.
    #[must_use]
    pub fn len(&self) -> usize {
        self.spans.iter().map(|span| span.interval().len()).sum()
    }

    /// Returns true when there are no positions covered by the location.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.spans.is_empty()
    }

    /// Returns the overall bounds spanning the first to last feature span.
    #[must_use]
    pub fn bounds(&self) -> Interval {
        let first = self.spans.first().expect("feature locations are non-empty");
        let last = self.spans.last().expect("feature locations are non-empty");
        Interval::new(first.interval().start(), last.interval().end())
            .expect("ordered spans always produce valid bounds")
    }

    /// Returns a shared strand when all spans use the same strand.
    #[must_use]
    pub fn strand(&self) -> Option<Strand> {
        let first = self.spans.first().map(|span| span.strand())?;
        self.spans
            .iter()
            .all(|span| span.strand() == first)
            .then_some(first)
    }

    /// Returns true when the location has exactly one span.
    #[must_use]
    pub fn is_simple(&self) -> bool {
        self.spans.len() == 1
    }

    /// Returns the single span for simple locations.
    #[must_use]
    pub fn single_span(&self) -> Option<FeatureSpan> {
        (self.spans.len() == 1).then(|| self.spans[0])
    }

    /// Returns true when any span overlaps the supplied interval.
    #[must_use]
    pub fn overlaps(&self, interval: Interval) -> bool {
        self.spans
            .iter()
            .any(|span| span.interval().intersects(interval))
    }

    /// Returns true when all spans are fully contained within the supplied interval.
    #[must_use]
    pub fn contained_within(&self, interval: Interval) -> bool {
        self.spans
            .iter()
            .all(|span| interval.contains_interval(span.interval()))
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
    /// Stable typed qualifiers for later IO and tool work.
    pub qualifiers: BTreeMap<String, String>,
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
            qualifiers: BTreeMap::new(),
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

    /// Adds a qualifier to the feature.
    #[must_use]
    pub fn with_qualifier(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.qualifiers.insert(key.into(), value.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::{Feature, FeatureKind, FeatureLocation, FeatureSpan};
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
        .with_note("example")
        .with_qualifier("product", "example protein");

        assert_eq!(feature.name.as_deref(), Some("geneX"));
        assert_eq!(feature.note.as_deref(), Some("example"));
        assert_eq!(
            feature.qualifiers.get("product").map(String::as_str),
            Some("example protein")
        );
    }

    #[test]
    fn validates_ordered_multi_span_locations() {
        let location = FeatureLocation::from_spans(vec![
            FeatureSpan::new(
                Interval::new(2, 6).expect("valid interval"),
                Strand::Forward,
            ),
            FeatureSpan::new(
                Interval::new(10, 14).expect("valid interval"),
                Strand::Forward,
            ),
        ])
        .expect("spans should be ordered");

        assert_eq!(location.len(), 8);
        assert_eq!(
            location.bounds(),
            Interval::new(2, 14).expect("valid bounds")
        );
        assert_eq!(location.strand(), Some(Strand::Forward));
    }

    #[test]
    fn rejects_overlapping_multi_span_locations() {
        let error = FeatureLocation::from_spans(vec![
            FeatureSpan::new(
                Interval::new(2, 8).expect("valid interval"),
                Strand::Forward,
            ),
            FeatureSpan::new(
                Interval::new(6, 10).expect("valid interval"),
                Strand::Forward,
            ),
        ])
        .expect_err("spans should not overlap");

        assert!(error.to_string().contains("non-overlapping"));
    }
}
