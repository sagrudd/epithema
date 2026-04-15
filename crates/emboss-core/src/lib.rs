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
//! - extracted feature regions are rebased onto local zero-based half-open coordinates

pub mod alignment;
pub mod alphabet;
pub mod error;
pub mod feature;
pub mod feature_ops;
pub mod feature_selector;
pub mod identifier;
pub mod interval;
pub mod metadata;
pub mod molecule;
pub mod pattern;
pub mod platform;
pub mod sequence;
pub mod strand;
pub mod translation;

pub use alignment::{Alignment, AlignmentRow, AlignmentSymbol, GAP_SYMBOL};
pub use alphabet::Alphabet;
pub use error::DomainError;
pub use feature::{Feature, FeatureKind, FeatureLocation, FeatureSpan};
pub use feature_ops::{
    ExtractedFeatureRecord, FeatureOperationError, FeatureSummary, copy_selected_features,
    drop_selected_features, extract_selected_regions, extract_single_region, mask_intervals,
    mask_selected_features, retain_selected_features, select_features, summarize_features,
};
pub use feature_selector::FeatureSelector;
pub use identifier::SequenceIdentifier;
pub use interval::Interval;
pub use metadata::{SequenceMetadata, SequenceTopology};
pub use molecule::MoleculeKind;
pub use pattern::{NucleotidePattern, PatternError, PatternMatch, ProteinPattern};
pub use platform::{PLATFORM_IDENTITY, PlatformIdentity};
pub use sequence::SequenceRecord;
pub use strand::Strand;
pub use translation::{
    TranslationError, ambiguous_codon, backtranslate_ambiguous, backtranslate_representative,
    representative_codon, translate_dna_frame, translate_dna_strict,
};
