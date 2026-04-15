//! Shared execution and service-layer scaffolding for EMBOSS-RS.

use emboss_tools::ToolRegistry;

/// Minimal shared runtime state for the initial CLI skeleton.
#[derive(Clone, Debug, Default)]
pub struct ServiceRuntime {
    registry: ToolRegistry,
}

impl ServiceRuntime {
    /// Creates a runtime with the current tool registry.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a human-readable status line for the CLI.
    #[must_use]
    pub fn status_line(&self) -> String {
        format!(
            "{} workspace skeleton active; {} tools registered",
            self.registry.platform().invocation_pattern(),
            self.registry.tools().len()
        )
    }

    /// Returns the current tool registry.
    #[must_use]
    pub fn registry(&self) -> &ToolRegistry {
        &self.registry
    }
}

#[cfg(test)]
mod tests {
    use super::ServiceRuntime;

    #[test]
    fn reports_empty_registry_status() {
        assert!(
            ServiceRuntime::new()
                .status_line()
                .contains("0 tools registered")
        );
    }
}
