use common::puzzle::Puzzle;
use puzzle23::{puzzle23a::Puzzle23a, puzzle23b::Puzzle23b};

fn main() {
    let input = include_str!("../data/input");
    let example = include_str!("../data/example");

    Puzzle23::run(input, example);
}

struct Puzzle23 {}

impl Puzzle for Puzzle23 {
    type PartA = Puzzle23a;
    type PartB = Puzzle23b;

    fn name() -> &'static str {
        "23"
    }
}
