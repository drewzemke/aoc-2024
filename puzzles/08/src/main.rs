use common::puzzle::Puzzle;
use puzzle08::{puzzle08a::Puzzle08a, puzzle08b::Puzzle08b};

fn main() {
    let input = include_str!("../data/input");
    let example = include_str!("../data/example");

    Puzzle08::run(input, example);
}

struct Puzzle08 {}

impl Puzzle for Puzzle08 {
    type PartA = Puzzle08a;
    type PartB = Puzzle08b;

    fn name() -> &'static str {
        "08"
    }
}
