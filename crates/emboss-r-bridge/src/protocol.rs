//! JSON request and response types for the developer-facing bridge driver.

use serde::{Deserialize, Serialize};

use crate::types::{
    BridgeAlignmentInput, BridgeChargeProfile, BridgeComplexityResult, BridgeCompositionRow,
    BridgeDistanceMatrix, BridgeGcRow, BridgeMatcherSummary, BridgePatternHit,
    BridgePepstatsResult, BridgeSequenceInput, BridgeSequenceRecord, BridgeTranslationCheck,
};

/// JSON bridge request envelope.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "method", content = "params", rename_all = "snake_case")]
pub enum BridgeRequest {
    /// Create one validated sequence record.
    NewSequence {
        /// Input sequence payload.
        record: BridgeSequenceInput,
    },
    /// Count sequence records in a supplied in-memory collection.
    SequenceCount {
        /// Ordered in-memory sequence records.
        records: Vec<BridgeSequenceInput>,
    },
    /// Select the 1-based Nth sequence record.
    NthSequence {
        /// Ordered in-memory sequence records.
        records: Vec<BridgeSequenceInput>,
        /// 1-based index to select.
        index: usize,
    },
    /// Skip the first N sequence records.
    SkipSequences {
        /// Ordered in-memory sequence records.
        records: Vec<BridgeSequenceInput>,
        /// Number of leading records to skip.
        count: usize,
    },
    /// Return all sequence records except the supplied 1-based index.
    NotSequence {
        /// Ordered in-memory sequence records.
        records: Vec<BridgeSequenceInput>,
        /// 1-based index to exclude.
        index: usize,
    },
    /// Extract the same 1-based inclusive interval from each record.
    ExtractSequences {
        /// Ordered in-memory sequence records.
        records: Vec<BridgeSequenceInput>,
        /// 1-based inclusive start.
        start: usize,
        /// 1-based inclusive end.
        end: usize,
    },
    /// Cut each sequence after the supplied interior position.
    CutSequences {
        /// Ordered in-memory sequence records.
        records: Vec<BridgeSequenceInput>,
        /// 1-based cut position.
        cut_position: usize,
    },
    /// Concatenate multiple sequence collections in deterministic order.
    UnionSequenceCollections {
        /// Ordered sequence collections.
        collections: Vec<Vec<BridgeSequenceInput>>,
    },
    /// Partition one sequence collection into fixed-size chunks.
    SplitSequencePartitions {
        /// Ordered in-memory sequence records.
        records: Vec<BridgeSequenceInput>,
        /// Number of records per partition.
        chunk_size: usize,
    },
    /// Remove gap characters from sequences.
    DegapSequences {
        /// Ordered in-memory sequence records.
        records: Vec<BridgeSequenceInput>,
    },
    /// Reverse sequence content without reverse-complement logic.
    ReverseSequences {
        /// Ordered in-memory sequence records.
        records: Vec<BridgeSequenceInput>,
    },
    /// Trim explicit residue counts from sequence ends.
    TrimSequences {
        /// Ordered in-memory sequence records.
        records: Vec<BridgeSequenceInput>,
        /// Left trim count.
        left_trim: usize,
        /// Right trim count.
        right_trim: usize,
    },
    /// Replace or clear sequence descriptions.
    UpdateDescriptions {
        /// Ordered in-memory sequence records.
        records: Vec<BridgeSequenceInput>,
        /// Optional replacement description.
        description: Option<String>,
        /// Whether existing descriptions should be cleared.
        clear: bool,
    },
    /// Representative codon back-translation.
    BacktranslateRepresentative {
        /// Ordered in-memory protein records.
        records: Vec<BridgeSequenceInput>,
    },
    /// Ambiguous codon back-translation.
    BacktranslateAmbiguous {
        /// Ordered in-memory protein records.
        records: Vec<BridgeSequenceInput>,
    },
    /// Translation-vs-protein comparison by paired order.
    CompareTranslationSets {
        /// Ordered nucleotide coding sequences.
        nucleotide_records: Vec<BridgeSequenceInput>,
        /// Ordered expected proteins.
        protein_records: Vec<BridgeSequenceInput>,
    },
    /// Nucleotide pattern scan.
    FuzzNucleotide {
        /// Ordered in-memory nucleotide records.
        records: Vec<BridgeSequenceInput>,
        /// Pattern text.
        pattern: String,
    },
    /// Protein pattern scan.
    FuzzProtein {
        /// Ordered in-memory protein records.
        records: Vec<BridgeSequenceInput>,
        /// Pattern text.
        pattern: String,
    },
    /// Forward translated-frame protein pattern scan.
    FuzzTranslatedFrames {
        /// Ordered in-memory nucleotide records.
        records: Vec<BridgeSequenceInput>,
        /// Protein pattern text.
        pattern: String,
    },
    /// Residue composition summary.
    CompositionSummary {
        /// Ordered in-memory sequence records.
        records: Vec<BridgeSequenceInput>,
    },
    /// GC summary.
    CountGcContent {
        /// Ordered in-memory sequence records.
        records: Vec<BridgeSequenceInput>,
    },
    /// Protein statistics summary.
    PepstatsSummary {
        /// Ordered in-memory protein records.
        records: Vec<BridgeSequenceInput>,
    },
    /// Linguistic complexity summary.
    ComplexityProfile {
        /// One in-memory nucleotide record.
        record: BridgeSequenceInput,
        /// Inclusive minimum k.
        k_min: usize,
        /// Inclusive maximum k.
        k_max: usize,
        /// Optional sliding-window length.
        window: Option<usize>,
        /// Optional sliding-window step size.
        step: Option<usize>,
    },
    /// Ungapped direct match summary.
    DirectMatchSequences {
        /// Query record.
        query: BridgeSequenceInput,
        /// Target record.
        target: BridgeSequenceInput,
    },
    /// Equal-length p-distance matrix.
    PDistanceForSequences {
        /// Ordered in-memory sequence records.
        records: Vec<BridgeSequenceInput>,
    },
    /// Simple consensus from an in-memory alignment.
    ConsensusSimple {
        /// Alignment input.
        alignment: BridgeAlignmentInput,
    },
    /// Ambiguity-aware consensus from an in-memory alignment.
    ConsensusAmbiguous {
        /// Alignment input.
        alignment: BridgeAlignmentInput,
    },
    /// Compute a sliding-window protein charge profile.
    ChargeProfile {
        /// Input protein sequence.
        record: BridgeSequenceInput,
        /// Sliding-window length.
        window: usize,
        /// Sliding-window step size.
        step: usize,
    },
}

/// JSON bridge response envelope.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "method", content = "result", rename_all = "snake_case")]
pub enum BridgeResponse {
    /// New sequence response.
    NewSequence {
        /// Created validated record.
        record: BridgeSequenceRecord,
    },
    /// Sequence count response.
    SequenceCount {
        /// Counted sequence records.
        count: usize,
    },
    /// Nth sequence response.
    NthSequence {
        /// Selected record.
        record: BridgeSequenceRecord,
    },
    /// Skip sequence response.
    SkipSequences {
        /// Remaining records after skipping.
        records: Vec<BridgeSequenceRecord>,
    },
    /// Not sequence response.
    NotSequence {
        /// Remaining records after exclusion.
        records: Vec<BridgeSequenceRecord>,
    },
    /// Extract sequence response.
    ExtractSequences {
        /// Extracted records.
        records: Vec<BridgeSequenceRecord>,
    },
    /// Cut sequence response.
    CutSequences {
        /// Left and right fragments in input order.
        records: Vec<BridgeSequenceRecord>,
    },
    /// Union response.
    UnionSequenceCollections {
        /// Concatenated records.
        records: Vec<BridgeSequenceRecord>,
    },
    /// Splitter response.
    SplitSequencePartitions {
        /// Deterministic partitions.
        partitions: Vec<Vec<BridgeSequenceRecord>>,
    },
    /// Degap response.
    DegapSequences {
        /// Degapped records.
        records: Vec<BridgeSequenceRecord>,
    },
    /// Reverse response.
    ReverseSequences {
        /// Reversed records.
        records: Vec<BridgeSequenceRecord>,
    },
    /// Trim response.
    TrimSequences {
        /// Trimmed records.
        records: Vec<BridgeSequenceRecord>,
    },
    /// Description update response.
    UpdateDescriptions {
        /// Updated records.
        records: Vec<BridgeSequenceRecord>,
    },
    /// Representative back-translation response.
    BacktranslateRepresentative {
        /// Back-translated nucleotide records.
        records: Vec<BridgeSequenceRecord>,
    },
    /// Ambiguous back-translation response.
    BacktranslateAmbiguous {
        /// Back-translated nucleotide records.
        records: Vec<BridgeSequenceRecord>,
    },
    /// Translation check response.
    CompareTranslationSets {
        /// Paired comparison rows.
        cases: Vec<BridgeTranslationCheck>,
    },
    /// Nucleotide pattern-scan response.
    FuzzNucleotide {
        /// Ordered hit rows.
        hits: Vec<BridgePatternHit>,
    },
    /// Protein pattern-scan response.
    FuzzProtein {
        /// Ordered hit rows.
        hits: Vec<BridgePatternHit>,
    },
    /// Translated-frame pattern-scan response.
    FuzzTranslatedFrames {
        /// Ordered hit rows.
        hits: Vec<BridgePatternHit>,
    },
    /// Composition response.
    CompositionSummary {
        /// Long-form composition rows.
        rows: Vec<BridgeCompositionRow>,
    },
    /// GC response.
    CountGcContent {
        /// GC summary rows.
        rows: Vec<BridgeGcRow>,
    },
    /// Pepstats response.
    PepstatsSummary {
        /// Summary and composition rows.
        result: BridgePepstatsResult,
    },
    /// Complexity response.
    ComplexityProfile {
        /// Complexity result.
        result: BridgeComplexityResult,
    },
    /// Direct match response.
    DirectMatchSequences {
        /// Matcher summary row.
        summary: BridgeMatcherSummary,
    },
    /// Distance matrix response.
    PDistanceForSequences {
        /// Pairwise p-distance matrix.
        matrix: BridgeDistanceMatrix,
    },
    /// Simple consensus response.
    ConsensusSimple {
        /// Consensus sequence.
        record: BridgeSequenceRecord,
    },
    /// Ambiguous consensus response.
    ConsensusAmbiguous {
        /// Consensus sequence.
        record: BridgeSequenceRecord,
    },
    /// Charge-profile response.
    ChargeProfile {
        /// Charge-profile payload.
        profile: BridgeChargeProfile,
    },
}
