//! Registry abstractions for known Epithema tools.

use epithema_tools::ToolDescriptor;

use crate::error::{ServiceError, duplicate_tool};
use crate::tool::ToolName;

/// Read-only registry behavior used by the service layer.
pub trait ToolCatalog {
    /// Returns all known tool descriptors.
    fn descriptors(&self) -> &[ToolDescriptor];

    /// Returns the descriptor for a tool when it exists.
    fn find(&self, tool: &ToolName) -> Option<&ToolDescriptor>;
}

/// Mutable service-owned registry of known tool descriptors.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ServiceRegistry {
    descriptors: Vec<ToolDescriptor>,
}

impl ServiceRegistry {
    /// Creates an empty service registry.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a tool descriptor, rejecting duplicate tool names.
    pub fn register(&mut self, descriptor: ToolDescriptor) -> Result<(), ServiceError> {
        let tool = ToolName::new(descriptor.name)?;

        if self.find(&tool).is_some() {
            return Err(duplicate_tool(&tool));
        }

        self.descriptors.push(descriptor);
        Ok(())
    }

    /// Returns the number of known tool descriptors.
    #[must_use]
    pub fn len(&self) -> usize {
        self.descriptors.len()
    }

    /// Returns true when no tools are registered.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.descriptors.is_empty()
    }
}

impl ToolCatalog for ServiceRegistry {
    fn descriptors(&self) -> &[ToolDescriptor] {
        &self.descriptors
    }

    fn find(&self, tool: &ToolName) -> Option<&ToolDescriptor> {
        self.descriptors
            .iter()
            .find(|descriptor| descriptor.name == tool.as_str())
    }
}

#[cfg(test)]
mod tests {
    use epithema_tools::ToolDescriptor;

    use super::{ServiceRegistry, ToolCatalog};

    #[test]
    fn rejects_duplicate_registration() {
        let mut registry = ServiceRegistry::new();
        let descriptor = ToolDescriptor::new("needle", "global alignment");

        registry
            .register(descriptor)
            .expect("first registration should succeed");

        assert!(registry.register(descriptor).is_err());
    }

    #[test]
    fn looks_up_registered_tools() {
        let mut registry = ServiceRegistry::new();
        registry
            .register(ToolDescriptor::new("seqret", "sequence conversion"))
            .expect("registration should succeed");

        let tool = crate::ToolName::new("seqret").expect("tool name should be valid");
        assert_eq!(
            registry.find(&tool).map(|descriptor| descriptor.name),
            Some("seqret")
        );
    }
}
