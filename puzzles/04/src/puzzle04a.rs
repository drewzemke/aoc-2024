use common::PuzzlePart;

use crate::Grid;

pub struct Puzzle04a {}

impl PuzzlePart for Puzzle04a {
    fn description() -> &'static str {
        "Count the number of times 'XMAS' appears in a grid."
    }

    fn solve(input: &str) -> String {
        Grid::parse(input).find_xmases().len().to_string()
    }
}
