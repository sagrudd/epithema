//! Fixture catalogue scaffolding for validation and documentation support.

/// Minimal fixture catalogue metadata.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FixtureCatalog {
    /// Human-readable source description.
    pub source: &'static str,
}

impl FixtureCatalog {
    /// Returns the initial workspace fixture catalogue marker.
    #[must_use]
    pub fn workspace() -> Self {
        Self {
            source: "workspace fixture catalogue",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::FixtureCatalog;

    #[test]
    fn exposes_workspace_catalogue() {
        assert_eq!(
            FixtureCatalog::workspace().source,
            "workspace fixture catalogue"
        );
    }
}
