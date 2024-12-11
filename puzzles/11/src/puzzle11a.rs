use common::puzzle::PuzzlePart;

use crate::{count_descs, parse};

pub struct Puzzle11a {}

impl PuzzlePart for Puzzle11a {
    fn description() -> &'static str {
        "Compute the number of stones after 25 steps of a replicating process."
    }

    fn solve(input: &str) -> String {
        parse(input)
            .into_iter()
            .map(|num| count_descs(num, 25))
            .sum::<u64>()
            .to_string()
    }
}
