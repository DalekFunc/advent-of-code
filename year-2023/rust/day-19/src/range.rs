use std::{collections::HashSet, iter};

use itertools::Itertools;

use crate::types::Part;

// exclusive
// therefore max range is 1..4001
// end has to larger than start
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Range {
    pub start: u32,
    pub end: u32,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct PartRange {
    pub x: Range,
    pub m: Range,
    pub a: Range,
    pub s: Range,
}

// if overlaps
// a..b..A.....B   c..d...C....D
// 2nd case: a...b...B...A   c...d...D...C
// 3rd case: a...b...B...A   c...d....C...D

// 1st case soln: b..A, d..C; a..b, c..d; A..B, C..D
// 2nd case soln: a..b, c..d; b..B, d..D; B..A, D..C
// 3rd case soln: b..=B, d..C; a..b, c..d;  b..B, c..D

impl Range {
    // a...b...A...B           b...a...B...A
    // a...b...B...A           b...a...A...B
    fn has_overlaps(&self, other: &Range) -> bool {
        std::cmp::min(self.end, other.end) as i32 - std::cmp::max(self.start, other.start) as i32
            > 0
    }

    fn overlap_len(&self, other: &Range) -> u32 {
        let phi = std::cmp::min(self.end, other.end) as i32
            - std::cmp::max(self.start, other.start) as i32;
        if phi > 0 {
            phi as u32
        } else {
            0
        }
    }

    fn is_subrange_of(&self, range: &Range) -> bool {
        self.start >= range.start && self.end <= range.end
    }

    fn contains(&self, range: &Range) -> bool {
        range.start >= self.start && range.end <= self.end
    }

    fn intersection(&self, other: &Range) -> Option<Range> {
        self.has_overlaps(other).then(|| Range {
            start: std::cmp::max(self.start, other.start),
            end: std::cmp::min(self.end, other.end),
        })
    }

    // self - intersection
    // IMPORTANT: This implementation assumes self contains intersection
    fn difference(&self, other: &Range) -> Vec<Range> {
        assert!(
            self.contains(other),
            "This difference implementation assumes self contains intersection"
        );

        let mut diff = vec![];

        // left
        if self.start != other.start {
            diff.push(Range {
                start: self.start,
                end: other.start,
            });
        }
        // right
        if self.end != other.end {
            diff.push(Range {
                start: other.end,
                end: self.end,
            });
        }

        diff
    }

    // there can be 1 - 3 output
    fn disjoint_ranges(&self, other: Range) -> Vec<Range> {
        if !self.has_overlaps(&other) {
            vec![*self, other]
        } else {
            if *self == other {
                vec![other]
            } else if self.is_subrange_of(&other) {
                Self::disjoint_subrange(other, *self)
            } else if other.is_subrange_of(self) {
                Self::disjoint_subrange(*self, other)
            } else {
                let (left, right) = if self.start < other.start {
                    (*self, other)
                } else {
                    (other, *self)
                };

                vec![
                    Range {
                        // left not overlapped
                        start: left.start,
                        end: right.start,
                    },
                    Range {
                        // overlapped
                        start: right.start,
                        end: left.end,
                    },
                    Range {
                        // right not overlapped
                        start: left.end,
                        end: right.end,
                    },
                ]
            }
        }
    }

    fn disjoint_subrange(bigger: Range, smaller: Range) -> Vec<Range> {
        if smaller.start == bigger.start {
            vec![
                smaller,
                Range {
                    start: smaller.end,
                    end: bigger.end,
                },
            ]
        } else if smaller.end == bigger.end {
            vec![
                smaller,
                Range {
                    start: bigger.start,
                    end: smaller.start,
                },
            ]
        } else {
            vec![
                smaller,
                Range {
                    start: bigger.start,
                    end: smaller.start,
                },
                Range {
                    start: smaller.end,
                    end: bigger.end,
                },
            ]
        }
    }

    fn len(&self) -> u64 {
        (self.end - self.start) as u64
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case( Range{start: 1, end: 5}, Range{start: 2, end: 4}, true)]
    #[case( Range{start: 1, end: 5}, Range{start: 2, end: 6}, true)]
    #[case( Range{start: 1, end: 5}, Range{start: 4, end: 6}, true)]
    #[case( Range{start: 1, end: 3}, Range{start: 3, end: 5}, false)]
    #[case( Range{start: 4, end: 5}, Range{start: 1, end: 3}, false)]
    fn test_range_overlap(#[case] lhs: Range, #[case] rhs: Range, #[case] expected: bool) {
        assert_eq!(lhs.has_overlaps(&rhs), expected);
        assert_eq!(rhs.has_overlaps(&lhs), expected);
    }

    #[rstest]
    #[case( Range{start: 1, end: 5}, Range{start: 2, end: 4}, 2)]
    #[case( Range{start: 1, end: 5}, Range{start: 2, end: 6}, 3)]
    #[case( Range{start: 1, end: 5}, Range{start: 4, end: 6}, 1)]
    #[case( Range{start: 1, end: 3}, Range{start: 3, end: 5}, 0)]
    #[case( Range{start: 4, end: 5}, Range{start: 1, end: 3}, 0)]
    fn test_range_overlap_len(#[case] lhs: Range, #[case] rhs: Range, #[case] expected: u32) {
        assert_eq!(lhs.overlap_len(&rhs), expected);
        assert_eq!(rhs.overlap_len(&lhs), expected);
    }
}

impl PartRange {
    fn has_overlaps(&self, other: &PartRange) -> bool {
        self.x.has_overlaps(&other.x)
            && self.m.has_overlaps(&other.m)
            && self.a.has_overlaps(&other.a)
            && self.s.has_overlaps(&other.s)
    }

    fn contains(&self, other: &PartRange) -> bool {
        self.x.contains(&other.x)
            && self.m.contains(&other.m)
            && self.a.contains(&other.a)
            && self.s.contains(&other.s)
    }

    fn intersection(&self, other: &PartRange) -> Option<PartRange> {
        self.has_overlaps(other).then(|| PartRange {
            x: self
                .x
                .intersection(&other.x)
                .expect("x should have intersection"),
            m: self
                .m
                .intersection(&other.m)
                .expect("m should have intersection"),
            a: self
                .a
                .intersection(&other.a)
                .expect("a should have intersection"),
            s: self
                .s
                .intersection(&other.s)
                .expect("s should have intersection"),
        })
    }

    // assumption intersection should be a subrange of self
    // This function returns all the disjoint ranges that are not part of intersections
    fn disjoints_with_intersection(&self, intersection: &PartRange) -> Vec<PartRange> {
        assert!(
            self.contains(intersection),
            "Self should contains intersection"
        );

        // there can be at most 16 partranges if the intersection split right in the middle
        let xd = self.x.difference(&intersection.x);
        let md = self.m.difference(&intersection.m);
        let ad = self.a.difference(&intersection.a);
        let sd = self.s.difference(&intersection.s);
        xd.into_iter()
            .cartesian_product(md)
            .cartesian_product(ad)
            .cartesian_product(sd)
            .map(|(((x, m), a), s)| PartRange { x, m, a, s })
            .collect_vec()
    }

    //   B
    //  AXa -> A, a, B, b, X -> continue with [A, a], save B, b, X to new_disjoint Rnage
    //   b
    fn disjoints(&self, other: &PartRange) -> (Vec<PartRange>, Vec<PartRange>) {
        let Some(intersection) = self.intersection(other) else {
            panic!("there must be intersections")
        };

        let mut bbx = vec![intersection];
        bbx.extend(other.disjoints_with_intersection(&intersection));

        (self.disjoints_with_intersection(&intersection), bbx)
    }

    // This function takes a list of disjoint ranges which may or may not overlaps with self
    // return new disjoint ranges that covers all the params ranges.
    fn partitions(&self, disjoint_ranges: &[PartRange]) -> Vec<PartRange> {
        let mut new_disjoint_ranges = vec![];

        let mut self_parts = HashSet::new();
        self_parts.insert(self.clone());

        for range in disjoint_ranges {
            // find the self_part that intersect with range
            let self_part = self_parts
                .iter()
                .find(|self_part| self_part.has_overlaps(range))
                .expect("There must be one overlap")
                .clone();

            let (new_self_parts, for_disjoint_ranges) = self_part.disjoints(range);
            self_parts.remove(&self_part);
            self_parts.extend(new_self_parts);
            new_disjoint_ranges.extend(for_disjoint_ranges);
        }

        new_disjoint_ranges.extend(self_parts);

        new_disjoint_ranges
    }

    pub fn combinations(&self) -> u64 {
        self.x.len() * self.m.len() * self.a.len() * self.s.len()
    }
}

// all ranges are mutually disjoint
struct DisjointRanges {
    ranges: Vec<PartRange>,
}

impl DisjointRanges {
    fn add(&mut self, new_range: PartRange) {
        // if new_range doesnt overlap with any of existings, its a simple push.
        // for any overlaps, takes overlapping existings out and perform disjoint with new_range each
        // add the disjoints from existings part back in ranges
        // repeat taking rest disjoint until they are mutally disjoint. (HOW?)

        // find all overlaps
        let non_overlaps = self
            .ranges
            .iter()
            .filter(|range| !range.has_overlaps(&new_range))
            .collect_vec();
        let overlaps = self
            .ranges
            .iter()
            .filter(|range| range.has_overlaps(&new_range))
            .collect_vec();

        // new range is disjoint with members
        if overlaps.is_empty() {
            self.ranges.push(new_range);
            return;
        }

        // if newrange is a subrange of a member of overlap
        // if newrange is a subrange of multiple members of overlap IMPOSSIBLE because members are disjoint
        if let Some(_super_range) = overlaps.iter().find(|range| range.contains(&new_range)) {
            return;
        }

        // if newrange is "super range" and has serveral subranges in overlap
        let subranges = overlaps
            .iter()
            .filter(|range| new_range.contains(range))
            .map(|m| *m)
            .collect_vec();
        let non_subranges = overlaps
            .iter()
            .filter(|range| !new_range.contains(range))
            .map(|m| *m)
            .map(|m| *m)
            .collect_vec();

        // disjoint<V> the new super range OR simply the newrange with remaining overlaps members that wasnt subranges
        // new ranges combined serveral members
        if non_subranges.is_empty() {
            self.ranges = Vec::from_iter(
                non_overlaps
                    .into_iter()
                    .map(|m| *m)
                    .chain(non_subranges)
                    .chain(iter::once(new_range)),
            )
        } else {
            let new_disjoint_ranges = new_range.partitions(&non_subranges);

            self.ranges = Vec::from_iter(
                non_overlaps
                    .into_iter()
                    .map(|m| *m)
                    .chain(new_disjoint_ranges),
            )
        }
    }

    fn combinations(&self) -> u64 {
        self.ranges.iter().map(PartRange::combinations).sum()
    }
}
