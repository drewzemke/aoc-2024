use common::puzzle::Puzzle;
use puzzle22::{puzzle22a::Puzzle22a, puzzle22b::Puzzle22b};

fn main() {
    let input = include_str!("../data/input");
    let example = include_str!("../data/example");

    Puzzle22::run(input, example);
}

struct Puzzle22 {}

impl Puzzle for Puzzle22 {
    type PartA = Puzzle22a;
    type PartB = Puzzle22b;

    fn name() -> &'static str {
        "22"
    }
}
