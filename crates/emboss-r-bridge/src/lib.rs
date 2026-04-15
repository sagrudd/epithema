//! Rust-side bridge scaffolding for integration with the sister `emboss-r` project.

use emboss_core::PLATFORM_IDENTITY;
use emboss_plot_contract::PlotPayload;

/// Minimal handshake metadata for a future R bridge.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BridgeHandshake {
    /// Sister package expected by the Rust workspace.
    pub package_name: &'static str,
    /// Plot backend expected by the Rust workspace.
    pub plot_backend: &'static str,
}

/// Returns the initial bridge handshake metadata.
#[must_use]
pub fn handshake() -> BridgeHandshake {
    BridgeHandshake {
        package_name: PLATFORM_IDENTITY.sister_project,
        plot_backend: PLATFORM_IDENTITY.plot_backend,
    }
}

/// Indicates whether the bridge can accept a plot payload.
#[must_use]
pub fn supports_plot_payload(_payload: &PlotPayload) -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::handshake;

    #[test]
    fn points_to_emboss_r() {
        assert_eq!(handshake().package_name, "emboss-r");
    }
}
