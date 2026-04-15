//! Typed service requests for front-end-neutral invocation.

use emboss_providers::AcquisitionRequest;

use crate::context::ExecutionContext;
use crate::tool::ToolName;

/// A typed request to resolve or invoke a governed tool.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InvocationRequest {
    /// Execution context for the current request.
    pub context: ExecutionContext,
    /// Requested tool name.
    pub tool: ToolName,
    /// Optional input-resolution request associated with the invocation.
    pub input: Option<AcquisitionRequest>,
}

impl InvocationRequest {
    /// Creates a request for the supplied tool.
    #[must_use]
    pub fn new(context: ExecutionContext, tool: ToolName) -> Self {
        Self {
            context,
            tool,
            input: None,
        }
    }

    /// Attaches an acquisition request for later provider-backed resolution.
    #[must_use]
    pub fn with_input(mut self, input: AcquisitionRequest) -> Self {
        self.input = Some(input);
        self
    }

    /// Returns the requested tool.
    #[must_use]
    pub fn tool(&self) -> &ToolName {
        &self.tool
    }

    /// Returns the optional acquisition request associated with the invocation.
    #[must_use]
    pub fn input(&self) -> Option<&AcquisitionRequest> {
        self.input.as_ref()
    }
}
