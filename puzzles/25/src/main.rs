use common::puzzle::Puzzle;
use puzzle25::{puzzle25a::Puzzle25a, puzzle25b::Puzzle25b};

fn main() {
    let input = include_str!("../data/input");
    let example = include_str!("../data/example");

    Puzzle25::run(input, example);
}

struct Puzzle25 {}

impl Puzzle for Puzzle25 {
    type PartA = Puzzle25a;
    type PartB = Puzzle25b;

    fn name() -> &'static str {
        "25"
    }
}
