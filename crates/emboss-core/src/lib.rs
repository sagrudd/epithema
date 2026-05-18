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
pub mod alignment_analysis;
pub mod alphabet;
pub mod codon_usage;
pub mod complexity;
pub mod composition;
pub mod error;
pub mod feature;
pub mod feature_ops;
pub mod feature_selector;
pub mod global_alignment;
pub mod identifier;
pub mod interval;
pub mod local_alignment;
pub mod metadata;
pub mod molecule;
pub mod pattern;
pub mod platform;
pub mod protein_charge;
pub mod protein_digest;
pub mod protein_hydropathy;
pub mod protein_isoelectric;
pub mod residue_properties;
pub mod revseq;
pub mod sequence;
pub mod strand;
pub mod translation;

pub use alignment::{Alignment, AlignmentRow, AlignmentSymbol, GAP_SYMBOL};
pub use alignment_analysis::{
    AlignmentAnalysisError, ConsensusStrategy, DirectMatchSummary, DistanceMatrix,
    consensus_sequence, direct_match_summary, p_distance_matrix,
};
pub use alphabet::Alphabet;
pub use codon_usage::{
    CodingSequenceSummary, CodonUsageError, CodonUsageProfile, amino_acid_for_sense_codon,
    cai_for_profile, derive_cai_weights, sense_codons, summarize_coding_sequence,
    total_variation_distance,
};
pub use complexity::{
    ComplexityError, ComplexityParameters, SequenceComplexity, WindowComplexity,
    sequence_complexity, sliding_window_complexity,
};
pub use composition::{CompositionError, GcSummary, ResidueComposition, protein_molecular_weight};
pub use error::DomainError;
pub use feature::{Feature, FeatureKind, FeatureLocation, FeatureSpan};
pub use feature_ops::{
    ExtractedFeatureRecord, FeatureOperationError, FeatureSummary, copy_selected_features,
    drop_selected_features, extract_selected_regions, extract_single_region, mask_intervals,
    mask_selected_features, retain_selected_features, select_features, summarize_features,
};
pub use feature_selector::FeatureSelector;
pub use global_alignment::{
    AlignmentMode, GlobalAlignmentError, GlobalAlignmentScoring, PairwiseAlignmentResult,
    PairwiseAlignmentSummary, global_align, infer_alignment_mode,
};
pub use identifier::SequenceIdentifier;
pub use interval::Interval;
pub use local_alignment::{
    LocalAlignmentError, LocalAlignmentScoring, LocalPairwiseAlignmentResult,
    LocalPairwiseAlignmentSummary, local_align,
};
pub use metadata::{SequenceMetadata, SequenceTopology};
pub use molecule::MoleculeKind;
pub use pattern::{NucleotidePattern, PatternError, PatternMatch, ProteinPattern};
pub use platform::{PLATFORM_IDENTITY, PlatformIdentity};
pub use protein_charge::{
    ChargeWindow, ProteinChargeError, ProteinChargeProfile, protein_charge_profile,
};
pub use protein_digest::{
    DigestProtease, DigestedPeptide, ProteinDigestError, digest_protein_sequence,
};
pub use protein_hydropathy::{
    HydropathyWindow, ProteinHydropathyError, ProteinHydropathyProfile,
    protein_hydropathy_profile,
};
pub use protein_isoelectric::{
    ProteinIsoelectricError, ProteinIsoelectricEstimate, TitratableResidueCounts,
    estimate_protein_isoelectric_point,
};
pub use residue_properties::{
    NucleotideBaseInfo, ProteinResidueProperty, nucleotide_base_info, nucleotide_base_infos,
    protein_residue_properties, protein_residue_property,
};
pub use revseq::{
    RevseqError, RevseqMode, reverse_complement_residues, reverse_residues,
    transform_sequence_record,
};
pub use sequence::SequenceRecord;
pub use strand::Strand;
pub use translation::{
    TranslationError, ambiguous_codon, backtranslate_ambiguous, backtranslate_representative,
    representative_codon, translate_dna_frame, translate_dna_strict,
};
