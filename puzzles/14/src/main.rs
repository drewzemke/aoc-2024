use common::puzzle::Puzzle;
use puzzle14::{puzzle14a::Puzzle14a, puzzle14b::Puzzle14b};

fn main() {
    let input = include_str!("../data/input");
    let example = include_str!("../data/example");

    Puzzle14::run(input, example);
}

struct Puzzle14 {}

impl Puzzle for Puzzle14 {
    type PartA = Puzzle14a;
    type PartB = Puzzle14b;

    fn name() -> &'static str {
        "14"
    }
}
