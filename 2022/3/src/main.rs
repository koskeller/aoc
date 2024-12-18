use item::Item;
use std::collections::HashSet;

//  Because Item is declared in the same module where we use it from,
//  so we can bypass validation and just do `let a = Item(b'a');`
//  So we put it into separate module to prevent creating it directly.
mod item {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub(crate) struct Item(u8);

    impl Item {
        pub(crate) fn score(self) -> usize {
            match self {
                Item(b'a'..=b'z') => 1 + (self.0 - b'a') as usize,
                Item(b'A'..=b'Z') => 27 + (self.0 - b'A') as usize,
                _ => unreachable!(),
            }
        }
    }

    impl std::fmt::Debug for Item {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0 as char)
        }
    }

    impl TryFrom<u8> for Item {
        type Error = anyhow::Error;

        fn try_from(value: u8) -> Result<Self, Self::Error> {
            match value {
                b'a'..=b'z' | b'A'..=b'Z' => Ok(Item(value)),
                _ => anyhow::bail!("{} is not a valid item", value as char),
            }
        }
    }
}

pub fn solve(input: &str) -> usize {
    let mut total_score = 0;

    for line in input.lines() {
        let (first, second) = line.split_at(line.len() / 2);

        let first_items = first
            .bytes()
            .map(Item::try_from)
            .collect::<Result<HashSet<_>, _>>()
            .unwrap();

        let dupe_score = second
            .bytes()
            .map(Item::try_from)
            .find_map(|item| {
                item.ok().and_then(|item| {
                    first_items
                        .iter()
                        .copied()
                        .find(|&first_item| first_item == item)
                })
            })
            .expect("there should be exactly one duplicate")
            .score();
        total_score += dupe_score;
    }

    total_score
}

fn main() {
    let input = include_str!("input");
    println!("{}", solve(input));
}
