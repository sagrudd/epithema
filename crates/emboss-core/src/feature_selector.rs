//! Typed feature-selection predicates for annotated sequence records.

use crate::feature::{Feature, FeatureKind};
use crate::interval::Interval;
use crate::strand::Strand;

/// Typed predicate model for selecting features from annotated records.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FeatureSelector {
    /// Match all features.
    Any,
    /// Match a specific feature kind.
    Kind(FeatureKind),
    /// Match a feature name or label exactly.
    Name(String),
    /// Match features carrying a qualifier key and optional value.
    Qualifier {
        /// Qualifier key to match.
        key: String,
        /// Optional exact qualifier value to match.
        value: Option<String>,
    },
    /// Match features on a particular strand.
    Strand(Strand),
    /// Match features overlapping the supplied interval.
    Overlaps(Interval),
    /// Match features fully contained within the supplied interval.
    ContainedWithin(Interval),
    /// Logical conjunction of child selectors.
    All(Vec<FeatureSelector>),
    /// Logical disjunction of child selectors.
    AnyOf(Vec<FeatureSelector>),
    /// Negated child selector.
    Not(Box<FeatureSelector>),
}

impl FeatureSelector {
    /// Returns true when the selector matches the supplied feature.
    #[must_use]
    pub fn matches(&self, feature: &Feature) -> bool {
        match self {
            Self::Any => true,
            Self::Kind(kind) => &feature.kind == kind,
            Self::Name(name) => feature.name.as_deref() == Some(name.as_str()),
            Self::Qualifier { key, value } => match feature.qualifiers.get(key) {
                Some(actual) => value.as_ref().is_none_or(|expected| expected == actual),
                None => false,
            },
            Self::Strand(strand) => feature.location.strand() == Some(*strand),
            Self::Overlaps(interval) => feature.location.overlaps(*interval),
            Self::ContainedWithin(interval) => feature.location.contained_within(*interval),
            Self::All(selectors) => selectors.iter().all(|selector| selector.matches(feature)),
            Self::AnyOf(selectors) => selectors.iter().any(|selector| selector.matches(feature)),
            Self::Not(selector) => !selector.matches(feature),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Feature, FeatureKind, FeatureLocation, FeatureSelector, Interval, Strand};

    #[test]
    fn matches_by_kind_and_qualifier() {
        let feature = Feature::new(
            FeatureKind::Gene,
            FeatureLocation::new(
                Interval::new(5, 10).expect("valid interval"),
                Strand::Forward,
            ),
        )
        .with_name("geneX")
        .with_qualifier("product", "enzyme");

        assert!(FeatureSelector::Kind(FeatureKind::Gene).matches(&feature));
        assert!(
            FeatureSelector::Qualifier {
                key: "product".to_owned(),
                value: Some("enzyme".to_owned()),
            }
            .matches(&feature)
        );
        assert!(
            !FeatureSelector::Qualifier {
                key: "product".to_owned(),
                value: Some("other".to_owned()),
            }
            .matches(&feature)
        );
    }
}
