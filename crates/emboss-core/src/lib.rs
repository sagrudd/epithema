//! Shared biological and computational primitives for EMBOSS-RS.
//!
//! This crate is the home for durable domain types that should remain independent
//! of any single CLI tool, file format, provider integration, or frontend.
//!
//! Core coordinate conventions:
//! - sequence intervals are zero-based and half-open: `[start, end)`
//! - feature locations are one or more ordered, non-overlapping spans
//! - sequence records own normalized uppercase residue content and attached features
//! - alignment coordinates refer to aligned columns, with `-` as the canonical gap

pub mod alignment;
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

pub use alignment::{Alignment, AlignmentRow, AlignmentSymbol, GAP_SYMBOL};
pub use alphabet::Alphabet;
pub use error::DomainError;
pub use feature::{Feature, FeatureKind, FeatureLocation, FeatureSpan};
pub use identifier::SequenceIdentifier;
pub use interval::Interval;
pub use metadata::{SequenceMetadata, SequenceTopology};
pub use molecule::MoleculeKind;
pub use platform::{PLATFORM_IDENTITY, PlatformIdentity};
pub use sequence::SequenceRecord;
pub use strand::Strand;
