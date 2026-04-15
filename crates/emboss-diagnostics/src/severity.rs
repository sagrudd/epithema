//! Diagnostic severity levels.

use std::fmt::{Display, Formatter};

/// Relative severity of a diagnostic emitted by the platform.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Severity {
    /// Informational or advisory note.
    Notice,
    /// Non-fatal warning that should be surfaced to callers.
    Warning,
    /// Error-level diagnostic intended for reports or validation summaries.
    Error,
}

impl Display for Severity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Notice => f.write_str("notice"),
            Self::Warning => f.write_str("warning"),
            Self::Error => f.write_str("error"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Severity;

    #[test]
    fn severity_orders_from_notice_to_error() {
        assert!(Severity::Notice < Severity::Warning);
        assert!(Severity::Warning < Severity::Error);
    }
}
