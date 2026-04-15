//! Service responses for discovery and placeholder invocation.

use emboss_diagnostics::ExecutionReport;
use emboss_tools::ToolDescriptor;

use crate::context::ExecutionContext;
use crate::result::{MethodResult, ResultPayload, ResultSummary};
use crate::tool::ToolName;

/// Current execution state at the service boundary.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InvocationStatus {
    /// The request resolved to a known tool, but implementation is still pending.
    NotImplemented,
    /// The request completed with a real tool execution path.
    Completed,
}

/// A typed service response for a resolved invocation request.
#[derive(Clone, Debug, PartialEq)]
pub struct InvocationResponse {
    /// Execution context used for the request.
    pub context: ExecutionContext,
    /// Requested tool name.
    pub tool: ToolName,
    /// Resolved tool descriptor.
    pub descriptor: ToolDescriptor,
    /// Current invocation status.
    pub status: InvocationStatus,
    /// Structured execution report for the invocation boundary.
    pub report: ExecutionReport,
    /// Front-end-neutral method result envelope for the invocation.
    pub result: MethodResult,
}

impl InvocationResponse {
    /// Creates a placeholder response for a known but unimplemented tool.
    #[must_use]
    pub fn not_implemented(
        context: ExecutionContext,
        tool: ToolName,
        descriptor: ToolDescriptor,
        report: ExecutionReport,
    ) -> Self {
        let summary = ResultSummary::new(format!("{} not implemented", descriptor.name))
            .with_line(descriptor.summary)
            .with_line("Execution path is registered but implementation is pending.");
        let result = MethodResult::new(tool.clone(), ResultPayload::Empty, summary, report.clone());
        Self {
            context,
            tool,
            descriptor,
            status: InvocationStatus::NotImplemented,
            report,
            result,
        }
    }

    /// Creates a successful invocation response backed by a real result.
    #[must_use]
    pub fn completed(
        context: ExecutionContext,
        tool: ToolName,
        descriptor: ToolDescriptor,
        report: ExecutionReport,
        result: MethodResult,
    ) -> Self {
        Self {
            context,
            tool,
            descriptor,
            status: InvocationStatus::Completed,
            report,
            result,
        }
    }
}
