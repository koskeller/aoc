use itertools::Itertools;
use std::collections::HashSet;

pub fn solve_hashset(input: &str, size: usize) -> usize {
    input
        .as_bytes()
        .windows(size)
        .position(|window| window.iter().collect::<HashSet<_>>().len() == size)
        .map(|pos| pos + size)
        .unwrap()
}

// Itertools have `unique` method, that uses HashSet under the hood.
pub fn solve_unique(input: &str, size: usize) -> usize {
    input
        .as_bytes()
        .windows(size)
        .position(|window| window.iter().unique().count() == size)
        .map(|pos| pos + size)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("input");
        assert_eq!(solve_hashset(input, 4), 1920);
        assert_eq!(solve_unique(input, 4), 1920);

        assert_eq!(solve_hashset(input, 14), 1920);
        assert_eq!(solve_unique(input, 14), 1920);
    }
}
