//! Owned biological sequence records.

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
        let residues = residues.into();
        let residues = residues.trim().to_owned();

        if residues.is_empty() {
            return Err(DomainError::EmptySequence);
        }

        let alphabet = Alphabet::from_molecule(molecule);
        alphabet.validate(molecule, &residues)?;

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

    /// Adds a feature if it lies within the sequence span.
    pub fn add_feature(&mut self, feature: Feature) -> Result<(), DomainError> {
        if !self.span().contains_interval(feature.location.interval) {
            return Err(DomainError::FeatureOutOfBounds {
                feature_end: feature.location.interval.end(),
                sequence_length: self.len(),
            });
        }

        self.features.push(feature);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::SequenceRecord;
    use crate::{
        Feature, FeatureKind, FeatureLocation, Interval, SequenceIdentifier, SequenceMetadata,
        Strand,
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
}
