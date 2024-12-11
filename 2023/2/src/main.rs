#![allow(unused)]

fn main() {
    let input = include_str!("input");
    println!("{}", part_one(input));
    println!("{}", part_two(input));
}

pub fn part_one(input: &str) -> u32 {
    input.lines().filter(is_valid_game).map(parse_game_id).sum()
}

pub fn part_two(input: &str) -> u32 {
    input.lines().map(fewest_cubes_power).sum()
}

fn is_valid_game(game: &&str) -> bool {
    for set in parse_sets(game) {
        let set = parse_set(set);
        if set.red > 12 || set.green > 13 || set.blue > 14 {
            return false;
        }
    }
    true
}

fn fewest_cubes_power(game: &str) -> u32 {
    let mut min_red = 0;
    let mut min_green = 0;
    let mut min_blue = 0;
    for set in parse_sets(game) {
        let set = parse_set(set);
        if min_red < set.red {
            min_red = set.red;
        }
        if min_green < set.green {
            min_green = set.green;
        }
        if min_blue < set.blue {
            min_blue = set.blue;
        }
    }

    min_red * min_green * min_blue
}

fn parse_game_id(s: &str) -> u32 {
    if let (Some(before), Some(after)) = (s.find(" "), s.find(":")) {
        s[before + 1..after].parse::<u32>().unwrap_or_default()
    } else {
        0
    }
}

fn parse_sets(s: &str) -> Vec<&str> {
    if let Some(start) = s.find(":") {
        s[start + 1..].split(";").map(|s| s.trim()).collect()
    } else {
        vec![]
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Set {
    pub blue: u32,
    pub red: u32,
    pub green: u32,
}

fn parse_set(s: &str) -> Set {
    let mut set = Set {
        blue: 0,
        red: 0,
        green: 0,
    };

    for part in s.split(", ") {
        let mut iter = part.split(" ");
        if let (Some(number), Some(color)) = (iter.next(), iter.next()) {
            match color {
                "blue" => set.blue = number.parse().unwrap_or_default(),
                "red" => set.red = number.parse().unwrap_or_default(),
                "green" => set.green = number.parse().unwrap_or_default(),
                _ => panic!("Unknown color"),
            }
        }
    }

    set
}
