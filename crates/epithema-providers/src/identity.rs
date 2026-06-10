//! Provider identity types.

use std::fmt::{Display, Formatter};

use epithema_diagnostics::{ErrorCategory, PlatformError};

/// Stable identifier for a configured or known provider backend.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct ProviderId(String);

impl ProviderId {
    /// Creates a provider identifier from a caller-supplied name.
    pub fn new(name: impl Into<String>) -> Result<Self, PlatformError> {
        let name = name.into();
        let normalized = name.trim().to_ascii_lowercase();

        if normalized.is_empty() {
            return Err(PlatformError::new(
                ErrorCategory::Validation,
                "provider id must not be empty",
            )
            .with_code("providers.id.empty"));
        }

        if !normalized
            .chars()
            .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '-' || ch == '_')
        {
            return Err(PlatformError::new(
                ErrorCategory::Validation,
                "provider id must use lower-case ASCII letters, digits, '-' or '_'",
            )
            .with_code("providers.id.invalid_format")
            .with_detail(format!("received '{normalized}'")));
        }

        Ok(Self(normalized))
    }

    /// Returns the normalized provider identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for ProviderId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::ProviderId;

    #[test]
    fn normalizes_provider_ids() {
        let provider = ProviderId::new(" ENA ").expect("provider id should be valid");
        assert_eq!(provider.as_str(), "ena");
    }

    #[test]
    fn rejects_invalid_provider_ids() {
        assert!(ProviderId::new("ena/http").is_err());
    }
}
