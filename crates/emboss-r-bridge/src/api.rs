//! Minimal Rust-callable bridge surface for future R wrappers.

use emboss_plot_contract::PlotPayload;
use emboss_service::EmbossService;

use crate::conversion::{project_health, project_version};
use crate::health::BridgeHealth;
use crate::types::BridgeToolSummary;
use crate::version::BridgeVersion;

/// Returns stable version and platform metadata for the bridge surface.
#[must_use]
pub fn bridge_version() -> BridgeVersion {
    project_version()
}

/// Returns a bridge-facing health summary using the supplied service instance.
#[must_use]
pub fn health_check_with_service(service: &EmbossService) -> BridgeHealth {
    project_health(service)
}

/// Returns a bridge-facing health summary using a default empty service runtime.
#[must_use]
pub fn health_check() -> BridgeHealth {
    health_check_with_service(&EmbossService::empty())
}

/// Lists bridge-safe tool summaries from the supplied service instance.
#[must_use]
pub fn list_tools(service: &EmbossService) -> Vec<BridgeToolSummary> {
    service
        .descriptors()
        .iter()
        .map(BridgeToolSummary::from)
        .collect()
}

/// Confirms that the bridge accepts Rust-side plot payload contracts for
/// handoff to the R-owned rendering layer.
#[must_use]
pub fn supports_plot_payload(_payload: &PlotPayload) -> bool {
    true
}

#[cfg(test)]
mod tests {
    use emboss_plot_contract::PlotPayload;
    use emboss_service::{EmbossService, ServiceRegistry};
    use emboss_tools::ToolDescriptor;

    use super::{bridge_version, health_check, list_tools, supports_plot_payload};

    #[test]
    fn exposes_bridge_version() {
        let version = bridge_version();
        assert_eq!(version.sister_package, "emboss-r");
    }

    #[test]
    fn reports_default_health() {
        let health = health_check();
        assert!(health.operation_status.ok);
        assert_eq!(health.tools_registered, 0);
    }

    #[test]
    fn lists_projected_tools() {
        let mut registry = ServiceRegistry::new();
        registry
            .register(ToolDescriptor::new("seqret", "sequence conversion"))
            .expect("tool registration should succeed");
        let service = EmbossService::new(registry);

        let tools = list_tools(&service);
        assert_eq!(tools.len(), 1);
        assert_eq!(tools[0].name, "seqret");
    }

    #[test]
    fn accepts_plot_payload_contracts() {
        let payload = PlotPayload::empty("example");
        assert!(supports_plot_payload(&payload));
    }
}
