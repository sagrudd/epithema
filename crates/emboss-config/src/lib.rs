//! Typed platform configuration defaults and policy overrides for EMBOSS-RS.
//!
//! This crate centralizes runtime configuration for paths, acquisition policy,
//! and provider enablement so that the CLI, shared service layer, autodoc, and
//! future tools use the same platform settings.

pub mod env;
pub mod paths;
pub mod policy;
pub mod provider;

pub use env::ConfigEnvironment;
pub use paths::PlatformPaths;
pub use policy::{AcquisitionPolicy, AutodocPolicy};
pub use provider::{PlatformConfig, ProviderSettings};
