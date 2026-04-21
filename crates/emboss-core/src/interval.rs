//! Coordinate and interval primitives.
//!
//! EMBOSS-RS uses zero-based, half-open intervals throughout the core model.
//! An interval `[start, end)` includes `start` and excludes `end`.

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

    /// Creates a zero-based half-open interval from 1-based inclusive coordinates.
    pub fn from_one_based_inclusive(start: usize, end: usize) -> Result<Self, DomainError> {
        if start == 0 || end == 0 {
            return Err(DomainError::InvalidInterval { start, end });
        }

        Self::new(start - 1, end)
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

    /// Returns true when this interval overlaps another interval.
    #[must_use]
    pub fn intersects(self, other: Self) -> bool {
        self.start < other.end && other.start < self.end
    }

    /// Returns the overlapping region between two intervals when one exists.
    #[must_use]
    pub fn intersection(self, other: Self) -> Option<Self> {
        let start = self.start.max(other.start);
        let end = self.end.min(other.end);
        (start < end).then_some(Self { start, end })
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
    fn converts_one_based_inclusive_coordinates() {
        let interval = Interval::from_one_based_inclusive(2, 5).expect("valid interval");
        assert_eq!(interval, Interval::new(1, 5).expect("valid interval"));
    }

    #[test]
    fn rejects_zero_one_based_coordinates() {
        assert!(Interval::from_one_based_inclusive(0, 3).is_err());
        assert!(Interval::from_one_based_inclusive(1, 0).is_err());
    }

    #[test]
    fn reports_containment() {
        let outer = Interval::new(2, 10).expect("valid outer interval");
        let inner = Interval::new(3, 5).expect("valid inner interval");

        assert!(outer.contains_interval(inner));
        assert!(!inner.contains_interval(outer));
    }

    #[test]
    fn computes_intersection() {
        let left = Interval::new(2, 8).expect("valid interval");
        let right = Interval::new(5, 10).expect("valid interval");

        assert_eq!(
            left.intersection(right),
            Some(Interval::new(5, 8).expect("valid overlap"))
        );
    }
}
