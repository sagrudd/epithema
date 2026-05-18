//! Deterministic protein digestion helpers for retained protease-analysis tools.

use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::residue_properties::protein_residue_property;

/// Supported v1 proteases or cleavage reagents.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DigestProtease {
    /// Cleaves after `K` or `R` unless the following residue is `P`.
    Trypsin,
    /// Cleaves after `K`.
    LysC,
    /// Cleaves after `R`.
    ArgC,
    /// Cleaves after `M`.
    CnBr,
}

impl DigestProtease {
    /// Stable lower-case CLI/documentation label.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Trypsin => "trypsin",
            Self::LysC => "lys-c",
            Self::ArgC => "arg-c",
            Self::CnBr => "cnbr",
        }
    }
}

/// One full-digest peptide fragment.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DigestedPeptide {
    /// One-based peptide ordinal within the digested record.
    pub ordinal: usize,
    /// One-based inclusive start coordinate in the source protein.
    pub start: usize,
    /// One-based inclusive end coordinate in the source protein.
    pub end: usize,
    /// One-based cleavage position that terminates the peptide, if a cleavage occurred.
    pub cleavage_after: Option<usize>,
    /// Peptide sequence.
    pub sequence: String,
}

/// Errors raised by deterministic protein digestion.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProteinDigestError {
    /// The sequence contains a residue unsupported by the v1 digestion model.
    UnsupportedResidue {
        /// The unsupported residue symbol after normalization.
        residue: char,
        /// The 1-based residue position in the normalized ungapped sequence.
        position: usize,
    },
}

impl Display for ProteinDigestError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnsupportedResidue { residue, position } => write!(
                f,
                "unsupported protein residue '{residue}' at position {position} for digestion"
            ),
        }
    }
}

impl Error for ProteinDigestError {}

/// Digests one protein sequence deterministically with the selected v1 protease rule set.
pub fn digest_protein_sequence(
    sequence: &str,
    protease: DigestProtease,
) -> Result<Vec<DigestedPeptide>, ProteinDigestError> {
    let residues = normalized_protein_residues(sequence)?;
    if residues.is_empty() {
        return Ok(Vec::new());
    }

    let mut peptides = Vec::new();
    let mut start = 0usize;
    let mut ordinal = 1usize;

    for index in 0..residues.len() {
        if !cleaves_after(&residues, index, protease) {
            continue;
        }

        peptides.push(DigestedPeptide {
            ordinal,
            start: start + 1,
            end: index + 1,
            cleavage_after: Some(index + 1),
            sequence: residues[start..=index].iter().collect(),
        });
        ordinal += 1;
        start = index + 1;
    }

    if start < residues.len() {
        peptides.push(DigestedPeptide {
            ordinal,
            start: start + 1,
            end: residues.len(),
            cleavage_after: None,
            sequence: residues[start..].iter().collect(),
        });
    }

    Ok(peptides)
}

fn normalized_protein_residues(sequence: &str) -> Result<Vec<char>, ProteinDigestError> {
    let mut residues = Vec::new();
    for (index, residue) in sequence.chars().enumerate() {
        let residue = residue.to_ascii_uppercase();
        if residue == '-' {
            continue;
        }

        if protein_residue_property(residue).is_none() {
            return Err(ProteinDigestError::UnsupportedResidue {
                residue,
                position: index + 1,
            });
        }

        residues.push(residue);
    }
    Ok(residues)
}

fn cleaves_after(residues: &[char], index: usize, protease: DigestProtease) -> bool {
    let residue = residues[index];
    let next = residues.get(index + 1).copied();

    match protease {
        DigestProtease::Trypsin => {
            (residue == 'K' || residue == 'R') && !matches!(next, Some('P'))
        }
        DigestProtease::LysC => residue == 'K',
        DigestProtease::ArgC => residue == 'R',
        DigestProtease::CnBr => residue == 'M',
    }
}

#[cfg(test)]
mod tests {
    use super::{DigestProtease, ProteinDigestError, digest_protein_sequence};

    #[test]
    fn digests_tryptic_peptides_with_proline_block() {
        let peptides =
            digest_protein_sequence("AKRPQMK", DigestProtease::Trypsin).expect("digest ok");

        assert_eq!(peptides.len(), 2);
        assert_eq!(peptides[0].sequence, "AK");
        assert_eq!(peptides[0].cleavage_after, Some(2));
        assert_eq!(peptides[1].sequence, "RPQMK");
        assert_eq!(peptides[1].cleavage_after, Some(7));
    }

    #[test]
    fn digests_with_cnbr_after_methionine() {
        let peptides =
            digest_protein_sequence("MAMK", DigestProtease::CnBr).expect("digest ok");

        assert_eq!(peptides.len(), 3);
        assert_eq!(peptides[0].sequence, "M");
        assert_eq!(peptides[1].sequence, "AM");
        assert_eq!(peptides[2].sequence, "K");
    }

    #[test]
    fn rejects_unsupported_residues() {
        assert_eq!(
            digest_protein_sequence("MAX", DigestProtease::Trypsin),
            Err(ProteinDigestError::UnsupportedResidue {
                residue: 'X',
                position: 3,
            })
        );
    }
}
