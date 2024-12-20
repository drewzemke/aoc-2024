use std::collections::HashMap;

use common::puzzle::PuzzlePart;

pub struct Puzzle19b {}

impl PuzzlePart for Puzzle19b {
    fn description() -> &'static str {
        "Count the number of ways all of the stripe designs can be constructed from stripe patterns."
    }

    fn solve(input: &str) -> String {
        let patterns = input
            .lines()
            .next()
            .unwrap()
            .split(", ")
            .collect::<Vec<_>>();

        let designs = input.lines().skip(2).collect::<Vec<_>>();
        let mut history: HashMap<&str, u64> = HashMap::new();

        designs
            .into_iter()
            .map(|design| how_many_ways(design, &patterns, &mut history))
            .sum::<u64>()
            .to_string()
    }
}

fn how_many_ways<'a>(
    design: &'a str,
    patterns: &[&str],
    history: &mut HashMap<&'a str, u64>,
) -> u64 {
    if design.is_empty() {
        return 1;
    }

    if let Some(res) = history.get(design) {
        *res
    } else {
        // map each pattern to the number of ways the design can be constructed
        // with that pattern as the first thing
        let res = patterns
            .iter()
            .map(|pattern| {
                if let Some(rest) = design.strip_prefix(pattern) {
                    how_many_ways(rest, patterns, history)
                } else {
                    0
                }
            })
            .sum::<u64>();
        history.insert(design, res);
        res
    }
}
