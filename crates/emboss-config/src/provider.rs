//! Provider-related platform configuration.

use emboss_providers::ProviderId;

use crate::{AcquisitionPolicy, AutodocPolicy, PlatformPaths};

/// Per-provider configuration block.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderSettings {
    /// Stable provider identity.
    pub id: ProviderId,
    /// Whether the provider is enabled for selection.
    pub enabled: bool,
}

impl ProviderSettings {
    /// Creates an enabled provider settings block for the supplied identity.
    #[must_use]
    pub fn enabled(id: ProviderId) -> Self {
        Self { id, enabled: true }
    }
}

/// Root platform configuration for EMBOSS-RS runtime behavior.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlatformConfig {
    /// Managed platform paths.
    pub paths: PlatformPaths,
    /// Acquisition policy shared across front ends.
    pub acquisition: AcquisitionPolicy,
    /// Autodoc-specific acquisition policy.
    pub autodoc: AutodocPolicy,
    /// Known provider configuration blocks.
    provider_settings: Vec<ProviderSettings>,
}

impl PlatformConfig {
    /// Creates the default platform configuration.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns configured provider settings.
    #[must_use]
    pub fn provider_settings(&self) -> &[ProviderSettings] {
        &self.provider_settings
    }

    /// Appends a provider settings block.
    #[must_use]
    pub fn with_provider(mut self, provider: ProviderSettings) -> Self {
        self.provider_settings.push(provider);
        self
    }
}

impl Default for PlatformConfig {
    fn default() -> Self {
        Self {
            paths: PlatformPaths::default(),
            acquisition: AcquisitionPolicy::default(),
            autodoc: AutodocPolicy::default(),
            provider_settings: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{PlatformConfig, ProviderSettings};
    use emboss_providers::ProviderId;

    #[test]
    fn platform_config_defaults_are_local_and_remote_capable() {
        let config = PlatformConfig::default();
        assert!(config.acquisition.allow_local_files);
        assert!(config.acquisition.allow_remote_acquisition);
        assert!(config.provider_settings().is_empty());
    }

    #[test]
    fn appends_provider_settings() {
        let config = PlatformConfig::default().with_provider(ProviderSettings::enabled(
            ProviderId::new("ena").expect("valid provider id"),
        ));

        assert_eq!(config.provider_settings().len(), 1);
        assert_eq!(config.provider_settings()[0].id.as_str(), "ena");
    }
}
