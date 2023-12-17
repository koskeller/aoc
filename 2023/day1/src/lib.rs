use std::collections::HashMap;

pub fn part_one(input: &str) -> usize {
    input
        .lines()
        .map(|s| {
            let ch = s
                .chars()
                .filter(|n| match n {
                    '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => true,
                    _ => false,
                })
                .collect::<Vec<char>>();
            let first = ch.first().unwrap();
            let last = ch.last().unwrap();
            let n = format!("{first}{last}");
            n.parse::<usize>().unwrap()
        })
        .sum()
}

pub fn part_two(input: &str) -> usize {
    let mut digits = HashMap::new();
    digits.insert("one", 1);
    digits.insert("two", 2);
    digits.insert("three", 3);
    digits.insert("four", 4);
    digits.insert("five", 5);
    digits.insert("six", 6);
    digits.insert("seven", 7);
    digits.insert("eight", 8);
    digits.insert("nine", 9);
    digits.insert("1", 1);
    digits.insert("2", 2);
    digits.insert("3", 3);
    digits.insert("4", 4);
    digits.insert("5", 5);
    digits.insert("6", 6);
    digits.insert("7", 7);
    digits.insert("8", 8);
    digits.insert("9", 9);

    let mut result = vec![];

    for line in input.lines() {
        let mut numbers = vec![];
        let mut i = 0;
        while i < line.len() {
            for (word, &value) in &digits {
                if line[i..].starts_with(word) {
                    numbers.push(value);
                    break;
                }
            }
            i += 1;
        }
        if let (Some(&first), Some(&last)) = (numbers.first(), numbers.last()) {
            result.push(first * 10 + last);
        }
    }

    result.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = include_str!("input");
        assert_eq!(part_one(input), 54877);
    }

    #[test]
    fn test_part_two() {
        let input = include_str!("input");
        assert_eq!(part_two(input), 281);
    }
}
