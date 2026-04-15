//! Service responses for discovery and placeholder invocation.

use emboss_tools::ToolDescriptor;

use crate::context::ExecutionContext;
use crate::tool::ToolName;

/// Current execution state at the service boundary.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InvocationStatus {
    /// The request resolved to a known tool, but implementation is still pending.
    NotImplemented,
}

/// A typed service response for a resolved invocation request.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InvocationResponse {
    /// Execution context used for the request.
    pub context: ExecutionContext,
    /// Requested tool name.
    pub tool: ToolName,
    /// Resolved tool descriptor.
    pub descriptor: ToolDescriptor,
    /// Current invocation status.
    pub status: InvocationStatus,
}

impl InvocationResponse {
    /// Creates a placeholder response for a known but unimplemented tool.
    #[must_use]
    pub fn not_implemented(
        context: ExecutionContext,
        tool: ToolName,
        descriptor: ToolDescriptor,
    ) -> Self {
        Self {
            context,
            tool,
            descriptor,
            status: InvocationStatus::NotImplemented,
        }
    }
}
