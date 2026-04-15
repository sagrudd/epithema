//! Filesystem-related platform paths.

use std::path::PathBuf;

/// Platform-managed filesystem roots used by EMBOSS-RS.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlatformPaths {
    /// Root for durable data assets.
    pub data_root: PathBuf,
    /// Root for cached provider-acquired assets.
    pub cache_root: PathBuf,
    /// Root for documentation and autodoc acquisition assets.
    pub docs_asset_root: PathBuf,
}

impl Default for PlatformPaths {
    fn default() -> Self {
        Self {
            data_root: PathBuf::from("var/data"),
            cache_root: PathBuf::from("var/cache"),
            docs_asset_root: PathBuf::from("var/docs"),
        }
    }
}
