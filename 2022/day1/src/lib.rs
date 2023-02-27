use itertools::{FoldWhile, Itertools};
use std::{cmp::Reverse, collections::BinaryHeap};

pub fn solve(input: &str) -> usize {
    let mut max = 0;
    for group in input.split("\n\n") {
        let mut sum = 0;
        for line in group.lines() {
            let value = line.parse::<usize>().unwrap();
            sum += value;
        }
        if sum > max {
            max = sum;
        }
    }

    max
}

pub fn solve_func(input: &str) -> usize {
    let lines = input
        .lines()
        .map(|v| v.parse::<usize>().ok())
        .collect::<Vec<_>>();
    lines
        .split(|line| line.is_none())
        .map(|group| group.iter().map(|v| v.unwrap()).sum::<usize>())
        .max()
        .unwrap()
}

pub fn solve_bonus(input: &str) -> usize {
    let mut group_sums = input
        .lines()
        .map(|v| v.parse::<usize>().ok())
        .batching(|it| {
            it.fold_while(None, |acc: Option<usize>, v| match v {
                Some(v) => FoldWhile::Continue(Some(acc.unwrap_or_default() + v)),
                None => FoldWhile::Done(acc),
            })
            .into_inner()
        })
        .map(Reverse);

    let mut heap = BinaryHeap::new();

    for init in (&mut group_sums).take(3) {
        heap.push(init);
    }

    for rest in group_sums {
        heap.push(rest);
        heap.pop();
    }

    heap.into_iter().map(|Reverse(v)| v).sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("input");
        assert_eq!(solve(input), 69883);
        assert_eq!(solve_func(input), 69883);
        assert_eq!(solve_bonus(input), 207576);
    }
}
