//! Validation harness scaffolding for EMBOSS-RS.

use emboss_fixtures::FixtureCatalog;

/// Minimal validation context for future harness expansion.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ValidationContext {
    /// Fixture source used by the current validation run.
    pub fixtures: FixtureCatalog,
}

impl ValidationContext {
    /// Creates a validation context for the workspace fixture catalogue.
    #[must_use]
    pub fn new() -> Self {
        Self {
            fixtures: FixtureCatalog::workspace(),
        }
    }
}

impl Default for ValidationContext {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::ValidationContext;

    #[test]
    fn binds_fixture_catalogue() {
        assert_eq!(
            ValidationContext::new().fixtures.source,
            "workspace fixture catalogue"
        );
    }
}
