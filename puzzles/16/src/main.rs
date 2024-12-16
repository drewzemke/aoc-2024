use common::puzzle::Puzzle;
use puzzle16::{puzzle16a::Puzzle16a, puzzle16b::Puzzle16b};

fn main() {
    let input = include_str!("../data/input");
    let example = include_str!("../data/example");

    Puzzle16::run(input, example);
}

struct Puzzle16 {}

impl Puzzle for Puzzle16 {
    type PartA = Puzzle16a;
    type PartB = Puzzle16b;

    fn name() -> &'static str {
        "16"
    }
}
