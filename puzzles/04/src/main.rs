use common::puzzle::Puzzle;
use puzzle04::{puzzle04a::Puzzle04a, puzzle04b::Puzzle04b};

fn main() {
    let input = include_str!("../data/input");
    let example = include_str!("../data/example");

    Puzzle04::run(input, example);
}

struct Puzzle04 {}

impl Puzzle for Puzzle04 {
    type PartA = Puzzle04a;
    type PartB = Puzzle04b;

    fn name() -> &'static str {
        "04"
    }
}
