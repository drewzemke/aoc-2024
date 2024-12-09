use crate::AntennaGrid;
use common::puzzle::PuzzlePart;

pub struct Puzzle08b {}

impl PuzzlePart for Puzzle08b {
    fn description() -> &'static str {
        "Count the number of 'general antinodes' formed by pairs of antennae within a grid"
    }

    fn solve(input: &str) -> String {
        let grid = AntennaGrid::parse(input);

        grid.all_general_antinodes().len().to_string()
    }
}
