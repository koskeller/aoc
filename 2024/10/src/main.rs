#![allow(unused)]

use std::collections::HashSet;
use utils::Grid;

fn main() {
    let s = include_str!("input");
    println!("{}", part_one(s.to_owned()));
}

fn part_one(input: String) -> usize {
    let mut answer = 0;
    let grid = Grid::new(input.to_owned());

    // Starts at height 0, ends at height 9, and always increases by a height of exactly 1 at each step.
    let mut trailheads: Vec<(usize, usize)> = vec![];

    for row in 0..grid.rows() {
        for col in 0..grid.cols() {
            if grid.get(row, col) == 0 {
                trailheads.push((row, col));
            }
        }
    }

    for (row, col) in trailheads {
        let mut summits = HashSet::new();
        let mut paths = vec![(row, col)];

        while let Some((row, col)) = paths.pop() {
            let cur = grid.get(row, col);
            for (r, c) in grid.neighbors(row, col) {
                if cur == 8 && grid.get(r, c) == 9 {
                    summits.insert((r, c));
                } else if grid.get(r, c) == cur + 1 {
                    paths.push((r, c));
                }
            }
        }
        answer += summits.len();
    }

    return answer;
}
