use common::puzzle::Puzzle;
use puzzle12::{puzzle12a::Puzzle12a, puzzle12b::Puzzle12b};

fn main() {
    let input = include_str!("../data/input");
    let example = include_str!("../data/example");

    Puzzle12::run(input, example);
}

struct Puzzle12 {}

impl Puzzle for Puzzle12 {
    type PartA = Puzzle12a;
    type PartB = Puzzle12b;

    fn name() -> &'static str {
        "12"
    }
}
