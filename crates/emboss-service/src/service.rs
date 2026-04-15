//! Shared service façade for front-end-neutral tool discovery and invocation.

use emboss_config::PlatformConfig;
use emboss_core::PLATFORM_IDENTITY;
use emboss_diagnostics::{ExecutionOutcome, ExecutionReport, OutcomeStatus};
use emboss_providers::ProviderRegistry;
use emboss_tools::ToolDescriptor;

use crate::ServiceDocumentationAcquisition;
use crate::context::ExecutionContext;
use crate::error::{ServiceError, unknown_tool};
use crate::registry::{ServiceRegistry, ToolCatalog};
use crate::request::InvocationRequest;
use crate::response::InvocationResponse;

/// Front-end-neutral EMBOSS-RS service façade.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct EmbossService {
    registry: ServiceRegistry,
    config: PlatformConfig,
    providers: ProviderRegistry,
}

impl EmbossService {
    /// Creates a service façade for the supplied registry.
    #[must_use]
    pub fn new(registry: ServiceRegistry) -> Self {
        Self::with_platform(registry, PlatformConfig::default(), ProviderRegistry::new())
    }

    /// Creates a service façade with explicit platform configuration and providers.
    #[must_use]
    pub fn with_platform(
        registry: ServiceRegistry,
        config: PlatformConfig,
        providers: ProviderRegistry,
    ) -> Self {
        Self {
            registry,
            config,
            providers,
        }
    }

    /// Creates an empty service façade.
    #[must_use]
    pub fn empty() -> Self {
        Self::new(ServiceRegistry::new())
    }

    /// Returns the active tool registry.
    #[must_use]
    pub fn registry(&self) -> &ServiceRegistry {
        &self.registry
    }

    /// Returns the active platform configuration.
    #[must_use]
    pub fn config(&self) -> &PlatformConfig {
        &self.config
    }

    /// Returns the active provider registry.
    #[must_use]
    pub fn providers(&self) -> &ProviderRegistry {
        &self.providers
    }

    /// Returns the formal documentation acquisition gateway for docgen paths.
    #[must_use]
    pub fn documentation_acquisition(&self) -> ServiceDocumentationAcquisition<'_> {
        ServiceDocumentationAcquisition::new(&self.config, &self.providers)
    }

    /// Returns a human-readable status line for front ends.
    #[must_use]
    pub fn status_line(&self) -> String {
        format!(
            "{} service ready; {} tools registered; {} providers configured",
            PLATFORM_IDENTITY.invocation_pattern(),
            self.registry.len(),
            self.providers.len()
        )
    }

    /// Returns the known tool descriptors.
    #[must_use]
    pub fn descriptors(&self) -> &[ToolDescriptor] {
        self.registry.descriptors()
    }

    /// Resolves a request to a known tool and returns the placeholder invocation
    /// response used until tool execution is implemented.
    pub fn invoke(&self, request: InvocationRequest) -> Result<InvocationResponse, ServiceError> {
        let descriptor = self
            .registry
            .find(request.tool())
            .copied()
            .ok_or_else(|| unknown_tool(request.tool()))?;

        let report = ExecutionReport::from_context(
            &request.context,
            PLATFORM_IDENTITY.binary_name,
            env!("CARGO_PKG_VERSION"),
            ExecutionOutcome::new(OutcomeStatus::NotImplemented).with_summary(format!(
                "tool '{}' is governed but not implemented yet",
                descriptor.name
            )),
        );

        Ok(InvocationResponse::not_implemented(
            request.context,
            request.tool,
            descriptor,
            report,
        ))
    }

    /// Builds the default CLI-oriented context for callers that do not supply one.
    #[must_use]
    pub fn default_context(&self) -> ExecutionContext {
        ExecutionContext::cli()
    }
}

#[cfg(test)]
mod tests {
    use emboss_tools::ToolDescriptor;

    use super::EmbossService;
    use crate::{
        ExecutionContext, InvocationOrigin, InvocationRequest, OutcomeStatus, ServiceRegistry,
        ToolName,
    };

    #[test]
    fn resolves_registered_tool_to_placeholder_response() {
        let mut registry = ServiceRegistry::new();
        registry
            .register(ToolDescriptor::new("needle", "global alignment"))
            .expect("registration should succeed");

        let service = EmbossService::new(registry);
        let request = InvocationRequest::new(
            ExecutionContext::for_origin(InvocationOrigin::Cli),
            ToolName::new("needle").expect("tool name should be valid"),
        );

        let response = service.invoke(request).expect("tool should resolve");
        assert_eq!(response.descriptor.name, "needle");
        assert_eq!(response.tool.as_str(), "needle");
        assert_eq!(
            response.report.outcome.status,
            OutcomeStatus::NotImplemented
        );
    }

    #[test]
    fn rejects_unknown_tool_invocation() {
        let service = EmbossService::empty();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("water").expect("tool name should be valid"),
        );

        assert!(service.invoke(request).is_err());
    }

    #[test]
    fn starts_with_default_platform_configuration_and_no_providers() {
        let service = EmbossService::empty();
        assert!(service.providers().is_empty());
        assert!(service.config().acquisition.allow_remote_acquisition);
    }
}
