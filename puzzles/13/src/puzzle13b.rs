use common::puzzle::PuzzlePart;

use crate::ClawMachine;

pub struct Puzzle13b {}

impl PuzzlePart for Puzzle13b {
    fn description() -> &'static str {
        "Find the fewest number of tokens needed to win prizes in some BIG claw machines."
    }

    fn solve(input: &str) -> String {
        input
            .split("\n\n")
            .map(str::trim)
            .map(ClawMachine::parse)
            .map(Option::unwrap)
            .map(|mut m| {
                m.embiggen();
                m
            })
            .filter_map(|m| m.solve())
            // button A costs 3, button B costs 1
            .map(|(a, b)| 3 * a + b)
            .sum::<i64>()
            .to_string()
    }
}
