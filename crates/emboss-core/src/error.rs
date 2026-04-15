//! Error types for foundational EMBOSS-RS domain validation.

use std::error::Error;
use std::fmt::{Display, Formatter};

use emboss_diagnostics::{ErrorCategory, PlatformError};

use crate::alphabet::Alphabet;
use crate::molecule::MoleculeKind;

/// Domain-level validation errors for foundational EMBOSS-RS types.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DomainError {
    /// A required identifier was empty or contained only whitespace.
    EmptyIdentifier,
    /// A sequence contained no residues.
    EmptySequence,
    /// The supplied alphabet is not compatible with the requested molecule kind.
    IncompatibleAlphabet {
        /// Molecule kind associated with the sequence.
        molecule: MoleculeKind,
        /// Alphabet requested for validation.
        alphabet: Alphabet,
    },
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
    /// A feature location contained no spans.
    EmptyFeatureLocation,
    /// Feature spans were not strictly ordered and disjoint.
    OverlappingFeatureSpans {
        /// Exclusive end of the previous span.
        previous_end: usize,
        /// Start of the following conflicting span.
        next_start: usize,
    },
    /// A feature location extended beyond the associated sequence length.
    FeatureOutOfBounds {
        /// Exclusive feature end coordinate.
        feature_end: usize,
        /// Length of the associated sequence.
        sequence_length: usize,
    },
    /// A requested interval or coordinate extended beyond sequence length.
    SequenceIntervalOutOfBounds {
        /// Exclusive end coordinate of the requested interval.
        interval_end: usize,
        /// Length of the associated sequence.
        sequence_length: usize,
    },
}

impl Display for DomainError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyIdentifier => write!(f, "identifier must not be empty"),
            Self::EmptySequence => write!(f, "sequence must contain at least one residue"),
            Self::IncompatibleAlphabet { molecule, alphabet } => write!(
                f,
                "alphabet {alphabet} is not compatible with molecule kind {molecule}"
            ),
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
            Self::EmptyFeatureLocation => {
                write!(f, "feature location must contain at least one span")
            }
            Self::OverlappingFeatureSpans {
                previous_end,
                next_start,
            } => write!(
                f,
                "feature spans must be ordered and non-overlapping: previous end ({previous_end}) exceeds next start ({next_start})"
            ),
            Self::FeatureOutOfBounds {
                feature_end,
                sequence_length,
            } => write!(
                f,
                "feature end ({feature_end}) exceeds sequence length ({sequence_length})"
            ),
            Self::SequenceIntervalOutOfBounds {
                interval_end,
                sequence_length,
            } => write!(
                f,
                "requested interval end ({interval_end}) exceeds sequence length ({sequence_length})"
            ),
        }
    }
}

impl Error for DomainError {}

impl From<DomainError> for PlatformError {
    fn from(value: DomainError) -> Self {
        match value {
            DomainError::EmptyIdentifier => {
                PlatformError::new(ErrorCategory::Validation, value.to_string())
                    .with_code("core.identifier.empty")
            }
            DomainError::EmptySequence => {
                PlatformError::new(ErrorCategory::Validation, value.to_string())
                    .with_code("core.sequence.empty")
            }
            DomainError::IncompatibleAlphabet { .. } => {
                PlatformError::new(ErrorCategory::Validation, value.to_string())
                    .with_code("core.sequence.incompatible_alphabet")
            }
            DomainError::InvalidInterval { .. } => {
                PlatformError::new(ErrorCategory::Validation, value.to_string())
                    .with_code("core.interval.invalid")
            }
            DomainError::InvalidResidues { .. } => {
                PlatformError::new(ErrorCategory::Validation, value.to_string())
                    .with_code("core.sequence.invalid_residues")
            }
            DomainError::FeatureOutOfBounds { .. } => {
                PlatformError::new(ErrorCategory::Validation, value.to_string())
                    .with_code("core.feature.out_of_bounds")
            }
            DomainError::EmptyFeatureLocation => {
                PlatformError::new(ErrorCategory::Validation, value.to_string())
                    .with_code("core.feature.empty_location")
            }
            DomainError::OverlappingFeatureSpans { .. } => {
                PlatformError::new(ErrorCategory::Validation, value.to_string())
                    .with_code("core.feature.overlapping_spans")
            }
            DomainError::SequenceIntervalOutOfBounds { .. } => {
                PlatformError::new(ErrorCategory::Validation, value.to_string())
                    .with_code("core.sequence.interval_out_of_bounds")
            }
        }
    }
}
