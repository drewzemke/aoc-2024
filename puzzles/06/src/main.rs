use common::puzzle::Puzzle;
use puzzle06::{puzzle06a::Puzzle06a, puzzle06b::Puzzle06b};

fn main() {
    let input = include_str!("../data/input");
    let example = include_str!("../data/example");

    Puzzle06::run(input, example);
}

struct Puzzle06 {}

impl Puzzle for Puzzle06 {
    type PartA = Puzzle06a;
    type PartB = Puzzle06b;

    fn name() -> &'static str {
        "06"
    }
}
