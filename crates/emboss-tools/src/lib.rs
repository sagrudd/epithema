//! Governed EMBOSS-RS tool descriptors and shared tool-family implementations.

use emboss_core::{PLATFORM_IDENTITY, PlatformIdentity};

pub mod alignment_analysis;
pub mod alignment_tools;
pub mod codon_tools;
pub mod feature_tools;
pub mod pairwise_alignment;
pub mod pattern_tools;
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
    /// Short summary for help and documentation generation.
    pub summary: &'static str,
}

impl ToolDescriptor {
    /// Creates a tool descriptor from stable identity metadata.
    #[must_use]
    pub const fn new(name: &'static str, summary: &'static str) -> Self {
        Self { name, summary }
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
        alignment_tools::INFOALIGN_DESCRIPTOR,
        alignment_tools::EXTRACTALIGN_DESCRIPTOR,
        alignment_analysis::MATCHER_DESCRIPTOR,
        alignment_analysis::DISTMAT_DESCRIPTOR,
        alignment_analysis::CONS_DESCRIPTOR,
        alignment_analysis::CONSAMBIG_DESCRIPTOR,
        pairwise_alignment::NEEDLE_DESCRIPTOR,
        pairwise_alignment::NEEDLEALL_DESCRIPTOR,
        sequence_stream::NEWSEQ_DESCRIPTOR,
        sequence_stream::SEQCOUNT_DESCRIPTOR,
        sequence_stream::NOTSEQ_DESCRIPTOR,
        sequence_stream::NTHSEQ_DESCRIPTOR,
        sequence_stream::SKIPSEQ_DESCRIPTOR,
        sequence_edit::DEGAPSEQ_DESCRIPTOR,
        sequence_edit::REVSEQ_DESCRIPTOR,
        sequence_edit::TRIMSEQ_DESCRIPTOR,
        sequence_edit::DESCSEQ_DESCRIPTOR,
        feature_tools::MASKSEQ_DESCRIPTOR,
        feature_tools::MASKFEAT_DESCRIPTOR,
        feature_tools::EXTRACTFEAT_DESCRIPTOR,
        feature_tools::FEATCOPY_DESCRIPTOR,
        codon_tools::CAI_DESCRIPTOR,
        codon_tools::CHIPS_DESCRIPTOR,
        codon_tools::CODCMP_DESCRIPTOR,
        codon_tools::CODCOPY_DESCRIPTOR,
        pattern_tools::FUZZNUC_DESCRIPTOR,
        pattern_tools::FUZZPRO_DESCRIPTOR,
        pattern_tools::FUZZTRAN_DESCRIPTOR,
        sequence_stats::COMPLEX_DESCRIPTOR,
        sequence_stats::COMPSEQ_DESCRIPTOR,
        sequence_stats::GEECEE_DESCRIPTOR,
        sequence_stats::PEPSTATS_DESCRIPTOR,
        translation_tools::BACKTRANSEQ_DESCRIPTOR,
        translation_tools::BACKTRANAMBIG_DESCRIPTOR,
        translation_tools::CHECKTRANS_DESCRIPTOR,
        sequence_transform::EXTRACTSEQ_DESCRIPTOR,
        sequence_transform::CUTSEQ_DESCRIPTOR,
        sequence_transform::UNION_DESCRIPTOR,
        sequence_transform::SPLITTER_DESCRIPTOR,
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
                "infoalign",
                "extractalign",
                "matcher",
                "distmat",
                "cons",
                "consambig",
                "needle",
                "needleall",
                "newseq",
                "seqcount",
                "notseq",
                "nthseq",
                "skipseq",
                "degapseq",
                "revseq",
                "trimseq",
                "descseq",
                "maskseq",
                "maskfeat",
                "extractfeat",
                "featcopy",
                "cai",
                "chips",
                "codcmp",
                "codcopy",
                "fuzznuc",
                "fuzzpro",
                "fuzztran",
                "complex",
                "compseq",
                "geecee",
                "pepstats",
                "backtranseq",
                "backtranambig",
                "checktrans",
                "extractseq",
                "cutseq",
                "union",
                "splitter",
            ]
        );
    }
}
