//! Shared error construction for the EMBOSS-RS service layer.

use emboss_diagnostics::{ErrorCategory, PlatformError};

use crate::tool::ToolName;

/// Shared error type used by the service layer.
pub type ServiceError = PlatformError;

pub(crate) fn empty_tool_name() -> ServiceError {
    PlatformError::new(ErrorCategory::Validation, "tool name must not be empty")
        .with_code("service.tool_name.empty")
}

pub(crate) fn duplicate_tool(tool: &ToolName) -> ServiceError {
    PlatformError::new(
        ErrorCategory::Registry,
        format!("tool '{tool}' is already registered in the service registry"),
    )
    .with_code("service.registry.duplicate_tool")
}

pub(crate) fn unknown_tool(tool: &ToolName) -> ServiceError {
    PlatformError::new(
        ErrorCategory::Registry,
        format!("tool '{tool}' is not registered in the service registry"),
    )
    .with_code("service.registry.unknown_tool")
}
