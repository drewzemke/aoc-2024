use common::puzzle::Puzzle;
use puzzle11::{puzzle11a::Puzzle11a, puzzle11b::Puzzle11b};

fn main() {
    let input = include_str!("../data/input");
    let example = include_str!("../data/example");

    Puzzle11::run(input, example);
}

struct Puzzle11 {}

impl Puzzle for Puzzle11 {
    type PartA = Puzzle11a;
    type PartB = Puzzle11b;

    fn name() -> &'static str {
        "11"
    }
}
