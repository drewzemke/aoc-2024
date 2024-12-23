use crate::Network;
use common::puzzle::PuzzlePart;

pub struct Puzzle23b {}

impl PuzzlePart for Puzzle23b {
    fn description() -> &'static str {
        "Find the 'password' obtained by sorting the nodes in the *largest* connect subgraph of a network."
    }

    fn solve(input: &str) -> String {
        let graph = Network::parse(input).unwrap();

        let subgraphs = graph.complete_subgraphs();
        let mut largest_subgraph = subgraphs
            .last()
            .and_then(|g| g.into_iter().next())
            .unwrap()
            .into_iter()
            .collect::<Vec<_>>();

        largest_subgraph.sort_unstable();

        largest_subgraph.join(",")
    }
}
