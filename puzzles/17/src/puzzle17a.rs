use crate::Computer;
use common::puzzle::PuzzlePart;
use itertools::Itertools;

pub struct Puzzle17a {}

impl PuzzlePart for Puzzle17a {
    fn description() -> &'static str {
        "Find the output of a 3-bit computer."
    }

    fn solve(input: &str) -> String {
        let mut computer = Computer::parse(input).unwrap();

        computer.run();

        computer.output.iter().map(u8::to_string).join(",")
    }
}
