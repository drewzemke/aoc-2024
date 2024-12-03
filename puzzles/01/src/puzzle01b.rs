use crate::{count_occurrences, parse_two_lists};
use common::PuzzlePart;

pub struct Puzzle01b {}

impl PuzzlePart for Puzzle01b {
    fn description() -> &'static str {
        "Sum the 'similarity scores' of numbers in two lists."
    }

    fn solve(input: &str) -> String {
        let (left, right) = parse_two_lists(input);

        left.iter()
            // similarity score is the product of the value and
            // the number of times it appears in the other list
            .map(|x| count_occurrences(*x, &right) * (*x as usize))
            .sum::<usize>()
            .to_string()
    }
}
