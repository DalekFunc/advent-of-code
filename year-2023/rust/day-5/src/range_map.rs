// region:    --- Modules

use std::{collections::BTreeMap, fmt, mem, ops::RangeBounds};

use crate::disjoint_range::{
    have_overlap, is_disjoint, join_connected_ranges, transfer, DisjointRange,
};

// endregion: --- Modules

type R = DisjointRange;

// A special BTreeMap which satsify the following requirements:
// All keys (from range) are mutually disjoint and together they cover the entire span of u64
// Same for all the values.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RangeMap {
    map: BTreeMap<R, R>,
}

impl RangeMap {
    pub fn build() -> RangeMapBuilder {
        RangeMapBuilder::new()
    }

    // keys becomes values, values becomes keys.
    // still satisfy the special requirement.
    pub fn invert(&self) -> Self {
        let mut inverted = BTreeMap::new();
        self.map.iter().for_each(|(key, value)| {
            inverted.insert(*value, *key);
        });

        Self { map: inverted }
    }

    pub fn map(&self, value: u64) -> u64 {
        let from = self
            .map
            .keys()
            .find(|range| range.contains(&value))
            .expect("Range map should cover entire span.");
        let to = self.map.get(from).expect("Value should exist.");

        transfer(from, to, value)
    }

    pub fn map_range(&self, mut domain: DisjointRange) -> Vec<DisjointRange> {
        let mut result = vec![];
        for (from, to) in &self.map {
            if is_disjoint(from, &domain) {
                continue;
            } else {
                let diff = domain.start - from.start;

                if from.start <= domain.start && from.end >= domain.end {
                    result.push(DisjointRange {
                        start: to.start + diff,
                        end: to.start + diff + domain.end - domain.start, // diff: padding; d.end - d.start: length
                    });

                    break;
                } else {
                    result.push(DisjointRange {
                        start: to.start + diff,
                        end: to.end,
                    });

                    domain = ((from.end + 1)..=(domain.end)).into();
                }
            }
        }

        result
    }

    // self is lhs
    pub fn concatenate(&self, rhs: &Self) -> Self {
        // FIXME: we are only handling cases we will use. it is incomplete.
        // Return type: overlapping, remaining
        // remember lhs is flipped
        fn overlap_lhs_is_longer(lhs: (&R, &R), rhs: (&R, &R)) -> (R, (R, R)) {
            assert!(have_overlap(lhs.0, rhs.0));
            let length = rhs.0.end - rhs.0.start + 1;
            (
                (lhs.1.start..=lhs.1.start + length - 1).into(),
                (
                    ((lhs.0.start + length)..=(lhs.0.end)).into(),
                    ((lhs.1.start + length)..=(lhs.1.end)).into(),
                ),
            )
        }

        fn overlap_rhs_is_longer(lhs: (&R, &R), rhs: (&R, &R)) -> (R, (R, R)) {
            assert!(have_overlap(lhs.0, rhs.0));
            let length = lhs.0.end - lhs.0.start + 1;

            (
                (rhs.1.start..=rhs.1.start + length - 1).into(),
                (
                    ((rhs.0.start + length)..=(rhs.0.end)).into(),
                    ((rhs.1.start + length)..=(rhs.1.end)).into(),
                ),
            )
        }

        // lhs_to are sorted.
        let lhs = self.invert();

        // compare each sorted lhs_to with rhs_form
        // since both are sorted and span entire u64
        // there can only be 3 scenarios from rhs point of view:
        // a) lhs_to or whatever remamining covers the same range as rhs_from
        // b) lhs_to or whatever remamining covers partially smaller range of rhs_from
        // c) lhs_to or whatever remamining covers more than range of rhs_from
        // for cases b we keep matching the remaining part of rhs_from
        // for case c vice versa

        let mut lhs_iter = lhs.map.iter();
        let mut rhs_iter = rhs.map.iter();

        let mut lhs_matching = lhs_iter.next(); // remember lhs_matching.0 is lhs_to
        let mut rhs_matching = rhs_iter.next();

        let mut lhs_unmatched;
        let mut rhs_unmatched;

        let mut builder = RangeMap::build();

        loop {
            let Some(lhs_m) = lhs_matching else {
                break;
            };
            let Some(rhs_m) = rhs_matching else {
                break;
            };

            // case a
            if lhs_m.0 == rhs_m.0 {
                builder = builder.insert(*lhs_m.1, *rhs_m.1);
                lhs_matching = lhs_iter.next();
                rhs_matching = rhs_iter.next();
            } else if lhs_m.0.end > rhs_m.0.end {
                // case c
                // insert the overlapping part
                // save the remaining part to lhs_matching
                let (lhs_overlapping, unmatched) = overlap_lhs_is_longer(lhs_m, rhs_m);
                lhs_unmatched = unmatched;
                builder = builder.insert(lhs_overlapping, *rhs_m.1);

                lhs_matching = Some((&lhs_unmatched.0, &lhs_unmatched.1));
                rhs_matching = rhs_iter.next();
            } else {
                // case b
                // insert the overlapping part
                // save the remaining part to rhs_matching
                let (rhs_overlapping, unmatched) = overlap_rhs_is_longer(lhs_m, rhs_m);
                rhs_unmatched = unmatched;
                builder = builder.insert(*lhs_m.1, rhs_overlapping);

                lhs_matching = lhs_iter.next();
                rhs_matching = Some((&rhs_unmatched.0, &rhs_unmatched.1));
            }
        }

        builder.fill_gaps()
    }

    // return the entire span of from range
    // keys is sorted
    #[allow(unused)]
    fn domain(&self) -> R {
        self.map
            .keys()
            .cloned()
            .reduce(|lhs, rhs| join_connected_ranges(&lhs, &rhs))
            .expect("RangeMap's keys should not be empty")
    }

    // return the entire span of from range
    // remember values are unsorted.
    #[allow(unused)]
    fn range(&self) -> R {
        let mut ranges: Vec<DisjointRange> = self.map.values().cloned().collect();

        ranges.sort();

        ranges
            .into_iter()
            .reduce(|lhs, rhs| join_connected_ranges(&lhs, &rhs))
            .expect("RangeMap's values should not be empty")
    }
}

// region:    --- Traits

impl Default for RangeMap {
    fn default() -> Self {
        let mut map = BTreeMap::new();
        map.insert(DisjointRange::FULL, DisjointRange::FULL);

        Self { map }
    }
}

impl fmt::Display for RangeMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (0..=100).for_each(|n| {
            let _ = writeln!(f, "{} -> {}", n, self.map(n));
        });

        Ok(())
    }
}

// endregion: --- Traits

// region:    --- Builder

#[derive(Debug, Default)]
pub struct RangeMapBuilder {
    map: BTreeMap<R, R>,
}

impl RangeMapBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(mut self, from: R, to: R) -> Self {
        self.map.insert(from, to);
        self
    }

    pub fn push(mut self, mapping: (u64, u64, u64)) -> Self {
        let from = (mapping.1..=mapping.1 + mapping.2 - 1).into();
        let to = (mapping.0..=mapping.0 + mapping.2 - 1).into();
        self.map.insert(from, to);
        self
    }

    pub fn fill_gaps(mut self) -> RangeMap {
        let mut untracked_start = 0;
        let keys: Vec<DisjointRange> = self.map.keys().cloned().collect();
        for key in &keys {
            // if there are gaps
            if key.start - untracked_start > 1 {
                let identical_mapping: DisjointRange = (untracked_start..=key.start - 1).into();
                self.map.insert(identical_mapping, identical_mapping);
            }
            untracked_start = key.end.saturating_add(1);
        }

        if keys.iter().last().unwrap().end != u64::MAX {
            let identical_mapping = (untracked_start..=u64::MAX).into();
            self.map.insert(identical_mapping, identical_mapping);
        }

        RangeMap {
            map: mem::take(&mut self.map),
        }
    }
}

// endregion: --- Builder

#[cfg(test)]
mod tests {
    use crate::range_map::RangeMap;
    use rstest::{fixture, rstest};

    use super::DisjointRange;

    #[test]
    fn quick_test() {
        // let fixture = RangeMap::default();
        // println!("{}", fixture);

        let map_builder = RangeMap::build();
        let map_builder = map_builder.insert((50..=97).into(), (52..=99).into());
        let map_builder = map_builder.insert((98..=99).into(), (50..=51).into());
        let fixture = map_builder.fill_gaps();

        println!("{}", fixture);
        println!("{:?}", fixture.domain());
        println!("{:?}", fixture.range());
    }

    #[test]
    fn test_transfer_range() {
        // let fixture = RangeMap::default();
        // println!("{}", fixture);

        let map_builder = RangeMap::build();
        let map_builder = map_builder.insert((50..=97).into(), (52..=99).into());
        let map_builder = map_builder.insert((98..=99).into(), (50..=51).into());
        let fixture = map_builder.fill_gaps();

        println!("{:?}", fixture);
        println!("{:?}", fixture.domain());
        println!("{:?}", fixture.range());
        assert_eq!(
            vec![DisjointRange { start: 0, end: 0 }],
            fixture.map_range((0..=0).into())
        );
        assert_eq!(
            vec![DisjointRange { start: 0, end: 49 }],
            fixture.map_range((0..=49).into())
        );
        assert_eq!(
            vec![
                DisjointRange { start: 0, end: 49 },
                DisjointRange { start: 52, end: 52 }
            ],
            fixture.map_range((0..=50).into())
        );
        assert_eq!(
            vec![DisjointRange {
                start: 100,
                end: 100
            }],
            fixture.map_range((100..=100).into())
        );
    }

    #[rstest]
    #[case(RangeMap::default(), 0, 0)]
    #[case(RangeMap::default(), 1, 1)]
    #[case(RangeMap::default(), u64::MAX, u64::MAX)]
    #[case(RangeMap::default(), u64::MIN, u64::MIN)]
    fn test_transfer(#[case] map: RangeMap, #[case] value: u64, #[case] expected: u64) {
        assert_eq!(expected, map.map(value));
    }

    #[rstest]
    #[case(RangeMap::default(), DisjointRange::FULL)]
    fn test_domain(#[case] map: RangeMap, #[case] expected: DisjointRange) {
        assert_eq!(expected, map.domain());
    }

    #[rstest]
    #[case(RangeMap::default(), DisjointRange::FULL)]
    fn test_range(#[case] map: RangeMap, #[case] expected: DisjointRange) {
        assert_eq!(expected, map.range());
    }

    #[rstest]
    #[case(RangeMap::default(), RangeMap::default(), RangeMap::default())]
    fn test_concatanate(#[case] lhs: RangeMap, #[case] rhs: RangeMap, #[case] expected: RangeMap) {
        assert_eq!(expected, lhs.concatenate(&rhs));
    }

    #[fixture]
    pub fn map1() -> RangeMap {
        let mut builder = RangeMap::build();
        builder = builder.push((50, 98, 2));
        builder = builder.push((52, 50, 48));
        builder.fill_gaps()
    }

    #[fixture]
    pub fn map2() -> RangeMap {
        let mut builder = RangeMap::build();
        builder = builder.push((0, 15, 37));
        builder = builder.push((37, 52, 2));
        builder = builder.push((39, 0, 15));
        builder.fill_gaps()
    }

    #[rstest]
    fn test_concatanate_with_fixtures(map1: RangeMap, map2: RangeMap) {
        println!("{:?}", map1);
        println!("{:?}", map2);

        let expected = map1.concatenate(&map2);
        println!("{:?}", expected)
    }
}
