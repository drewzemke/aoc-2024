use common::puzzle::Puzzle;
use puzzle17::{puzzle17a::Puzzle17a, puzzle17b::Puzzle17b};

fn main() {
    let input = include_str!("../data/input");
    let example = include_str!("../data/example");

    Puzzle17::run(input, example);
}

struct Puzzle17 {}

impl Puzzle for Puzzle17 {
    type PartA = Puzzle17a;
    type PartB = Puzzle17b;

    fn name() -> &'static str {
        "17"
    }
}
