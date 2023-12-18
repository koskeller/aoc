#![allow(unused)]

use regex::Regex;
use std::{collections::HashMap, ops::Range};

fn part_one(input: &str) -> usize {
    get_schema(input)
        .into_iter()
        .map(|(_, v)| v.iter().sum::<usize>())
        .sum()
}

fn part_two(input: &str) -> usize {
    get_schema(input)
        .into_iter()
        .filter(|(_, v)| v.len() == 2)
        .map(|(_, v)| v.iter().fold(1, |acc, &x| acc * x))
        .sum()
}

fn get_schema(input: &str) -> HashMap<(i32, i32), Vec<usize>> {
    let mut set = parse_board(input);
    let re = Regex::new(r"\d+").unwrap();
    for (line_index, line) in input.lines().enumerate() {
        for number in re.find_iter(line) {
            let edges = get_edges(line_index as i32, number.range());
            for edge in &edges {
                if set.contains_key(edge) {
                    set.entry(*edge)
                        .and_modify(|v| v.push(number.as_str().parse::<usize>().unwrap()));
                    break;
                }
            }
        }
    }
    set
}

fn parse_board(input: &str) -> HashMap<(i32, i32), Vec<usize>> {
    let mut set = HashMap::new();
    for (line_index, line) in input.lines().enumerate() {
        for (char_index, char) in line.chars().enumerate() {
            if !char.is_digit(10) && char != '.' {
                set.insert((line_index as i32, char_index as i32), Vec::new());
            }
        }
    }
    set
}

// fn get_edges(line: i32, pos: i32) -> Vec<(i32, i32)> {
//     vec![
//         (line - 1, pos - 1),
//         (line - 1, pos),
//         (line - 1, pos + 1),
//         (line, pos - 1),
//         (line, pos + 1),
//         (line + 1, pos - 1),
//         (line + 1, pos),
//         (line + 1, pos + 1),
//     ]
// }

fn get_edges(line: i32, range: Range<usize>) -> Vec<(i32, i32)> {
    let min = range.clone().min().unwrap() as i32;
    let max = range.clone().max().unwrap() as i32;
    let mut edges = vec![
        (line - 1, min - 1),
        (line - 1, max + 1),
        (line, min - 1),
        (line, max + 1),
        (line + 1, min - 1),
        (line + 1, max + 1),
    ];
    for n in range {
        edges.push((line - 1, n as i32));
        edges.push((line + 1, n as i32));
    }
    edges
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_board() {
        let board = parse_board("...*......\n..35..633.\n......#...");
        assert_eq!(board.len(), 2);
        assert!(board.contains_key(&(0, 3)));
        assert!(board.contains_key(&(2, 6)));
    }

    #[test]
    fn test_part_one() {
        let input = include_str!("input");
        assert_eq!(part_one(input), 530849);
    }

    #[test]
    fn test_part_two() {
        let input = include_str!("input");
        assert_eq!(part_two(input), 84900879);
    }
}
