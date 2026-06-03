//! Shared reverse and reverse-complement helpers for `revseq`-style tools.

use std::fmt::{Display, Formatter};

use crate::{MoleculeKind, SequenceRecord};

/// Explicit reverse behavior requested by `revseq`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RevseqMode {
    /// Reverse-complement nucleotide sequences and reverse other molecule kinds.
    Auto,
    /// Reverse residues only.
    ReverseOnly,
    /// Require reverse-complement behavior.
    ReverseComplement,
}

impl Display for RevseqMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Auto => f.write_str("auto"),
            Self::ReverseOnly => f.write_str("reverse-only"),
            Self::ReverseComplement => f.write_str("reverse-complement"),
        }
    }
}

/// Errors returned by shared `revseq` transformations.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RevseqError {
    /// Reverse-complement was requested for a non-nucleotide molecule.
    UnsupportedReverseComplement {
        /// Molecule kind that cannot be reverse-complemented.
        molecule: MoleculeKind,
    },
    /// A nucleotide symbol could not be complemented.
    UnsupportedResidue {
        /// Molecule kind being complemented.
        molecule: MoleculeKind,
        /// Unsupported normalized residue.
        residue: char,
    },
    /// Annotated records are not yet remapped by `revseq`.
    UnsupportedAnnotatedRecord,
    /// Creating the transformed record failed domain validation.
    InvalidSequence(crate::DomainError),
}

impl Display for RevseqError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnsupportedReverseComplement { molecule } => write!(
                f,
                "reverse-complement is not supported for molecule kind {molecule}"
            ),
            Self::UnsupportedResidue { molecule, residue } => write!(
                f,
                "reverse-complement for molecule kind {molecule} does not support residue '{residue}'"
            ),
            Self::UnsupportedAnnotatedRecord => {
                f.write_str("revseq does not yet support records with attached features")
            }
            Self::InvalidSequence(error) => Display::fmt(error, f),
        }
    }
}

impl std::error::Error for RevseqError {}

/// Returns the residue string in reverse order without complementing.
#[must_use]
pub fn reverse_residues(residues: &str) -> String {
    residues.chars().rev().collect()
}

/// Returns the reverse-complement of a nucleotide residue string.
pub fn reverse_complement_residues(
    molecule: MoleculeKind,
    residues: &str,
) -> Result<String, RevseqError> {
    match molecule {
        MoleculeKind::Dna => residues
            .chars()
            .rev()
            .map(|symbol| {
                complement_dna(symbol).ok_or(RevseqError::UnsupportedResidue {
                    molecule,
                    residue: symbol,
                })
            })
            .collect(),
        MoleculeKind::Rna => residues
            .chars()
            .rev()
            .map(|symbol| {
                complement_rna(symbol).ok_or(RevseqError::UnsupportedResidue {
                    molecule,
                    residue: symbol,
                })
            })
            .collect(),
        _ => Err(RevseqError::UnsupportedReverseComplement { molecule }),
    }
}

/// Transforms one record according to the requested `revseq` mode.
pub fn transform_sequence_record(
    record: &SequenceRecord,
    mode: RevseqMode,
) -> Result<SequenceRecord, RevseqError> {
    if record.has_features() {
        return Err(RevseqError::UnsupportedAnnotatedRecord);
    }

    let residues = match mode {
        RevseqMode::Auto => {
            if record.molecule().is_nucleotide() {
                reverse_complement_residues(record.molecule(), record.residues())?
            } else {
                reverse_residues(record.residues())
            }
        }
        RevseqMode::ReverseOnly => reverse_residues(record.residues()),
        RevseqMode::ReverseComplement => {
            reverse_complement_residues(record.molecule(), record.residues())?
        }
    };

    SequenceRecord::new(record.identifier().clone(), record.molecule(), residues)
        .map(|updated| updated.with_metadata(record.metadata().clone()))
        .map_err(RevseqError::InvalidSequence)
}

fn complement_dna(symbol: char) -> Option<char> {
    match symbol {
        'A' => Some('T'),
        'T' => Some('A'),
        'G' => Some('C'),
        'C' => Some('G'),
        'N' => Some('N'),
        'R' => Some('Y'),
        'Y' => Some('R'),
        'S' => Some('S'),
        'W' => Some('W'),
        'K' => Some('M'),
        'M' => Some('K'),
        'B' => Some('V'),
        'V' => Some('B'),
        'D' => Some('H'),
        'H' => Some('D'),
        '-' => Some('-'),
        '*' => Some('*'),
        _ => None,
    }
}

fn complement_rna(symbol: char) -> Option<char> {
    match symbol {
        'A' => Some('U'),
        'U' => Some('A'),
        'G' => Some('C'),
        'C' => Some('G'),
        'N' => Some('N'),
        'R' => Some('Y'),
        'Y' => Some('R'),
        'S' => Some('S'),
        'W' => Some('W'),
        'K' => Some('M'),
        'M' => Some('K'),
        'B' => Some('V'),
        'V' => Some('B'),
        'D' => Some('H'),
        'H' => Some('D'),
        '-' => Some('-'),
        '*' => Some('*'),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        Feature, FeatureKind, FeatureLocation, Interval, MoleculeKind, SequenceIdentifier, Strand,
    };

    use super::{
        reverse_complement_residues, reverse_residues, transform_sequence_record, RevseqError,
        RevseqMode,
    };

    fn record(id: &str, molecule: MoleculeKind, residues: &str) -> crate::SequenceRecord {
        crate::SequenceRecord::new(
            SequenceIdentifier::new(id).expect("valid identifier"),
            molecule,
            residues,
        )
        .expect("valid sequence")
    }

    #[test]
    fn reverses_without_complement() {
        assert_eq!(reverse_residues("ACGT"), "TGCA");
    }

    #[test]
    fn reverse_complements_dna_with_ambiguity_symbols() {
        let transformed = reverse_complement_residues(MoleculeKind::Dna, "ATGRN")
            .expect("dna reverse complement should succeed");
        assert_eq!(transformed, "NYCAT");
    }

    #[test]
    fn reverse_complements_rna() {
        let transformed = reverse_complement_residues(MoleculeKind::Rna, "AUGCN")
            .expect("rna reverse complement should succeed");
        assert_eq!(transformed, "NGCAU");
    }

    #[test]
    fn auto_mode_reverse_complements_dna_and_reverses_protein() {
        let dna =
            transform_sequence_record(&record("dna", MoleculeKind::Dna, "ATGC"), RevseqMode::Auto)
                .expect("dna should transform");
        let protein = transform_sequence_record(
            &record("protein", MoleculeKind::Protein, "MA*"),
            RevseqMode::Auto,
        )
        .expect("protein should reverse only");

        assert_eq!(dna.residues(), "GCAT");
        assert_eq!(protein.residues(), "*AM");
    }

    #[test]
    fn explicit_reverse_complement_rejects_protein() {
        let error = transform_sequence_record(
            &record("protein", MoleculeKind::Protein, "MA"),
            RevseqMode::ReverseComplement,
        )
        .expect_err("protein reverse complement should fail");
        assert_eq!(
            error,
            RevseqError::UnsupportedReverseComplement {
                molecule: MoleculeKind::Protein
            }
        );
    }

    #[test]
    fn rejects_annotated_records() {
        let mut dna = record("dna", MoleculeKind::Dna, "ATGC");
        dna.add_feature(Feature::new(
            FeatureKind::Gene,
            FeatureLocation::new(Interval::new(0, 2).expect("interval"), Strand::Forward),
        ))
        .expect("feature should fit");

        let error = transform_sequence_record(&dna, RevseqMode::Auto)
            .expect_err("annotated record should fail");
        assert_eq!(error, RevseqError::UnsupportedAnnotatedRecord);
    }
}
