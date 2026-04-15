//! Tool descriptors and registry scaffolding for the governed EMBOSS-RS surface.

use emboss_core::{PLATFORM_IDENTITY, PlatformIdentity};

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
    /// Creates an empty tool registry for the initial workspace skeleton.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
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

#[cfg(test)]
mod tests {
    use super::ToolRegistry;

    #[test]
    fn binds_to_platform_identity() {
        assert_eq!(ToolRegistry::new().platform().binary_name, "emboss-rs");
    }
}
