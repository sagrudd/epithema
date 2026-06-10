//! Owned biological sequence records.
//!
//! Sequence records couple a validated owned residue string with durable
//! identity, metadata, and lightweight feature annotations. Residues are stored
//! in normalized uppercase form with whitespace removed during construction.

use crate::alphabet::Alphabet;
use crate::error::DomainError;
use crate::feature::Feature;
use crate::identifier::SequenceIdentifier;
use crate::interval::Interval;
use crate::metadata::SequenceMetadata;
use crate::molecule::MoleculeKind;

/// An owned biological sequence record with core metadata and annotations.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SequenceRecord {
    identifier: SequenceIdentifier,
    molecule: MoleculeKind,
    alphabet: Alphabet,
    residues: String,
    metadata: SequenceMetadata,
    features: Vec<Feature>,
}

impl SequenceRecord {
    /// Creates a validated sequence record from owned residue content.
    pub fn new(
        identifier: SequenceIdentifier,
        molecule: MoleculeKind,
        residues: impl Into<String>,
    ) -> Result<Self, DomainError> {
        let alphabet = Alphabet::from_molecule(molecule);
        Self::with_alphabet(identifier, molecule, alphabet, residues)
    }

    /// Creates a validated sequence record with an explicit alphabet.
    pub fn with_alphabet(
        identifier: SequenceIdentifier,
        molecule: MoleculeKind,
        alphabet: Alphabet,
        residues: impl Into<String>,
    ) -> Result<Self, DomainError> {
        let residues = residues.into();
        let residues = alphabet.normalize(molecule, &residues)?;

        if residues.is_empty() {
            return Err(DomainError::EmptySequence);
        }

        Ok(Self {
            identifier,
            molecule,
            alphabet,
            residues,
            metadata: SequenceMetadata::default(),
            features: Vec::new(),
        })
    }

    /// Returns the stable sequence identifier.
    #[must_use]
    pub fn identifier(&self) -> &SequenceIdentifier {
        &self.identifier
    }

    /// Returns the molecule kind.
    #[must_use]
    pub fn molecule(&self) -> MoleculeKind {
        self.molecule
    }

    /// Returns the derived alphabet used for residue validation.
    #[must_use]
    pub fn alphabet(&self) -> Alphabet {
        self.alphabet
    }

    /// Returns the residue string.
    #[must_use]
    pub fn residues(&self) -> &str {
        &self.residues
    }

    /// Returns the residue at the supplied zero-based position.
    #[must_use]
    pub fn residue_at(&self, position: usize) -> Option<char> {
        self.residues
            .as_bytes()
            .get(position)
            .map(|byte| char::from(*byte))
    }

    /// Returns the sequence length in residues.
    #[must_use]
    pub fn len(&self) -> usize {
        self.residues.len()
    }

    /// Returns true if the sequence contains no residues.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.residues.is_empty()
    }

    /// Returns the full span of the sequence as an interval.
    #[must_use]
    pub fn span(&self) -> Interval {
        Interval::new(0, self.len()).expect("non-empty sequences always have a valid span")
    }

    /// Returns the current metadata.
    #[must_use]
    pub fn metadata(&self) -> &SequenceMetadata {
        &self.metadata
    }

    /// Replaces the current metadata.
    #[must_use]
    pub fn with_metadata(mut self, metadata: SequenceMetadata) -> Self {
        self.metadata = metadata;
        self
    }

    /// Returns the attached features.
    #[must_use]
    pub fn features(&self) -> &[Feature] {
        &self.features
    }

    /// Returns true when the sequence has any attached features.
    #[must_use]
    pub fn has_features(&self) -> bool {
        !self.features.is_empty()
    }

    /// Returns a validated subsequence view over the requested interval.
    pub fn subsequence(&self, interval: Interval) -> Result<&str, DomainError> {
        if !self.span().contains_interval(interval) {
            return Err(DomainError::SequenceIntervalOutOfBounds {
                interval_end: interval.end(),
                sequence_length: self.len(),
            });
        }

        Ok(&self.residues[interval.start()..interval.end()])
    }

    /// Adds a feature if it lies within the sequence span.
    pub fn add_feature(&mut self, feature: Feature) -> Result<(), DomainError> {
        for span in feature.location.spans() {
            if !self.span().contains_interval(span.interval()) {
                return Err(DomainError::FeatureOutOfBounds {
                    feature_end: span.interval().end(),
                    sequence_length: self.len(),
                });
            }
        }

        self.features.push(feature);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::SequenceRecord;
    use crate::{
        Feature, FeatureKind, FeatureLocation, FeatureSpan, Interval, SequenceIdentifier,
        SequenceMetadata, Strand,
    };

    #[test]
    fn stores_metadata_and_validates_features() {
        let identifier = SequenceIdentifier::new("seq1").expect("valid identifier");
        let metadata = SequenceMetadata::new().with_description("example sequence");
        let mut record = SequenceRecord::new(identifier, crate::MoleculeKind::Dna, "ACGTAC")
            .expect("valid sequence")
            .with_metadata(metadata);

        assert_eq!(
            record.metadata().description.as_deref(),
            Some("example sequence")
        );

        let feature = Feature::new(
            FeatureKind::Gene,
            FeatureLocation::new(
                Interval::new(1, 4).expect("valid feature interval"),
                Strand::Forward,
            ),
        );

        record
            .add_feature(feature)
            .expect("feature should fit in sequence");
        assert_eq!(record.features().len(), 1);
    }

    #[test]
    fn rejects_feature_outside_sequence() {
        let identifier = SequenceIdentifier::new("seq2").expect("valid identifier");
        let mut record = SequenceRecord::new(identifier, crate::MoleculeKind::Protein, "MSTN")
            .expect("valid sequence");
        let feature = Feature::new(
            FeatureKind::Region,
            FeatureLocation::new(
                Interval::new(0, 6).expect("valid interval shape"),
                Strand::Unknown,
            ),
        );

        assert!(record.add_feature(feature).is_err());
    }

    #[test]
    fn normalizes_residues_and_supports_subsequence_extraction() {
        let identifier = SequenceIdentifier::new("seq3").expect("valid identifier");
        let record = SequenceRecord::new(identifier, crate::MoleculeKind::Dna, "ac gt\nac")
            .expect("valid sequence");

        assert_eq!(record.residues(), "ACGTAC");
        assert_eq!(record.residue_at(2), Some('G'));
        assert_eq!(
            record
                .subsequence(Interval::new(1, 4).expect("valid interval"))
                .expect("subsequence should exist"),
            "CGT"
        );
    }

    #[test]
    fn accepts_multi_span_feature_within_bounds() {
        let identifier = SequenceIdentifier::new("seq4").expect("valid identifier");
        let mut record = SequenceRecord::new(identifier, crate::MoleculeKind::Dna, "ACGTACGTACGT")
            .expect("valid sequence");
        let location = FeatureLocation::from_spans(vec![
            FeatureSpan::new(
                Interval::new(1, 4).expect("valid interval"),
                Strand::Forward,
            ),
            FeatureSpan::new(
                Interval::new(8, 10).expect("valid interval"),
                Strand::Forward,
            ),
        ])
        .expect("valid location");

        record
            .add_feature(Feature::new(FeatureKind::Gene, location))
            .expect("feature should fit");
        assert!(record.has_features());
    }
}
