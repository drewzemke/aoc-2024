use common::puzzle::Puzzle;
use puzzle18::{puzzle18a::Puzzle18a, puzzle18b::Puzzle18b};

fn main() {
    let input = include_str!("../data/input");
    let example = include_str!("../data/example");

    Puzzle18::run(input, example);
}

struct Puzzle18 {}

impl Puzzle for Puzzle18 {
    type PartA = Puzzle18a;
    type PartB = Puzzle18b;

    fn name() -> &'static str {
        "18"
    }
}
