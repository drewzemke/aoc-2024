use common::puzzle::Puzzle;
use puzzle01::{puzzle01a::Puzzle01a, puzzle01b::Puzzle01b};

fn main() {
    let input = include_str!("../data/input");
    let example = include_str!("../data/example");

    Puzzle01::run(input, example);
}

struct Puzzle01 {}

impl Puzzle for Puzzle01 {
    type PartA = Puzzle01a;
    type PartB = Puzzle01b;

    fn name() -> &'static str {
        "01"
    }
}
