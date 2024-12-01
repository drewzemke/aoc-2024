use common::puzzle::Puzzle;
use puzzleDAYNUM::{puzzleDAYNUMa::PuzzleDAYNUMa, puzzleDAYNUMb::PuzzleDAYNUMb};

fn main() {
    let input = include_str!("../data/input");
    let example = include_str!("../data/example");

    PuzzleDAYNUM::run(input, example);
}

struct PuzzleDAYNUM {}

impl Puzzle for PuzzleDAYNUM {
    type PartA = PuzzleDAYNUMa;
    type PartB = PuzzleDAYNUMb;

    fn name() -> &'static str {
        "DAYNUM"
    }
}
