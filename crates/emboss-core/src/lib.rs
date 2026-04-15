//! Shared biological and computational primitives for EMBOSS-RS.
//!
//! This crate is the home for durable domain types that should remain independent
//! of any single CLI tool, file format, provider integration, or frontend.

pub mod alphabet;
pub mod error;
pub mod feature;
pub mod identifier;
pub mod interval;
pub mod metadata;
pub mod molecule;
pub mod platform;
pub mod sequence;
pub mod strand;

pub use alphabet::Alphabet;
pub use error::DomainError;
pub use feature::{Feature, FeatureKind, FeatureLocation};
pub use identifier::SequenceIdentifier;
pub use interval::Interval;
pub use metadata::SequenceMetadata;
pub use molecule::MoleculeKind;
pub use platform::{PLATFORM_IDENTITY, PlatformIdentity};
pub use sequence::SequenceRecord;
pub use strand::Strand;
