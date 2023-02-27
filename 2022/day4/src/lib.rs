use itertools::Itertools;
use std::ops::RangeInclusive;

// In Rust, we can use "inclusive ranges" (std::ops::RangeInclusive).
#[allow(dead_code)]
fn contains_range(bigger: RangeInclusive<i32>, smaller: RangeInclusive<i32>) -> bool {
    bigger.contains(smaller.start()) && bigger.contains(smaller.end())
}

// Alternatively we can implement it as a method on RangeInclusive itself!
// And we can make that impl generic.
//
// This is a neat technique, because it lets us add methods to any type,
// even types we haven't declared ourselves (say, types that come from
// the standard library, or another crate).
trait InclusiveRangeExt {
    fn contains_range(&self, other: &Self) -> bool;

    // We can have trait methods with default implementations.
    fn contains_or_is_contained(&self, other: &Self) -> bool {
        self.contains_range(other) || other.contains_range(self)
    }

    fn overlaps(&self, other: &Self) -> bool;

    fn overlaps_or_is_overlapped(&self, other: &Self) -> bool {
        self.overlaps(other) || other.overlaps(self)
    }
}

impl<T> InclusiveRangeExt for RangeInclusive<T>
where
    T: PartialOrd,
{
    fn contains_range(&self, other: &Self) -> bool {
        self.contains(other.start()) && self.contains(other.end())
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.contains(other.start()) || self.contains(other.end())
    }
}

pub fn solve(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            l.split(',')
                .map(|range| {
                    range
                        .split('-')
                        .map(|n| n.parse().expect("range start/end should be u32"))
                        .collect_tuple::<(u32, u32)>()
                        .map(|(start, end)| start..=end)
                        .expect("each range should have start and end")
                })
                .collect_tuple::<(_, _)>()
                .expect("each line must have pair of ranges")
        })
        .filter(|(a, b)| a.contains_or_is_contained(b))
        .count()
}

pub fn solve_part2(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            l.split(',')
                .map(|range| {
                    range
                        .split('-')
                        .map(|n| n.parse().expect("range start/end should be u32"))
                        .collect_tuple::<(u32, u32)>()
                        .map(|(start, end)| start..=end)
                        .expect("each range should have start and end")
                })
                .collect_tuple::<(_, _)>()
                .expect("each line must have pair of ranges")
        })
        .filter(|(a, b)| a.overlaps_or_is_overlapped(b))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("input");
        assert_eq!(solve(input), 605);
        assert_eq!(solve_part2(input), 914);
    }
}
