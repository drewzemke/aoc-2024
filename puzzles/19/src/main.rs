use common::puzzle::Puzzle;
use puzzle19::{puzzle19a::Puzzle19a, puzzle19b::Puzzle19b};

fn main() {
    let input = include_str!("../data/input");
    let example = include_str!("../data/example");

    Puzzle19::run(input, example);
}

struct Puzzle19 {}

impl Puzzle for Puzzle19 {
    type PartA = Puzzle19a;
    type PartB = Puzzle19b;

    fn name() -> &'static str {
        "19"
    }
}
