//! Error types for foundational EMBOSS-RS domain validation.

use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::alphabet::Alphabet;
use crate::molecule::MoleculeKind;

/// Domain-level validation errors for foundational EMBOSS-RS types.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DomainError {
    /// A required identifier was empty or contained only whitespace.
    EmptyIdentifier,
    /// A sequence contained no residues.
    EmptySequence,
    /// An interval was invalid for the chosen coordinate system.
    InvalidInterval {
        /// Proposed inclusive start coordinate.
        start: usize,
        /// Proposed exclusive end coordinate.
        end: usize,
    },
    /// A residue string did not match the expected alphabet.
    InvalidResidues {
        /// Molecule kind associated with the invalid residues.
        molecule: MoleculeKind,
        /// Alphabet used for validation.
        alphabet: Alphabet,
        /// First invalid residue encountered.
        invalid_symbol: char,
        /// Zero-based residue offset of the invalid symbol.
        position: usize,
    },
    /// A feature location extended beyond the associated sequence length.
    FeatureOutOfBounds {
        /// Exclusive feature end coordinate.
        feature_end: usize,
        /// Length of the associated sequence.
        sequence_length: usize,
    },
}

impl Display for DomainError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyIdentifier => write!(f, "identifier must not be empty"),
            Self::EmptySequence => write!(f, "sequence must contain at least one residue"),
            Self::InvalidInterval { start, end } => {
                write!(
                    f,
                    "invalid interval: start ({start}) must be less than end ({end})"
                )
            }
            Self::InvalidResidues {
                molecule,
                alphabet,
                invalid_symbol,
                position,
            } => write!(
                f,
                "invalid residue '{invalid_symbol}' at position {position} for {molecule} using {alphabet}"
            ),
            Self::FeatureOutOfBounds {
                feature_end,
                sequence_length,
            } => write!(
                f,
                "feature end ({feature_end}) exceeds sequence length ({sequence_length})"
            ),
        }
    }
}

impl Error for DomainError {}
