//! Strand-orientation primitives.

use std::fmt::{Display, Formatter};

/// Strand orientation for strand-aware sequence and feature contexts.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Strand {
    /// Forward or plus strand.
    Forward,
    /// Reverse or minus strand.
    Reverse,
    /// Explicitly unstranded context.
    Unstranded,
    /// Strand is not known.
    Unknown,
}

impl Strand {
    /// Returns the strand opposite to the current one when that is defined.
    #[must_use]
    pub fn opposite(self) -> Self {
        match self {
            Self::Forward => Self::Reverse,
            Self::Reverse => Self::Forward,
            Self::Unstranded => Self::Unstranded,
            Self::Unknown => Self::Unknown,
        }
    }
}

impl Display for Strand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let symbol = match self {
            Self::Forward => "+",
            Self::Reverse => "-",
            Self::Unstranded => ".",
            Self::Unknown => "?",
        };

        f.write_str(symbol)
    }
}

#[cfg(test)]
mod tests {
    use super::Strand;

    #[test]
    fn reverses_orientation() {
        assert_eq!(Strand::Forward.opposite(), Strand::Reverse);
    }
}
