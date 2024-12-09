use crate::{Equation, Op};
use common::puzzle::PuzzlePart;

pub struct Puzzle07a {}

impl PuzzlePart for Puzzle07a {
    fn description() -> &'static str {
        "Sum the 'test' values that can be forms by adding/multiplying other numbers."
    }

    fn solve(input: &str) -> String {
        let ops = vec![Op::Add, Op::Mul];

        input
            .lines()
            .map(Equation::parse)
            .filter(|eqn| eqn.is_equalable(&ops))
            .map(|eqn| eqn.lhs)
            .sum::<i64>()
            .to_string()
    }
}
