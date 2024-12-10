use common::puzzle::Puzzle;
use puzzle09::{puzzle09a::Puzzle09a, puzzle09b::Puzzle09b};

fn main() {
    let input = include_str!("../data/input");
    let example = include_str!("../data/example");

    Puzzle09::run(input, example);
}

struct Puzzle09 {}

impl Puzzle for Puzzle09 {
    type PartA = Puzzle09a;
    type PartB = Puzzle09b;

    fn name() -> &'static str {
        "09"
    }
}
