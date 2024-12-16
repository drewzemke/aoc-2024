use common::puzzle::Puzzle;
use puzzle15::{puzzle15a::Puzzle15a, puzzle15b::Puzzle15b};

fn main() {
    let input = include_str!("../data/input");
    let example = include_str!("../data/example");

    Puzzle15::run(input, example);
}

struct Puzzle15 {}

impl Puzzle for Puzzle15 {
    type PartA = Puzzle15a;
    type PartB = Puzzle15b;

    fn name() -> &'static str {
        "15"
    }
}
