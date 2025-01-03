use crate::parse_locks_and_keys;
use common::puzzle::PuzzlePart;

pub struct Puzzle25a {}

impl PuzzlePart for Puzzle25a {
    fn description() -> &'static str {
        "Determine how many 5-pin lock/key pairs fit (loosely) together."
    }

    fn solve(input: &str) -> String {
        let (locks, keys) = parse_locks_and_keys(input);

        locks
            .iter()
            .map(|lock| keys.iter().filter(|key| key.loosely_fits(lock)).count())
            .sum::<usize>()
            .to_string()
    }
}
