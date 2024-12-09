use common::puzzle::Puzzle;
use puzzle07::{puzzle07a::Puzzle07a, puzzle07b::Puzzle07b};

fn main() {
    let input = include_str!("../data/input");
    let example = include_str!("../data/example");

    Puzzle07::run(input, example);
}

struct Puzzle07 {}

impl Puzzle for Puzzle07 {
    type PartA = Puzzle07a;
    type PartB = Puzzle07b;

    fn name() -> &'static str {
        "07"
    }
}
