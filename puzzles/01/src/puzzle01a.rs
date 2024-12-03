use crate::parse_two_lists;
use common::PuzzlePart;

pub struct Puzzle01a {}

impl PuzzlePart for Puzzle01a {
    fn description() -> &'static str {
        "Sum the (absolute) differences between correspondingly ordered elements of two lists."
    }

    fn solve(input: &str) -> String {
        let (mut left, mut right) = parse_two_lists(input);

        left.sort_unstable();
        right.sort_unstable();

        left.iter()
            .zip(right.iter())
            .map(|(left, right)| left.abs_diff(*right))
            .sum::<u64>()
            .to_string()
    }
}
