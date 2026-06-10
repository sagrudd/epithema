//! Rust-side bridge scaffolding for integration with the sister `epithemaR` project.
//!
//! This crate defines the Rust-facing contract seam that `epithemaR` consumes.
//! It projects stable, bridge-safe summaries and a small first analytical method
//! subset from `epithema-service`, `epithema-diagnostics`, and related crates
//! without duplicating service logic or embedding R package mechanics in the
//! workspace.

pub mod api;
pub mod conversion;
pub mod error;
pub mod health;
pub mod methods;
pub mod protocol;
pub mod types;
pub mod version;

pub use api::{
    bridge_version, health_check, health_check_with_service, list_tools, serialize_plot_contract,
    summarize_alignment, summarize_features, summarize_plot_contract, summarize_sequence,
    summarize_table_result, supports_plot_payload,
};
pub use error::BridgeErrorSummary;
pub use health::BridgeHealth;
pub use methods::{
    backtranslate_ambiguous_sequences, backtranslate_representative_sequences, charge_profile,
    compare_translation_sets, complexity_profile, composition_summary, consensus_ambiguous,
    consensus_simple, copy_features, count_gc_content, cut_sequences, degap_sequences,
    describe_sequence_file, describe_sequences, direct_match_sequences, extract_features,
    extract_sequences, fuzz_nucleotide, fuzz_protein, fuzz_translated_frames, mask_features,
    mask_sequences, new_sequence, not_sequence, nth_sequence, p_distance_for_sequences,
    pepstats_summary, reverse_sequences, sequence_count, skip_sequences, split_sequence_partitions,
    trim_sequences, union_sequence_collections, update_descriptions,
};
pub use protocol::{BridgeRequest, BridgeResponse};
pub use types::{
    BridgeAlignmentInput, BridgeAlignmentRowInput, BridgeAlignmentSummary, BridgeArtifactSummary,
    BridgeChargeProfile, BridgeChargeWindow, BridgeComplexityResult, BridgeComplexitySummary,
    BridgeComplexityWindow, BridgeCompositionRow, BridgeDescseqRow, BridgeDiagnosticSummary,
    BridgeDistanceMatrix, BridgeFeatureSummary, BridgeGcRow, BridgeIntervalInput,
    BridgeMatcherSummary, BridgeOperationStatus, BridgePatternHit, BridgePepstatsResult,
    BridgePepstatsSummaryRow, BridgePlotContract, BridgePlotSummary, BridgeProvenanceSummary,
    BridgeResultSummary, BridgeSequenceInput, BridgeSequenceRecord, BridgeSequenceSummary,
    BridgeTableSummary, BridgeToolSummary, BridgeTranslationCheck,
};
pub use version::BridgeVersion;
