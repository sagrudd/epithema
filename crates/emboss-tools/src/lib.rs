//! Governed EMBOSS-RS tool descriptors and shared tool-family implementations.

use emboss_core::{PLATFORM_IDENTITY, PlatformIdentity};

pub mod sequence_stream;
pub mod sequence_transform;

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
        sequence_stream::NEWSEQ_DESCRIPTOR,
        sequence_stream::SEQCOUNT_DESCRIPTOR,
        sequence_stream::NOTSEQ_DESCRIPTOR,
        sequence_stream::NTHSEQ_DESCRIPTOR,
        sequence_stream::SKIPSEQ_DESCRIPTOR,
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
                "newseq",
                "seqcount",
                "notseq",
                "nthseq",
                "skipseq",
                "extractseq",
                "cutseq",
                "union",
                "splitter",
            ]
        );
    }
}
