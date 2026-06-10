//! Molecule classification primitives.

use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// Broad molecule classes used throughout Epithema.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum MoleculeKind {
    /// DNA sequences.
    Dna,
    /// RNA sequences.
    Rna,
    /// Protein or peptide sequences.
    Protein,
    /// Molecule kind is not yet known.
    Unknown,
}

impl MoleculeKind {
    /// Returns the canonical lowercase label for the molecule kind.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Dna => "dna",
            Self::Rna => "rna",
            Self::Protein => "protein",
            Self::Unknown => "unknown",
        }
    }

    /// Returns true when the molecule is nucleotide-based.
    #[must_use]
    pub fn is_nucleotide(self) -> bool {
        matches!(self, Self::Dna | Self::Rna)
    }

    /// Returns true when the molecule represents amino-acid sequence.
    #[must_use]
    pub fn is_protein(self) -> bool {
        matches!(self, Self::Protein)
    }
}

impl Display for MoleculeKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for MoleculeKind {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.trim().to_ascii_lowercase().as_str() {
            "dna" => Ok(Self::Dna),
            "rna" => Ok(Self::Rna),
            "protein" | "peptide" => Ok(Self::Protein),
            "unknown" => Ok(Self::Unknown),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MoleculeKind;

    #[test]
    fn parses_molecule_aliases() {
        assert_eq!("DNA".parse::<MoleculeKind>(), Ok(MoleculeKind::Dna));
        assert_eq!("peptide".parse::<MoleculeKind>(), Ok(MoleculeKind::Protein));
    }
}
