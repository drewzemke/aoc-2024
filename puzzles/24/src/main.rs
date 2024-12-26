use common::puzzle::Puzzle;
use puzzle24::{puzzle24a::Puzzle24a, puzzle24b::Puzzle24b};

fn main() {
    let input = include_str!("../data/input");
    let example = include_str!("../data/example");

    Puzzle24::run(input, example);
}

struct Puzzle24 {}

impl Puzzle for Puzzle24 {
    type PartA = Puzzle24a;
    type PartB = Puzzle24b;

    fn name() -> &'static str {
        "24"
    }
}
