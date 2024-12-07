use common::puzzle::Puzzle;
use puzzle05::{puzzle05a::Puzzle05a, puzzle05b::Puzzle05b};

fn main() {
    let input = include_str!("../data/input");
    let example = include_str!("../data/example");

    Puzzle05::run(input, example);
}

struct Puzzle05 {}

impl Puzzle for Puzzle05 {
    type PartA = Puzzle05a;
    type PartB = Puzzle05b;

    fn name() -> &'static str {
        "05"
    }
}
