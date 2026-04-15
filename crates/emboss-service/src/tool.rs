//! Tool identity types used at the service boundary.

use std::fmt::{Display, Formatter};

use crate::error::ServiceError;

/// A normalized tool identifier used at the service boundary.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct ToolName(String);

impl ToolName {
    /// Creates a tool identifier from a caller-supplied name.
    pub fn new(name: impl Into<String>) -> Result<Self, ServiceError> {
        let name = name.into();
        let normalized = name.trim();

        if normalized.is_empty() {
            return Err(ServiceError::EmptyToolName);
        }

        Ok(Self(normalized.to_owned()))
    }

    /// Returns the normalized tool name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for ToolName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::ToolName;

    #[test]
    fn rejects_empty_tool_names() {
        assert!(ToolName::new("   ").is_err());
    }
}
