Here's a simple Rust workspace structure for Advent of Code:

1. First, create a workspace root with `Cargo.toml`:

```toml
[workspace]
members = [
    "utils",
    "days/*"
]

[workspace.dependencies]
regex = "1.10"
itertools = "0.12"
```

2. Make a `utils` folder with common code:

```bash
utils/
  Cargo.toml
  src/
    lib.rs
```

3. Add each day as a binary crate:

```bash
days/
  day01/
    Cargo.toml
    src/
      main.rs
  day02/
    Cargo.toml
    src/
      main.rs
```

4. In each day's `Cargo.toml`:

```toml
[package]
name = "day01"
version = "0.1.0"

[dependencies]
utils = { path = "../../utils" }
regex = { workspace = true }
```

5. Example `main.rs` for a day:

```rust
use utils::read_input;

fn main() {
    let input = read_input("day01");
    println!("Part 1: {}", solve_part1(&input));
    println!("Part 2: {}", solve_part2(&input));
}
```

To run a specific day:

```bash
cargo run -p day01
