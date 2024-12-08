use crate::GuardGrid;
use common::puzzle::PuzzlePart;

pub struct Puzzle06a {}

impl PuzzlePart for Puzzle06a {
    fn description() -> &'static str {
        "Count the number of spaces that a guard walks over while moving around a room."
    }

    fn solve(input: &str) -> String {
        let (grid, start, dir) = GuardGrid::parse(input);

        grid.walk(start, dir).len().to_string()
    }
}
