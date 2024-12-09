use crate::{Equation, Op, OpTree};
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
            .filter(|eqn| {
                OpTree::all_left_assoc_trees(&eqn.rhs, &ops)
                    .iter()
                    .any(|tree| eqn.lhs == tree.eval())
            })
            .map(|eqn| eqn.lhs)
            .sum::<i64>()
            .to_string()
    }
}
