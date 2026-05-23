//! Governed EMBOSS-RS tool descriptors and shared tool-family implementations.

use emboss_core::{PLATFORM_IDENTITY, PlatformIdentity};

pub mod alignment_analysis;
pub mod alignment_tools;
pub mod archive_tools;
pub mod codon_tools;
pub mod feature_tools;
pub mod pairwise_alignment;
pub mod pattern_tools;
pub mod protein_plots;
pub mod restriction_tools;
pub mod retrieval_tools;
pub mod sequence_edit;
pub mod sequence_stats;
pub mod sequence_stream;
pub mod sequence_transform;
pub mod translation_tools;

/// Metadata for a governed tool entry.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ToolDescriptor {
    /// Stable tool name as exposed through `emboss-rs <tool>`.
    pub name: &'static str,
    /// Stable governed tool family used for discovery and documentation grouping.
    pub family: &'static str,
    /// Short summary for help and documentation generation.
    pub summary: &'static str,
}

impl ToolDescriptor {
    /// Creates a tool descriptor from stable identity metadata.
    #[must_use]
    pub const fn new(name: &'static str, summary: &'static str) -> Self {
        Self {
            name,
            family: "ungrouped",
            summary,
        }
    }

    /// Associates the descriptor with its governed tool family.
    #[must_use]
    pub const fn with_family(mut self, family: &'static str) -> Self {
        self.family = family;
        self
    }
}

/// Registry of governed tools for the current runtime.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ToolRegistry {
    tools: Vec<ToolDescriptor>,
}

impl ToolRegistry {
    /// Creates a tool registry containing the currently implemented cohort.
    #[must_use]
    pub fn new() -> Self {
        Self {
            tools: governed_tool_descriptors().to_vec(),
        }
    }

    /// Returns the currently registered tool descriptors.
    #[must_use]
    pub fn tools(&self) -> &[ToolDescriptor] {
        &self.tools
    }

    /// Returns the platform identity associated with this registry.
    #[must_use]
    pub fn platform(&self) -> PlatformIdentity {
        PLATFORM_IDENTITY
    }
}

/// Returns the descriptors for currently governed and implemented tools.
#[must_use]
pub const fn governed_tool_descriptors() -> &'static [ToolDescriptor] {
    &[
        alignment_tools::ALIGNCOPY_DESCRIPTOR,
        alignment_tools::ALIGNCOPYPAIR_DESCRIPTOR,
        alignment_tools::DIFFSEQ_DESCRIPTOR,
        alignment_tools::EDIALIGN_DESCRIPTOR,
        alignment_tools::INFOALIGN_DESCRIPTOR,
        alignment_tools::EXTRACTALIGN_DESCRIPTOR,
        alignment_tools::NTHSEQSET_DESCRIPTOR,
        archive_tools::RUNINFO_DESCRIPTOR,
        archive_tools::RUNGET_DESCRIPTOR,
        alignment_analysis::MATCHER_DESCRIPTOR,
        alignment_analysis::DISTMAT_DESCRIPTOR,
        alignment_analysis::CONS_DESCRIPTOR,
        alignment_analysis::CONSAMBIG_DESCRIPTOR,
        pairwise_alignment::NEEDLE_DESCRIPTOR,
        pairwise_alignment::NEEDLEALL_DESCRIPTOR,
        pairwise_alignment::WATER_DESCRIPTOR,
        retrieval_tools::SEQRET_DESCRIPTOR,
        retrieval_tools::REFSEQGET_DESCRIPTOR,
        sequence_stream::NEWSEQ_DESCRIPTOR,
        sequence_stream::MAKENUCSEQ_DESCRIPTOR,
        sequence_stream::MAKEPROTSEQ_DESCRIPTOR,
        sequence_stream::SEQCOUNT_DESCRIPTOR,
        sequence_stream::NOTSEQ_DESCRIPTOR,
        sequence_stream::NTHSEQ_DESCRIPTOR,
        sequence_stream::SKIPSEQ_DESCRIPTOR,
        sequence_stream::LISTOR_DESCRIPTOR,
        sequence_stream::SKIPREDUNDANT_DESCRIPTOR,
        sequence_edit::BIOSED_DESCRIPTOR,
        sequence_edit::DEGAPSEQ_DESCRIPTOR,
        sequence_edit::REVSEQ_DESCRIPTOR,
        sequence_edit::MSBAR_DESCRIPTOR,
        sequence_edit::TRIMEST_DESCRIPTOR,
        sequence_edit::TRIMSEQ_DESCRIPTOR,
        sequence_edit::DESCSEQ_DESCRIPTOR,
        sequence_edit::VECTORSTRIP_DESCRIPTOR,
        feature_tools::MASKSEQ_DESCRIPTOR,
        feature_tools::MASKAMBIGNUC_DESCRIPTOR,
        feature_tools::MASKAMBIGPROT_DESCRIPTOR,
        feature_tools::MASKFEAT_DESCRIPTOR,
        feature_tools::EXTRACTFEAT_DESCRIPTOR,
        feature_tools::FEATCOPY_DESCRIPTOR,
        feature_tools::CODERET_DESCRIPTOR,
        feature_tools::FEATMERGE_DESCRIPTOR,
        feature_tools::FEATREPORT_DESCRIPTOR,
        feature_tools::FEATTEXT_DESCRIPTOR,
        feature_tools::SPLITSOURCE_DESCRIPTOR,
        feature_tools::TWOFEAT_DESCRIPTOR,
        codon_tools::CAI_DESCRIPTOR,
        codon_tools::CHIPS_DESCRIPTOR,
        codon_tools::CUSP_DESCRIPTOR,
        codon_tools::CODCMP_DESCRIPTOR,
        codon_tools::CODCOPY_DESCRIPTOR,
        pattern_tools::DREG_DESCRIPTOR,
        pattern_tools::EINVERTED_DESCRIPTOR,
        pattern_tools::FUZZNUC_DESCRIPTOR,
        pattern_tools::FUZZPRO_DESCRIPTOR,
        pattern_tools::FUZZTRAN_DESCRIPTOR,
        pattern_tools::PALINDROME_DESCRIPTOR,
        pattern_tools::PREG_DESCRIPTOR,
        pattern_tools::PATMATDB_DESCRIPTOR,
        pattern_tools::SEQMATCHALL_DESCRIPTOR,
        pattern_tools::WORDMATCH_DESCRIPTOR,
        pattern_tools::WORDFINDER_DESCRIPTOR,
        protein_plots::CHARGE_DESCRIPTOR,
        protein_plots::HMOMENT_DESCRIPTOR,
        protein_plots::OCTANOL_DESCRIPTOR,
        protein_plots::PEPWINDOW_DESCRIPTOR,
        restriction_tools::RECODER_DESCRIPTOR,
        restriction_tools::SILENT_DESCRIPTOR,
        sequence_stats::AAINDEXEXTRACT_DESCRIPTOR,
        sequence_stats::COMPLEX_DESCRIPTOR,
        sequence_stats::COMPSEQ_DESCRIPTOR,
        sequence_stats::DAN_DESCRIPTOR,
        sequence_stats::GEECEE_DESCRIPTOR,
        sequence_stats::INFOBASE_DESCRIPTOR,
        sequence_stats::INFOSEQ_DESCRIPTOR,
        sequence_stats::INFORESIDUE_DESCRIPTOR,
        sequence_stats::IEP_DESCRIPTOR,
        sequence_stats::ODDCOMP_DESCRIPTOR,
        sequence_stats::PEPDIGEST_DESCRIPTOR,
        sequence_stats::PEPSTATS_DESCRIPTOR,
        sequence_stats::WORDCOUNT_DESCRIPTOR,
        translation_tools::BACKTRANSEQ_DESCRIPTOR,
        translation_tools::BACKTRANAMBIG_DESCRIPTOR,
        translation_tools::CHECKTRANS_DESCRIPTOR,
        translation_tools::TRANSEQ_DESCRIPTOR,
        translation_tools::GETORF_DESCRIPTOR,
        translation_tools::PRETTYSEQ_DESCRIPTOR,
        translation_tools::TRANALIGN_DESCRIPTOR,
        sequence_transform::EXTRACTSEQ_DESCRIPTOR,
        sequence_transform::CUTSEQ_DESCRIPTOR,
        sequence_transform::UNION_DESCRIPTOR,
        sequence_transform::PASTESEQ_DESCRIPTOR,
        sequence_transform::SPLITTER_DESCRIPTOR,
        sequence_transform::MERGER_DESCRIPTOR,
        sequence_transform::MEGAMERGER_DESCRIPTOR,
        sequence_transform::SIZESEQ_DESCRIPTOR,
        sequence_transform::SHUFFLESEQ_DESCRIPTOR,
    ]
}

#[cfg(test)]
mod tests {
    use super::{ToolRegistry, governed_tool_descriptors};

    #[test]
    fn binds_to_platform_identity() {
        assert_eq!(ToolRegistry::new().platform().binary_name, "emboss-rs");
    }

    #[test]
    fn exposes_sequence_stream_cohort() {
        let names: Vec<_> = governed_tool_descriptors()
            .iter()
            .map(|descriptor| descriptor.name)
            .collect();

        assert_eq!(
            names,
            vec![
            "aligncopy",
            "aligncopypair",
            "diffseq",
            "edialign",
            "infoalign",
            "extractalign",
            "nthseqset",
                "runinfo",
                "runget",
                "matcher",
                "distmat",
                "cons",
                "consambig",
                "needle",
                "needleall",
                "water",
                "seqret",
                "refseqget",
                "newseq",
                "makenucseq",
                "makeprotseq",
                "seqcount",
                "notseq",
                "nthseq",
                "skipseq",
                "listor",
                "skipredundant",
                "biosed",
                "degapseq",
                "revseq",
                "msbar",
                "trimest",
                "trimseq",
                "descseq",
                "vectorstrip",
                "maskseq",
                "maskambignuc",
                "maskambigprot",
                "maskfeat",
                "extractfeat",
                "featcopy",
                "coderet",
                "featmerge",
                "featreport",
                "feattext",
                "splitsource",
                "twofeat",
                "cai",
                "chips",
                "cusp",
                "codcmp",
                "codcopy",
                "dreg",
                "einverted",
                "fuzznuc",
                "fuzzpro",
                "fuzztran",
                "palindrome",
                "preg",
                "patmatdb",
                "seqmatchall",
                "wordmatch",
                "wordfinder",
                "charge",
                "hmoment",
                "octanol",
                "pepwindow",
                "recoder",
                "silent",
                "aaindexextract",
                "complex",
                "compseq",
                "dan",
                "geecee",
                "infobase",
                "infoseq",
                "inforesidue",
                "iep",
                "oddcomp",
                "pepdigest",
                "pepstats",
                "wordcount",
                "backtranseq",
                "backtranambig",
                "checktrans",
                "transeq",
                "getorf",
                "prettyseq",
                "tranalign",
                "extractseq",
                "cutseq",
                "union",
                "pasteseq",
                "splitter",
                "merger",
                "megamerger",
                "sizeseq",
                "shuffleseq",
            ]
        );
    }
}
