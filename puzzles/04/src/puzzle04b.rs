use crate::XmasGrid;
use common::{grid::Grid, puzzle::PuzzlePart};

pub struct Puzzle04b {}

const PATTERNS: [&str; 4] = [
    "M.M\n.A.\nS.S",
    "S.S\n.A.\nM.M",
    "S.M\n.A.\nS.M",
    "M.S\n.A.\nM.S",
];

impl PuzzlePart for Puzzle04b {
    fn description() -> &'static str {
        "Count the number of times X's of 'MAS' appear in a grid."
    }

    fn solve(input: &str) -> String {
        let grid = XmasGrid::parse(input);

        PATTERNS
            .into_iter()
            .map(Grid::parse)
            .map(|pattern| grid.count_matches(&pattern))
            .sum::<usize>()
            .to_string()
    }
}
