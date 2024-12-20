use std::collections::HashMap;

use common::puzzle::PuzzlePart;

pub struct Puzzle19a {}

impl PuzzlePart for Puzzle19a {
    fn description() -> &'static str {
        "Count how many stripe designs can be constructed from stripe patterns."
    }

    fn solve(input: &str) -> String {
        let patterns = input
            .lines()
            .next()
            .unwrap()
            .split(", ")
            .collect::<Vec<_>>();

        let designs = input.lines().skip(2).collect::<Vec<_>>();
        let mut history: HashMap<&str, bool> = HashMap::new();

        designs
            .into_iter()
            .filter(|design| can_build(design, &patterns, &mut history))
            .count()
            .to_string()
    }
}

fn can_build<'a>(design: &'a str, patterns: &[&str], history: &mut HashMap<&'a str, bool>) -> bool {
    if design.is_empty() {
        return true;
    }

    if let Some(res) = history.get(design) {
        *res
    } else {
        let res = patterns.iter().any(|pattern| {
            design
                .strip_prefix(pattern)
                .is_some_and(|rest| can_build(rest, patterns, history))
        });
        history.insert(design, res);
        res
    }
}
