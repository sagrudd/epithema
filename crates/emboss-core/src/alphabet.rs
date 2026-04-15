//! Alphabet definitions for biological residue validation.

use std::fmt::{Display, Formatter};

use crate::error::DomainError;
use crate::molecule::MoleculeKind;

/// Canonical residue alphabets used for foundational validation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Alphabet {
    /// DNA nucleotide alphabet with common ambiguity symbols.
    Dna,
    /// RNA nucleotide alphabet with common ambiguity symbols.
    Rna,
    /// Protein alphabet with common ambiguity symbols.
    Protein,
    /// Free-text or currently unconstrained sequence content.
    Text,
}

impl Alphabet {
    /// Derives an alphabet from a molecule kind.
    #[must_use]
    pub fn from_molecule(molecule: MoleculeKind) -> Self {
        match molecule {
            MoleculeKind::Dna => Self::Dna,
            MoleculeKind::Rna => Self::Rna,
            MoleculeKind::Protein => Self::Protein,
            MoleculeKind::Unknown => Self::Text,
        }
    }

    /// Returns true if the supplied symbol is valid for this alphabet.
    #[must_use]
    pub fn allows(self, symbol: char) -> bool {
        let symbol = symbol.to_ascii_uppercase();

        match self {
            Self::Dna => matches!(
                symbol,
                'A' | 'C'
                    | 'G'
                    | 'T'
                    | 'N'
                    | 'R'
                    | 'Y'
                    | 'S'
                    | 'W'
                    | 'K'
                    | 'M'
                    | 'B'
                    | 'D'
                    | 'H'
                    | 'V'
                    | '-'
                    | '*'
            ),
            Self::Rna => matches!(
                symbol,
                'A' | 'C'
                    | 'G'
                    | 'U'
                    | 'N'
                    | 'R'
                    | 'Y'
                    | 'S'
                    | 'W'
                    | 'K'
                    | 'M'
                    | 'B'
                    | 'D'
                    | 'H'
                    | 'V'
                    | '-'
                    | '*'
            ),
            Self::Protein => matches!(
                symbol,
                'A' | 'B'
                    | 'C'
                    | 'D'
                    | 'E'
                    | 'F'
                    | 'G'
                    | 'H'
                    | 'I'
                    | 'J'
                    | 'K'
                    | 'L'
                    | 'M'
                    | 'N'
                    | 'O'
                    | 'P'
                    | 'Q'
                    | 'R'
                    | 'S'
                    | 'T'
                    | 'U'
                    | 'V'
                    | 'W'
                    | 'X'
                    | 'Y'
                    | 'Z'
                    | '*'
                    | '-'
            ),
            Self::Text => !symbol.is_whitespace(),
        }
    }

    /// Validates a residue string against the alphabet.
    pub fn validate(self, molecule: MoleculeKind, residues: &str) -> Result<(), DomainError> {
        for (position, symbol) in residues.chars().enumerate() {
            if !self.allows(symbol) {
                return Err(DomainError::InvalidResidues {
                    molecule,
                    alphabet: self,
                    invalid_symbol: symbol,
                    position,
                });
            }
        }

        Ok(())
    }
}

impl Display for Alphabet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let label = match self {
            Self::Dna => "DNA alphabet",
            Self::Rna => "RNA alphabet",
            Self::Protein => "protein alphabet",
            Self::Text => "text alphabet",
        };

        f.write_str(label)
    }
}

#[cfg(test)]
mod tests {
    use super::Alphabet;
    use crate::molecule::MoleculeKind;

    #[test]
    fn rejects_rna_in_dna_alphabet() {
        let error = Alphabet::Dna
            .validate(MoleculeKind::Dna, "ACGU")
            .expect_err("U is not valid DNA");

        assert!(error.to_string().contains("invalid residue 'U'"));
    }
}
