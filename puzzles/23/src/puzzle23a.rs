use common::puzzle::PuzzlePart;

use crate::Network;

pub struct Puzzle23a {}

impl PuzzlePart for Puzzle23a {
    fn description() -> &'static str {
        "Count the number of sets of three interconnected nodes in a computer network where at least one node starts with 't'."
    }

    fn solve(input: &str) -> String {
        let graph = Network::parse(input).unwrap();

        (graph
            .k3_subgraphs()
            .iter()
            .filter(|nodes| nodes.iter().any(|node| node.starts_with('t')))
            .count()
            // divide by 3! = 6 since we over-counted
            // (don't worry I'll fix in part b)
            / 6)
        .to_string()
    }
}
