use crate::Report;
use common::puzzle::PuzzlePart;

pub struct Puzzle02b {}

impl PuzzlePart for Puzzle02b {
    fn description() -> &'static str {
        "Count the number of 'safe' reports with at most one abberation."
    }

    fn solve(input: &str) -> String {
        input
            .lines()
            .map(Report::parse_from_str)
            .filter(Report::is_almost_safe)
            .count()
            .to_string()
    }
}
