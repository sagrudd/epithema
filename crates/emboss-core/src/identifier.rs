//! Identifier and display-name primitives.

use crate::error::DomainError;

/// Stable sequence identifier with an optional display name.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct SequenceIdentifier {
    accession: String,
    display_name: Option<String>,
}

impl SequenceIdentifier {
    /// Creates an identifier from the supplied accession or primary key.
    pub fn new(accession: impl Into<String>) -> Result<Self, DomainError> {
        let accession = accession.into();
        let accession = accession.trim();

        if accession.is_empty() {
            return Err(DomainError::EmptyIdentifier);
        }

        Ok(Self {
            accession: accession.to_owned(),
            display_name: None,
        })
    }

    /// Returns the stable accession or primary identifier.
    #[must_use]
    pub fn accession(&self) -> &str {
        &self.accession
    }

    /// Returns the optional human-readable display name.
    #[must_use]
    pub fn display_name(&self) -> Option<&str> {
        self.display_name.as_deref()
    }

    /// Sets a human-readable display name.
    #[must_use]
    pub fn with_display_name(mut self, display_name: impl Into<String>) -> Self {
        let display_name = display_name.into();
        let trimmed = display_name.trim();

        self.display_name = (!trimmed.is_empty()).then(|| trimmed.to_owned());
        self
    }

    /// Returns the preferred label for display purposes.
    #[must_use]
    pub fn display_label(&self) -> &str {
        self.display_name.as_deref().unwrap_or(&self.accession)
    }
}

#[cfg(test)]
mod tests {
    use super::SequenceIdentifier;

    #[test]
    fn prefers_display_name_when_present() {
        let identifier = SequenceIdentifier::new("NM_000000")
            .expect("valid identifier")
            .with_display_name("Example");

        assert_eq!(identifier.display_label(), "Example");
    }
}
