use super::Grid;
use crate::{dir::Dir, point::Point};
use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(Copy, Clone, Eq, PartialEq)]
struct AStarNode {
    f_score: u64,
    point: Point,
}

impl Ord for AStarNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.f_score.cmp(&self.f_score)
    }
}

impl PartialOrd for AStarNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Grid<T> {
    /// An implementation of the A* shortest path algorithm. Takes
    /// a callback argument allowing callers to specify what tiles
    /// (things of type `T`) count as passable space.
    ///
    /// Returns `None` if no path is found.
    pub fn shortest_path<F>(&self, start: Point, end: Point, is_space: F) -> Option<Vec<Point>>
    where
        F: Fn(&T) -> bool,
    {
        // all of the points that have yet to be considered
        let mut open_set = BinaryHeap::new();

        // all of the points that have been visited already
        let mut closed_set = HashSet::new();

        // maps each visited point to its previous point in the (currently-known)
        // shortest path to that point
        let mut predecessors = HashMap::new();

        // keeps track of distances along (currently-known) shortests paths
        // from the start point
        let mut g_scores = HashMap::new();

        // manhattan distance
        let heur = |pt: &Point| ((end.row - pt.row).abs() + (end.col - pt.col).abs()) as u64;

        g_scores.insert(start, 0);
        open_set.push(AStarNode {
            f_score: heur(&start),
            point: start,
        });

        while let Some(AStarNode { point: pt, .. }) = open_set.pop() {
            if pt == end {
                return Some(Self::build_path(end, &predecessors));
            }

            // if this point has already been visited, there's nothing to do
            if !closed_set.insert(pt) {
                continue;
            }

            let current_g = g_scores[&pt];

            for dir in Dir::all() {
                let neighbor = pt + dir.step();

                if !self.at(neighbor).is_some_and(&is_space) {
                    continue;
                }

                // compute what the g-score of the neighbor would be
                // if we continued the shortest path to it from this point
                let tentative_g = current_g + 1;

                // if this would improve our shortest path to the neighbor,
                // update our data
                if tentative_g < *g_scores.get(&neighbor).unwrap_or(&u64::MAX) {
                    predecessors.insert(neighbor, pt);
                    g_scores.insert(neighbor, tentative_g);

                    if !closed_set.contains(&neighbor) {
                        open_set.push(AStarNode {
                            f_score: tentative_g + heur(&neighbor),
                            point: neighbor,
                        });
                    }
                }
            }
        }

        None
    }

    fn build_path(end: Point, predecessors: &HashMap<Point, Point>) -> Vec<Point> {
        let mut current = end;
        let mut path = vec![];

        loop {
            path.push(current);

            match predecessors.get(&current) {
                Some(&prev) => current = prev,
                None => break,
            }
        }

        path.reverse();
        path
    }
}
