use common::puzzle::PuzzlePart;

use crate::Device;

pub struct Puzzle24a {}

impl PuzzlePart for Puzzle24a {
    fn description() -> &'static str {
        "Compute the decimal number output by a digital circuit of logic gates."
    }

    fn solve(input: &str) -> String {
        let mut device = Device::parse(input).unwrap();
        device.compute();
        device.output().to_string()
    }
}
