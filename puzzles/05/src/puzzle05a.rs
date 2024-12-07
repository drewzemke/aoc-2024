use crate::{parse_input, Update};
use common::puzzle::PuzzlePart;

pub struct Puzzle05a {}

impl PuzzlePart for Puzzle05a {
    fn description() -> &'static str {
        "Sum the middle numbers of correctly-ordered lists."
    }

    fn solve(input: &str) -> String {
        let (pairs, updates) = parse_input(input);

        updates
            .iter()
            .filter(|update| update.is_sorted_by(&pairs))
            .map(Update::middle)
            .sum::<u64>()
            .to_string()
    }
}
