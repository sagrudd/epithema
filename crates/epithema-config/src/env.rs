//! Environment-driven configuration overrides.

use crate::PlatformConfig;

/// Helper for applying environment overrides to platform configuration.
pub struct ConfigEnvironment;

impl ConfigEnvironment {
    /// Builds a platform configuration from defaults plus process environment overrides.
    #[must_use]
    pub fn from_process_environment() -> PlatformConfig {
        Self::apply_overrides(PlatformConfig::default(), std::env::vars())
    }

    /// Applies overrides from an arbitrary environment iterator.
    #[must_use]
    pub fn apply_overrides<I, K, V>(mut config: PlatformConfig, overrides: I) -> PlatformConfig
    where
        I: IntoIterator<Item = (K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        for (key, value) in overrides {
            match key.as_ref() {
                "EPITHEMA_DATA_DIR" => config.paths.data_root = value.as_ref().into(),
                "EPITHEMA_CACHE_DIR" => config.paths.cache_root = value.as_ref().into(),
                "EPITHEMA_DOCS_ASSET_DIR" => config.paths.docs_asset_root = value.as_ref().into(),
                "EPITHEMA_ALLOW_REMOTE" => {
                    if let Some(value) = parse_bool(value.as_ref()) {
                        config.acquisition.allow_remote_acquisition = value;
                    }
                }
                "EPITHEMA_PREFER_LOCAL_CACHE" => {
                    if let Some(value) = parse_bool(value.as_ref()) {
                        config.acquisition.prefer_local_cache = value;
                    }
                }
                _ => {}
            }
        }

        config
    }
}

fn parse_bool(value: &str) -> Option<bool> {
    match value.trim().to_ascii_lowercase().as_str() {
        "1" | "true" | "yes" | "on" => Some(true),
        "0" | "false" | "no" | "off" => Some(false),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::ConfigEnvironment;

    #[test]
    fn applies_environment_overrides() {
        let config = ConfigEnvironment::apply_overrides(
            crate::PlatformConfig::default(),
            [
                ("EPITHEMA_DATA_DIR", "/tmp/emboss-data"),
                ("EPITHEMA_ALLOW_REMOTE", "false"),
            ],
        );

        assert_eq!(config.paths.data_root.as_os_str(), "/tmp/emboss-data");
        assert!(!config.acquisition.allow_remote_acquisition);
    }
}
