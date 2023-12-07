use std::{
    fmt,
    ops::{RangeBounds, RangeInclusive},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DisjointRange {
    pub start: u64,
    pub end: u64,
}

impl DisjointRange {
    pub const FULL: DisjointRange = DisjointRange {
        start: u64::MIN,
        end: u64::MAX,
    };
}

pub fn join_connected_ranges(lhs: &DisjointRange, rhs: &DisjointRange) -> DisjointRange {
    assert!(is_adjacent(lhs, rhs), "Cannot join unconnected ranges.");
    DisjointRange {
        start: lhs.start,
        end: rhs.end,
    }
}

pub fn is_adjacent(lhs: &DisjointRange, rhs: &DisjointRange) -> bool {
    lhs.end + 1 == rhs.start
}

pub fn have_overlap(lhs: &DisjointRange, rhs: &DisjointRange) -> bool {
    lhs.end >= rhs.start
}

pub fn is_disjoint(lhs: &DisjointRange, rhs: &DisjointRange) -> bool {
    !have_overlap(lhs, rhs)
}

pub fn transfer(from: &DisjointRange, to: &DisjointRange, value: u64) -> u64 {
    assert!(from.contains(&value));

    value - from.start + to.start
}

// region:    --- Traits

impl fmt::Display for DisjointRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _ = writeln!(f, "{} -> {}", self.start, self.end);

        Ok(())
    }
}

impl PartialOrd for DisjointRange {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DisjointRange {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self == other {
            std::cmp::Ordering::Equal
        } else {
            self.end.cmp(&other.start)
        }
    }
}

impl RangeBounds<u64> for DisjointRange {
    fn start_bound(&self) -> std::ops::Bound<&u64> {
        std::ops::Bound::Included(&self.start)
    }

    fn end_bound(&self) -> std::ops::Bound<&u64> {
        std::ops::Bound::Included(&self.end)
    }
}

impl From<RangeInclusive<u64>> for DisjointRange {
    fn from(value: RangeInclusive<u64>) -> Self {
        Self {
            start: *value.start(),
            end: *value.end(),
        }
    }
}

// endregion: --- Traits
