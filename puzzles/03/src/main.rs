use common::puzzle::Puzzle;
use puzzle03::{puzzle03a::Puzzle03a, puzzle03b::Puzzle03b};

fn main() {
    let input = include_str!("../data/input");
    let example = include_str!("../data/example");

    Puzzle03::run(input, example);
}

struct Puzzle03 {}

impl Puzzle for Puzzle03 {
    type PartA = Puzzle03a;
    type PartB = Puzzle03b;

    fn name() -> &'static str {
        "03"
    }
}
