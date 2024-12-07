use crate::Grid;
use common::PuzzlePart;

pub struct Puzzle06a {}

impl PuzzlePart for Puzzle06a {
    fn description() -> &'static str {
        "Count the number of spaces that a guard walks over while moving around a room."
    }

    fn solve(input: &str) -> String {
        let (grid, start, dir) = Grid::parse(input);

        grid.walked_points(start, dir).len().to_string()
    }
}
