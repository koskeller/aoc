#![allow(unused)]

/// This makes two assumptions that ended up being true for the input:
/// - No number touches more than one symbol
/// - No non-star symbol touches exactly two numbers
use regex::Regex;
use std::{collections::HashMap, ops::Range};

fn part_one(input: &str) -> usize {
    get_schematic(input)
        .into_iter()
        .map(|(_, v)| v.iter().sum::<usize>())
        .sum()
}

fn part_two(input: &str) -> usize {
    get_schematic(input)
        .into_iter()
        .filter(|(_, v)| v.len() == 2)
        .map(|(_, v)| v.iter().fold(1, |acc, &x| acc * x))
        .sum()
}

fn get_schematic(input: &str) -> HashMap<(i32, i32), Vec<usize>> {
    let mut hm = parse_schematic(input);

    let re = Regex::new(r"\d+").unwrap();

    for (line_index, line) in input.lines().enumerate() {
        for number in re.find_iter(line) {
            let edges = get_edges(line_index as i32, number.range());
            let number_value = number.as_str().parse::<usize>().unwrap();
            for edge in &edges {
                if let Some(v) = hm.get_mut(edge) {
                    v.push(number_value);
                    break;
                }
            }
        }
    }

    hm
}

// Coordinates of all symbols mapped to values of adjusted numbers.
fn parse_schematic(input: &str) -> HashMap<(i32, i32), Vec<usize>> {
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

// Coordinates of all edges.
fn get_edges(line: i32, range: Range<usize>) -> Vec<(i32, i32)> {
    let min = range.start as i32 - 1;
    let max = range.end as i32;
    let mut edges = vec![(line, min), (line, max)];
    for n in min..=max {
        edges.extend([(line - 1, n as i32), (line + 1, n as i32)]);
    }
    edges
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_board() {
        let board = parse_schematic("...*......\n..35..633.\n......#...");
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
