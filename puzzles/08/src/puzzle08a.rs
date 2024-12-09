use common::puzzle::PuzzlePart;

use crate::AntennaGrid;

pub struct Puzzle08a {}

impl PuzzlePart for Puzzle08a {
    fn description() -> &'static str {
        "Count the number of 'simple antinodes' formed by pairs of antennae within a grid"
    }

    fn solve(input: &str) -> String {
        let grid = AntennaGrid::parse(input);

        grid.all_simple_antinodes()
            .iter()
            .filter(|node| grid.contains(**node))
            .count()
            .to_string()
    }
}
