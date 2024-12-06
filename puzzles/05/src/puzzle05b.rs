use crate::parse_input;
use common::PuzzlePart;

pub struct Puzzle05b {}

impl PuzzlePart for Puzzle05b {
    fn description() -> &'static str {
        "Sum the middle numbers of incorrectly-ordered lists (after ordering them)."
    }

    fn solve(input: &str) -> String {
        let (pairs, mut updates) = parse_input(input);

        #[expect(clippy::manual_inspect)]
        updates
            .iter_mut()
            .filter(|update| !update.is_sorted_by(&pairs))
            .map(|update| {
                update.sort_by(&pairs);
                update
            })
            .map(|update| update.middle())
            .sum::<u64>()
            .to_string()
    }
}
