use crate::{parse_instructions, Mul};
use common::puzzle::PuzzlePart;

pub struct Puzzle03b {}

impl PuzzlePart for Puzzle03b {
    fn description() -> &'static str {
        "Sum the *enabled* 'mul' expressions found in a corrupted string"
    }

    fn solve(input: &str) -> String {
        let mut enabled = true;

        parse_instructions(input)
            .iter()
            .filter_map(|inst| match inst {
                crate::Instruction::Do => {
                    enabled = true;
                    None
                }
                crate::Instruction::Dont => {
                    enabled = false;
                    None
                }
                crate::Instruction::Mul(mul) => {
                    if enabled {
                        Some(mul)
                    } else {
                        None
                    }
                }
            })
            .map(|Mul(x, y)| *x * *y)
            .sum::<u64>()
            .to_string()
    }
}
