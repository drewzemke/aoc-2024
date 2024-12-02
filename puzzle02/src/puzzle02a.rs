use crate::Report;
use common::puzzle::PuzzlePart;

pub struct Puzzle02a {}

impl PuzzlePart for Puzzle02a {
    fn description() -> &'static str {
        "Count the number of 'safe' reports."
    }

    fn solve(input: &str) -> String {
        input
            .lines()
            .map(Report::parse_from_str)
            .filter(Report::is_safe)
            .count()
            .to_string()
    }
}
