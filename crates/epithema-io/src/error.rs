//! IO error model for sequence formats.

use std::error::Error;
use std::fmt::{Display, Formatter};

use epithema_core::DomainError;

/// Format-specific IO failures for FASTA and FASTQ support.
#[derive(Debug)]
pub enum IoError {
    /// Underlying reader or writer IO failure.
    Io(std::io::Error),
    /// Found malformed or incomplete format structure.
    Parse {
        /// Format identifier, such as `fasta` or `fastq`.
        format: &'static str,
        /// 1-based line number when available.
        line: Option<usize>,
        /// Human-readable failure message.
        message: String,
    },
    /// Biological core validation failed while constructing a typed record.
    Domain(DomainError),
}

impl IoError {
    /// Creates a parse error for the supplied format and message.
    #[must_use]
    pub fn parse(format: &'static str, line: Option<usize>, message: impl Into<String>) -> Self {
        Self::Parse {
            format,
            line,
            message: message.into(),
        }
    }
}

impl Display for IoError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(error) => Display::fmt(error, f),
            Self::Parse {
                format,
                line,
                message,
            } => match line {
                Some(line) => write!(f, "invalid {format} content at line {line}: {message}"),
                None => write!(f, "invalid {format} content: {message}"),
            },
            Self::Domain(error) => Display::fmt(error, f),
        }
    }
}

impl Error for IoError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::Parse { .. } => None,
            Self::Domain(error) => Some(error),
        }
    }
}

impl From<std::io::Error> for IoError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<DomainError> for IoError {
    fn from(value: DomainError) -> Self {
        Self::Domain(value)
    }
}
