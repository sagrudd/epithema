//! Coordinate and interval primitives.

use crate::error::DomainError;

/// A zero-based, half-open interval `[start, end)`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Interval {
    start: usize,
    end: usize,
}

impl Interval {
    /// Creates a validated interval.
    pub fn new(start: usize, end: usize) -> Result<Self, DomainError> {
        if start >= end {
            return Err(DomainError::InvalidInterval { start, end });
        }

        Ok(Self { start, end })
    }

    /// Returns the inclusive start position.
    #[must_use]
    pub fn start(self) -> usize {
        self.start
    }

    /// Returns the exclusive end position.
    #[must_use]
    pub fn end(self) -> usize {
        self.end
    }

    /// Returns the interval length.
    #[must_use]
    pub fn len(self) -> usize {
        self.end - self.start
    }

    /// Returns true when the interval contains no positions.
    #[must_use]
    pub fn is_empty(self) -> bool {
        self.len() == 0
    }

    /// Returns true if the interval contains the supplied position.
    #[must_use]
    pub fn contains(self, position: usize) -> bool {
        (self.start..self.end).contains(&position)
    }

    /// Returns true if `other` lies entirely within `self`.
    #[must_use]
    pub fn contains_interval(self, other: Self) -> bool {
        other.start >= self.start && other.end <= self.end
    }
}

#[cfg(test)]
mod tests {
    use super::Interval;

    #[test]
    fn rejects_empty_interval() {
        assert!(Interval::new(4, 4).is_err());
    }

    #[test]
    fn reports_containment() {
        let outer = Interval::new(2, 10).expect("valid outer interval");
        let inner = Interval::new(3, 5).expect("valid inner interval");

        assert!(outer.contains_interval(inner));
        assert!(!inner.contains_interval(outer));
    }
}
