//! Error types for the shared EMBOSS-RS service layer.

use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::tool::ToolName;

/// Service-layer errors for runtime discovery and invocation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ServiceError {
    /// Tool names must not be empty.
    EmptyToolName,
    /// Attempted to register the same tool more than once.
    DuplicateTool {
        /// Tool name that caused the duplicate registration.
        tool: ToolName,
    },
    /// Requested a tool that the current registry does not know.
    UnknownTool {
        /// Tool name that could not be resolved.
        tool: ToolName,
    },
}

impl Display for ServiceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyToolName => write!(f, "tool name must not be empty"),
            Self::DuplicateTool { tool } => {
                write!(
                    f,
                    "tool '{tool}' is already registered in the service registry"
                )
            }
            Self::UnknownTool { tool } => {
                write!(f, "tool '{tool}' is not registered in the service registry")
            }
        }
    }
}

impl Error for ServiceError {}
