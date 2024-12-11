#![allow(unused)]

use regex::Regex;
use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("input");
    println!("{}", part_one(input));
    println!("{}", part_two(input));
}

fn part_one(input: &str) -> u32 {
    input
        .lines()
        .map(count_winning_numbers)
        .map(calculate_points)
        .sum()
}

fn part_two(input: &str) -> u32 {
    let mut winnings = HashMap::new();
    let mut cards = HashMap::new();

    for (line_index, line) in input.lines().enumerate() {
        winnings.insert(line_index, count_winning_numbers(line));
        cards.insert(line_index, 1);
    }

    for i in 0..cards.len() {
        let winnings_count = winnings.get(&i).unwrap();
        let current_card_count = cards.get(&i).unwrap().clone();
        for n in (i + 1)..(1 + i + *winnings_count as usize) {
            let card = cards.get_mut(&n).unwrap();
            *card += current_card_count;
        }
    }

    cards.values().sum()
}

fn count_winning_numbers(input: &str) -> u32 {
    let (winning, numbers) = parse_card(input);
    let matched = winning.intersection(&numbers).collect::<Vec<&&str>>();
    matched.len() as u32
}

fn calculate_points(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => 2_u32.pow(n - 1),
    }
}

fn parse_card(input: &str) -> (HashSet<&str>, HashSet<&str>) {
    let start = input.find(":").unwrap_or_default();
    let split = input.find("|").unwrap_or_default();

    let mut winning = HashSet::new();
    let mut numbers = HashSet::new();

    let re = Regex::new(r"\d+").unwrap();
    for digit in re.find_iter(&input[start..split]) {
        winning.insert(digit.as_str());
    }
    for digit in re.find_iter(&input[split..]) {
        numbers.insert(digit.as_str());
    }

    (winning, numbers)
}
