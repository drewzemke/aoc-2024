use common::puzzle::Puzzle;
use puzzle02::{puzzle02a::Puzzle02a, puzzle02b::Puzzle02b};

fn main() {
    let input = include_str!("../data/input");
    let example = include_str!("../data/example");

    Puzzle02::run(input, example);
}

struct Puzzle02 {}

impl Puzzle for Puzzle02 {
    type PartA = Puzzle02a;
    type PartB = Puzzle02b;

    fn name() -> &'static str {
        "02"
    }
}
