# 🎄 Advent of Code 2024! 🎄

## Usage

Set up a day's puzzle (from project root):
```shell
just setup <day_number>
```

Run puzzles from project root:
```shell
# solve both parts (A and B) of day 6's puzzles
just run 6

# solve part A of day 6
just run 6 a

# solve part A of day 6 with example input
just run 6 a e

# solve part B of day 6, built in release mode
just run 6 b r
```

Run puzzles from individual puzzle directory (eg. `./puzzle06`):
```shell
# solve both parts (A and B)
just run

# solve part A
just run a

# solve part A with example input
just run a e

# solve part B built in release mode
just run b r
```

## Required Tools

- Rust toolchain
- Deno 2.0
- Fish shell
- Just
