//! Documentation generation and autodoc scaffolding for EMBOSS-RS.

use emboss_fixtures::FixtureCatalog;
use emboss_tools::ToolRegistry;

/// Minimal documentation planning record for future autodoc work.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DocumentationPlan {
    /// Source of fixture and example material.
    pub fixture_source: &'static str,
    /// Number of registered tools in scope.
    pub tool_count: usize,
}

/// Builds the initial documentation planning record.
#[must_use]
pub fn initial_plan() -> DocumentationPlan {
    let registry = ToolRegistry::new();
    let fixtures = FixtureCatalog::workspace();

    DocumentationPlan {
        fixture_source: fixtures.source,
        tool_count: registry.tools().len(),
    }
}

#[cfg(test)]
mod tests {
    use super::initial_plan;

    #[test]
    fn starts_with_empty_registry() {
        assert_eq!(initial_plan().tool_count, 0);
    }
}
