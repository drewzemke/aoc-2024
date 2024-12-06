use crate::Grid;
use common::PuzzlePart;

pub struct Puzzle04a {}

const PATTERNS: [&str; 8] = [
    "XMAS",
    "SAMX",
    "X\nM\nA\nS",
    "S\nA\nM\nX",
    "X...\n.M..\n..A.\n...S",
    "S...\n.A..\n..M.\n...X",
    "...X\n..M.\n.A..\nS...",
    "...S\n..A.\n.M..\nX...",
];

impl PuzzlePart for Puzzle04a {
    fn description() -> &'static str {
        "Count the number of times 'XMAS' appears in a grid."
    }

    fn solve(input: &str) -> String {
        let grid = Grid::parse(input);

        PATTERNS
            .into_iter()
            .map(Grid::parse)
            .map(|pattern| grid.count_matches(&pattern))
            .sum::<usize>()
            .to_string()
    }
}
