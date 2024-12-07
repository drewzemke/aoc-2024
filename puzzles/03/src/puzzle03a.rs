use crate::{parse_all_muls, Mul};
use common::puzzle::PuzzlePart;

pub struct Puzzle03a {}

impl PuzzlePart for Puzzle03a {
    fn description() -> &'static str {
        "Sum the 'mul' expressions found in a corrupted string"
    }

    fn solve(input: &str) -> String {
        parse_all_muls(input)
            .iter()
            .map(|Mul(x, y)| *x * *y)
            .sum::<u64>()
            .to_string()
    }
}
