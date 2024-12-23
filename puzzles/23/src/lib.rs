use std::collections::{BTreeSet, HashMap, HashSet};

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

    /// Iterates over all of the complete subgraphs in this network
    ///
    /// NOTE: the zeroth iteration is all complete subgraphs of size 0, which is an empty list.
    /// The first iteration is all of the complete subgraphs of size 1, ie. the vertices.
    /// and the second is all complete subgraphs of size 2, ie. the edges.
    /// Things get interesting at the third iteration.
    pub fn complete_subgraphs(&self) -> impl Iterator<Item = HashSet<BTreeSet<&str>>> + Clone {
        SubgraphIterator::new(self)
    }
}

#[derive(Debug, Clone)]
struct SubgraphIterator<'a> {
    network: &'a Network<'a>,
    prev: HashSet<BTreeSet<&'a str>>,
    is_iter_zero: bool,
}

impl<'a> SubgraphIterator<'a> {
    fn new(network: &'a Network<'a>) -> Self {
        Self {
            network,
            prev: HashSet::new(),
            is_iter_zero: true,
        }
    }
}

impl<'a> Iterator for SubgraphIterator<'a> {
    type Item = HashSet<BTreeSet<&'a str>>;

    fn next(&mut self) -> Option<Self::Item> {
        // return just an empty list if this is the zeroth iteration
        if self.is_iter_zero {
            self.is_iter_zero = false;
            return Some(HashSet::new());
        }

        // compute the next set of complete graphs
        let mut next_gen = HashSet::new();

        if self.prev.is_empty() {
            // if this is the first iteration, just return the list of verts
            next_gen = self
                .network
                .0
                .keys()
                .map(|v| {
                    let mut h = BTreeSet::new();
                    h.insert(*v);
                    h
                })
                .collect();
        } else {
            // otherwise otherwise, compute the next set of complete graphs from
            // the previous one by, for each graph in the previous generation,
            // check each vertex to see if it's adject to every element of the graph
            for prev_graph in &self.prev {
                for vert in self.network.0.keys() {
                    if prev_graph.contains(vert) {
                        continue;
                    }

                    let vert_neighbors = &self.network.0[vert];
                    if prev_graph.iter().all(|v| vert_neighbors.contains(v)) {
                        // create a new subgraph
                        let mut new_graph = prev_graph.clone();
                        new_graph.insert(vert);
                        next_gen.insert(new_graph);
                    }
                }
            }
        }

        // no new subgraphs? sounds like we're done
        if next_gen.is_empty() {
            None
        } else {
            self.prev = next_gen.clone();
            Some(next_gen)
        }
    }
}
