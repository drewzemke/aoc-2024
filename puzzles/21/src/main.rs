use common::puzzle::Puzzle;
use puzzle21::{puzzle21a::Puzzle21a, puzzle21b::Puzzle21b};

fn main() {
    let input = include_str!("../data/input");
    let example = include_str!("../data/example");

    Puzzle21::run(input, example);
}

struct Puzzle21 {}

impl Puzzle for Puzzle21 {
    type PartA = Puzzle21a;
    type PartB = Puzzle21b;

    fn name() -> &'static str {
        "21"
    }
}
