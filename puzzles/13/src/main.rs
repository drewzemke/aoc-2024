use common::puzzle::Puzzle;
use puzzle13::{puzzle13a::Puzzle13a, puzzle13b::Puzzle13b};

fn main() {
    let input = include_str!("../data/input");
    let example = include_str!("../data/example");

    Puzzle13::run(input, example);
}

struct Puzzle13 {}

impl Puzzle for Puzzle13 {
    type PartA = Puzzle13a;
    type PartB = Puzzle13b;

    fn name() -> &'static str {
        "13"
    }
}
