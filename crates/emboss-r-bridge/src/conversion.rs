//! Conversion helpers from internal workspace types to bridge-safe summaries.

use emboss_core::PLATFORM_IDENTITY;
use emboss_diagnostics::{Diagnostic, PlatformError};
use emboss_service::{EmbossService, MethodResult};
use emboss_tools::ToolDescriptor;

use crate::error::BridgeErrorSummary;
use crate::health::BridgeHealth;
use crate::types::{
    BridgeDiagnosticSummary, BridgeOperationStatus, BridgeResultSummary, BridgeToolSummary,
};
use crate::version::BridgeVersion;

impl From<ToolDescriptor> for BridgeToolSummary {
    fn from(value: ToolDescriptor) -> Self {
        Self {
            name: value.name.to_owned(),
            summary: value.summary.to_owned(),
        }
    }
}

impl From<&ToolDescriptor> for BridgeToolSummary {
    fn from(value: &ToolDescriptor) -> Self {
        Self::from(*value)
    }
}

impl From<&Diagnostic> for BridgeDiagnosticSummary {
    fn from(value: &Diagnostic) -> Self {
        Self {
            severity: value.severity.to_string(),
            code: value.code().map(ToOwned::to_owned),
            message: value.message().to_owned(),
            context: value.context().map(ToOwned::to_owned),
            location: value.location().map(|location| location.scope().to_owned()),
        }
    }
}

impl From<&MethodResult> for BridgeResultSummary {
    fn from(value: &MethodResult) -> Self {
        Self {
            tool: value.tool.as_str().to_owned(),
            payload_kind: value.payload.kind_label().to_owned(),
            title: value.summary.title.clone(),
            lines: value.summary.lines.clone(),
            artifact_count: value.artifacts.len(),
            diagnostic_count: value.report.diagnostics().len(),
        }
    }
}

impl From<&PlatformError> for BridgeErrorSummary {
    fn from(value: &PlatformError) -> Self {
        Self {
            category: value.category().to_string(),
            code: value.code().map(ToOwned::to_owned),
            message: value.message().to_owned(),
            detail: value.detail().map(ToOwned::to_owned),
        }
    }
}

/// Projects stable version metadata for the bridge surface.
#[must_use]
pub fn project_version() -> BridgeVersion {
    BridgeVersion {
        package_version: env!("CARGO_PKG_VERSION").to_owned(),
        binary_name: PLATFORM_IDENTITY.binary_name.to_owned(),
        sister_package: PLATFORM_IDENTITY.sister_project.to_owned(),
        plot_backend: PLATFORM_IDENTITY.plot_backend.to_owned(),
    }
}

/// Projects bridge-facing health information from a shared service instance.
#[must_use]
pub fn project_health(service: &EmbossService) -> BridgeHealth {
    BridgeHealth {
        sister_package: PLATFORM_IDENTITY.sister_project.to_owned(),
        plot_backend: PLATFORM_IDENTITY.plot_backend.to_owned(),
        tools_registered: service.descriptors().len(),
        providers_configured: service.providers().len(),
        service_status: service.status_line(),
        operation_status: BridgeOperationStatus {
            ok: true,
            message: "Rust bridge scaffold is ready for future emboss-r bindings".to_owned(),
        },
    }
}

#[cfg(test)]
mod tests {
    use emboss_diagnostics::{
        Diagnostic, DiagnosticLocation, ErrorCategory, ExecutionContext, ExecutionOutcome,
        ExecutionReport, InvocationOrigin, OutcomeStatus, PlatformError, Severity,
    };
    use emboss_service::{EmbossService, MethodResult, ResultPayload, ResultSummary, ToolName};
    use emboss_tools::ToolDescriptor;

    use super::{project_health, project_version};
    use crate::types::{BridgeDiagnosticSummary, BridgeResultSummary, BridgeToolSummary};

    #[test]
    fn projects_version_metadata() {
        let version = project_version();
        assert_eq!(version.binary_name, "emboss-rs");
        assert_eq!(version.sister_package, "emboss-r");
        assert_eq!(version.plot_backend, "R");
    }

    #[test]
    fn converts_tool_descriptor_to_summary() {
        let summary = BridgeToolSummary::from(ToolDescriptor::new("needle", "global alignment"));
        assert_eq!(summary.name, "needle");
        assert_eq!(summary.summary, "global alignment");
    }

    #[test]
    fn converts_diagnostic_to_bridge_summary() {
        let diagnostic = Diagnostic::new(Severity::Warning, "missing provenance note")
            .with_code("bridge.provenance.missing")
            .with_context("autodoc import")
            .with_location(DiagnosticLocation::new("docs/source"));

        let summary = BridgeDiagnosticSummary::from(&diagnostic);
        assert_eq!(summary.severity, "warning");
        assert_eq!(summary.code.as_deref(), Some("bridge.provenance.missing"));
        assert_eq!(summary.location.as_deref(), Some("docs/source"));
    }

    #[test]
    fn converts_platform_error_to_bridge_summary() {
        let error = PlatformError::new(
            ErrorCategory::NotImplemented,
            "bridge method not implemented",
        )
        .with_code("bridge.method.not_implemented")
        .with_detail("tool dispatch projection is deferred");

        let summary = crate::error::BridgeErrorSummary::from(&error);
        assert_eq!(summary.category, "not-implemented");
        assert_eq!(
            summary.code.as_deref(),
            Some("bridge.method.not_implemented")
        );
    }

    #[test]
    fn projects_health_from_service() {
        let service = EmbossService::empty();
        let health = project_health(&service);
        assert_eq!(health.sister_package, "emboss-r");
        assert_eq!(health.providers_configured, 0);
        assert!(health.operation_status.ok);
    }

    #[test]
    fn projects_shared_method_result_summary() {
        let context = ExecutionContext::for_origin(InvocationOrigin::Cli);
        let report = ExecutionReport::from_context(
            &context,
            "emboss-rs",
            "0.1.0",
            ExecutionOutcome::new(OutcomeStatus::Succeeded).with_summary("ok"),
        );
        let result = MethodResult::new(
            ToolName::new("seqret").expect("tool name should build"),
            ResultPayload::Empty,
            ResultSummary::new("Sequence result").with_line("Length: 4"),
            report,
        );

        let summary = BridgeResultSummary::from(&result);
        assert_eq!(summary.tool, "seqret");
        assert_eq!(summary.payload_kind, "empty");
        assert_eq!(summary.title, "Sequence result");
        assert_eq!(summary.lines, vec!["Length: 4"]);
    }
}
