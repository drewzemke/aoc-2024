use common::puzzle::Puzzle;
use puzzle10::{puzzle10a::Puzzle10a, puzzle10b::Puzzle10b};

fn main() {
    let input = include_str!("../data/input");
    let example = include_str!("../data/example");

    Puzzle10::run(input, example);
}

struct Puzzle10 {}

impl Puzzle for Puzzle10 {
    type PartA = Puzzle10a;
    type PartB = Puzzle10b;

    fn name() -> &'static str {
        "10"
    }
}
