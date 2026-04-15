//! Typed service requests for front-end-neutral invocation.

use crate::context::ExecutionContext;
use crate::tool::ToolName;

/// A typed request to resolve or invoke a governed tool.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InvocationRequest {
    /// Execution context for the current request.
    pub context: ExecutionContext,
    /// Requested tool name.
    pub tool: ToolName,
}

impl InvocationRequest {
    /// Creates a request for the supplied tool.
    #[must_use]
    pub fn new(context: ExecutionContext, tool: ToolName) -> Self {
        Self { context, tool }
    }

    /// Returns the requested tool.
    #[must_use]
    pub fn tool(&self) -> &ToolName {
        &self.tool
    }
}
