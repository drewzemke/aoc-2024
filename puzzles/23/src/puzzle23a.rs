use common::puzzle::PuzzlePart;

use crate::Network;

pub struct Puzzle23a {}

impl PuzzlePart for Puzzle23a {
    fn description() -> &'static str {
        "Count the number of sets of three interconnected nodes in a computer network where at least one node starts with 't'."
    }

    fn solve(input: &str) -> String {
        let graph = Network::parse(input).unwrap();

        let mut subgraphs = graph.complete_subgraphs();

        subgraphs
            .nth(3) // size 3 subgraphs
            .unwrap()
            .iter()
            .filter(|nodes| nodes.iter().any(|node| node.starts_with('t')))
            .count()
            .to_string()
    }
}
