use std::collections::{HashMap, HashSet};

pub mod puzzle23a;
pub mod puzzle23b;

/// A graph stored as a map node -> neighbors
#[derive(Debug)]
pub struct Network<'a>(HashMap<&'a str, Vec<&'a str>>);

impl<'a> Network<'a> {
    pub fn parse(input: &'a str) -> Option<Self> {
        // each line is an edge; parse into pairs
        let edges = input
            .lines()
            .map(|line| line.split_once('-'))
            .collect::<Option<Vec<_>>>()?;

        // create a list of nodes, then iterate through it to build the adjacently map
        let nodes: HashSet<&str> = edges
            .iter()
            .flat_map(|(left, right)| [*left, *right])
            .collect();

        let node_map = nodes
            .iter()
            .map(|node| {
                // find all the nodes with edges connecting to this one
                let adjacent_indices = edges
                    .iter()
                    .filter_map(|(left, right)| {
                        if node == left {
                            Some(*right)
                        } else if node == right {
                            Some(*left)
                        } else {
                            None
                        }
                    })
                    .collect();

                (*node, adjacent_indices)
            })
            .collect();

        Some(Self(node_map))
    }

    /// finds all of the size-3 connected subgraphs of this network
    ///
    /// NOTE: this actually compute each k3 6 times, since it finds
    /// all possible permutations. I'll fix this in part b
    pub fn k3_subgraphs(&self) -> HashSet<Vec<&str>> {
        let mut subgraphs = HashSet::<Vec<&str>>::new();

        // look for loops of size 3 starting from each vertex
        for first in self.0.keys() {
            for second in &self.0[first] {
                for third in &self.0[second] {
                    if third != first && self.0[third].contains(first) {
                        subgraphs.insert(vec![first, second, third]);
                    }
                }
            }
        }

        subgraphs
    }
}
