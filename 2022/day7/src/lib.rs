use std::cell::RefCell;
use std::rc::Rc;

use camino::Utf8PathBuf;
use indexmap::IndexMap;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::all_consuming;
use nom::sequence::{preceded, separated_pair};
use nom::{bytes::complete::take_while1, combinator::map, Finish, IResult};

fn parse_path(i: &str) -> IResult<&str, Utf8PathBuf> {
    map(
        take_while1(|c: char| "abcdefghijklmnopqrstuvwxyz./".contains(c)),
        Into::into,
    )(i)
}

struct Ls;

fn parse_ls(i: &str) -> IResult<&str, Ls> {
    map(tag("ls"), |_| Ls)(i)
}

struct Cd(Utf8PathBuf);

fn parser_cd(i: &str) -> IResult<&str, Cd> {
    map(preceded(tag("cd "), parse_path), Cd)(i)
}

enum Command {
    Ls,
    Cd(Utf8PathBuf),
}

impl From<Ls> for Command {
    fn from(_lw: Ls) -> Self {
        Command::Ls
    }
}

impl From<Cd> for Command {
    fn from(cd: Cd) -> Self {
        Command::Cd(cd.0)
    }
}

fn parse_command(i: &str) -> IResult<&str, Command> {
    let (i, _) = tag("$ ")(i)?;
    alt((map(parse_ls, Into::into), map(parser_cd, Into::into)))(i)
}

enum Entry {
    Dir(Utf8PathBuf),
    File(u64, Utf8PathBuf),
}

fn parse_entry(i: &str) -> IResult<&str, Entry> {
    let parse_file = map(
        separated_pair(nom::character::complete::u64, tag(" "), parse_path),
        |(size, path)| Entry::File(size, path),
    );
    let parse_dir = map(preceded(tag("dir "), parse_path), Entry::Dir);

    alt((parse_file, parse_dir))(i)
}

enum Line {
    Command(Command),
    Entry(Entry),
}

fn parse_line(i: &str) -> IResult<&str, Line> {
    alt((
        map(parse_command, Line::Command),
        map(parse_entry, Line::Entry),
    ))(i)
}

type NodeHandle = Rc<RefCell<Node>>;

#[derive(Default)]
struct Node {
    size: usize,
    children: IndexMap<Utf8PathBuf, NodeHandle>,
    parent: Option<NodeHandle>,
}

impl Node {
    fn is_dir(&self) -> bool {
        self.size == 0 && !self.children.is_empty()
    }

    fn total_size(&self) -> u64 {
        self.children
            .values()
            .map(|child| child.borrow().total_size())
            .sum::<u64>()
            + self.size as u64
    }
}

fn all_dirs(n: NodeHandle) -> Box<dyn Iterator<Item = NodeHandle>> {
    // clippy is wrong and should feel bad
    #[allow(clippy::needless_collect)]
    let children = n.borrow().children.values().cloned().collect::<Vec<_>>();

    Box::new(
        std::iter::once(n).chain(
            children
                .into_iter()
                .filter_map(|c| {
                    if c.borrow().is_dir() {
                        Some(all_dirs(c))
                    } else {
                        None
                    }
                })
                .flatten(),
        ),
    )
}

fn get_tree(input: &str) -> NodeHandle {
    let lines = input
        .lines()
        .map(|l| all_consuming(parse_line)(l).finish().unwrap().1);

    let root = Rc::new(RefCell::new(Node::default()));
    let mut node = root.clone();

    for line in lines {
        match line {
            Line::Command(cmd) => match cmd {
                Command::Ls => {}
                Command::Cd(path) => match path.as_str() {
                    "/" => {}
                    ".." => {
                        let parent = node.borrow().parent.clone().unwrap();
                        node = parent;
                    }
                    _ => {
                        let child = node.borrow_mut().children.entry(path).or_default().clone();
                        node = child;
                    }
                },
            },
            Line::Entry(entry) => match entry {
                Entry::Dir(dir) => {
                    let entry = node.borrow_mut().children.entry(dir).or_default().clone();
                    entry.borrow_mut().parent = Some(node.clone());
                }
                Entry::File(size, file) => {
                    let entry = node.borrow_mut().children.entry(file).or_default().clone();
                    entry.borrow_mut().size = size as usize;
                    entry.borrow_mut().parent = Some(node.clone());
                }
            },
        }
    }

    root
}

pub fn solve(input: &str) -> usize {
    let root = get_tree(input);

    all_dirs(root)
        .map(|d| d.borrow().total_size())
        .filter(|&s| s <= 100_000)
        .sum::<u64>() as usize
}

pub fn solve_part2(input: &str) -> usize {
    let total_space = 70_000_000_u64;
    let needed_free_space = 30_000_000_u64;

    let root = get_tree(input);
    let used_space = root.borrow().total_size();
    let free_space = total_space.checked_sub(used_space).unwrap();
    let minimum_space_to_free = needed_free_space.checked_sub(free_space).unwrap();

    all_dirs(root)
        .map(|d| d.borrow().total_size())
        .filter(|&s| s >= minimum_space_to_free)
        .min()
        .unwrap() as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("input");
        assert_eq!(solve(input), 1845346);
        assert_eq!(solve_part2(input), 3636703);
    }
}
