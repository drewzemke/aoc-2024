use common::puzzle::Puzzle;
use puzzle20::{puzzle20a::Puzzle20a, puzzle20b::Puzzle20b};

fn main() {
    let input = include_str!("../data/input");
    let example = include_str!("../data/example");

    Puzzle20::run(input, example);
}

struct Puzzle20 {}

impl Puzzle for Puzzle20 {
    type PartA = Puzzle20a;
    type PartB = Puzzle20b;

    fn name() -> &'static str {
        "20"
    }
}
