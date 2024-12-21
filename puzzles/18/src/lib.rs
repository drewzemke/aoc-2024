use std::collections::{BinaryHeap, HashMap, HashSet};

use common::{dir::Dir, grid::Grid, grid_def, point::Point};

pub mod puzzle18a;
pub mod puzzle18b;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tile {
    Byte,
    Nothing,
    Visited,
}

impl From<Tile> for char {
    fn from(value: Tile) -> Self {
        match value {
            Tile::Byte => '#',
            Tile::Nothing => '.',
            Tile::Visited => 'O',
        }
    }
}

grid_def!(ByteGrid, Tile);

pub fn make_coords(line: &str) -> Option<(i64, i64)> {
    let (rows_str, cols_str) = line.split_once(',')?;
    Some((rows_str.parse::<i64>().ok()?, cols_str.parse::<i64>().ok()?))
}

impl ByteGrid {
    pub fn parse(input: &str, max_bytes: usize) -> Option<Self> {
        let mut lines = input.lines();

        // the first line has the size
        let (num_rows, num_cols) = lines.next().and_then(make_coords)?;

        // the rest are coordinates
        let coords = lines
            .take(max_bytes)
            .map(make_coords)
            .map(Option::unwrap)
            .collect::<Vec<_>>();

        // make a grid
        let grid = (0..num_rows)
            .map(|row_idx| {
                (0..num_cols)
                    .map(|col_idx| {
                        if coords.contains(&(row_idx, col_idx)) {
                            Tile::Byte
                        } else {
                            Tile::Nothing
                        }
                    })
                    .collect()
            })
            .collect();

        Some(Self(Grid(grid)))
    }

    pub fn add_byte(&mut self, pt: Point) {
        self.put(Tile::Byte, pt);
    }

    // TODO: try A*?
    pub fn least_steps(&self, start: Point, end: Point) -> Option<usize> {
        let mut least_step_map = HashMap::<Point, usize>::new();

        self.explore(start, 0, &mut least_step_map);

        least_step_map.get(&end).cloned()
    }

    // dumb algorithm that just checks in every possible direction, updating the map if we
    // find our way to a point we've already visited with a better score
    fn explore(&self, pt: Point, steps: usize, data: &mut HashMap<Point, usize>) {
        data.insert(pt, steps);

        for dir in Dir::all() {
            let neighbor = pt + dir.step();

            if self.at(neighbor).is_some_and(|t| *t != Tile::Byte)
                && (data.get(&neighbor).is_none()
                    || data.get(&neighbor).is_some_and(|v| steps + 1 < *v))
            {
                self.explore(neighbor, steps + 1, data);
            }
        }
    }

    pub fn a_star(&self, start: Point, end: Point) -> Option<Vec<Point>> {
        let mut open_set = BinaryHeap::new();
        let mut closed_set = HashSet::new();
        let mut predecessors = HashMap::new();
        let mut g_scores = HashMap::new();

        let heur = |pt: &Point| ((end.row - pt.row).abs() + (end.col - pt.col).abs()) as u64;

        g_scores.insert(start, 0);
        open_set.push(AStarNode {
            f_score: heur(&start),
            point: start,
        });

        while let Some(AStarNode { point: pt, .. }) = open_set.pop() {
            if pt == end {
                return Some(Self::reconstruct_path(end, &predecessors));
            }

            if !closed_set.insert(pt) {
                continue;
            }

            let current_g = g_scores[&pt];

            for dir in Dir::all() {
                let neighbor = pt + dir.step();

                if !self.at(neighbor).is_some_and(|t| *t != Tile::Byte) {
                    continue;
                }

                let tentative_g = current_g + 1;

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

    fn reconstruct_path(end: Point, predecessors: &HashMap<Point, Point>) -> Vec<Point> {
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

// Key optimizations:
// 1. BinaryHeap instead of HashSet for open set (O(log n) vs O(n) for finding minimum)
// 2. Closed set to avoid revisiting nodes
// 3. Pre-calculated f-scores stored in the heap nodes
// 4. Early continue for invalid neighbors
// 5. Path is now built in reverse order and reversed once at the end
// 6. Removed redundant f_scores HashMap
// 7. Cached current_g score to avoid HashMap lookups
