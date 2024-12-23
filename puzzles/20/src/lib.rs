use common::{grid::Grid, grid_def, point::Point};
use std::collections::{HashMap, HashSet};

pub mod puzzle20a;
pub mod puzzle20b;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tile {
    Nothing,
    Wall,
    Start,
    End,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Tile::Nothing,
            '#' => Tile::Wall,
            'S' => Tile::Start,
            'E' => Tile::End,
            _ => unreachable!(),
        }
    }
}

impl Tile {
    pub fn is_space(&self) -> bool {
        *self != Tile::Wall
    }
}

grid_def!(Racetrack, Tile);

pub type Cheat = (Point, Point, i64);

impl Racetrack {
    pub fn parse(input: &str) -> Self {
        Racetrack(Grid::parse(input))
    }

    /// A "cheat" is two points on the track (ie. not walls)
    /// that are at most manhattan-distance D from each other in the grid,
    pub fn cheats(&self, path: &[Point], max_dist: i64) -> Vec<Cheat> {
        let mut out = HashSet::<Cheat>::new();

        for pt in path {
            // a walk for manhattan-distance-at-most-20 in the grid consists
            // of walking horizontally for N steps and then vertically
            // for M-N steps, where 0 <= N <= M and 0 <= M <= max_dist
            for h_dir in [1, -1] {
                for v_dir in [1, -1] {
                    for m in 2..=max_dist {
                        for n in 0..=m {
                            let new_pt = pt + Point::from((v_dir * n, h_dir * (m - n)));
                            if self.at(new_pt).is_some_and(Tile::is_space) {
                                out.insert((*pt, new_pt, m));
                            }
                        }
                    }
                }
            }
        }

        out.into_iter().collect()
    }

    pub fn savings(&self, path: &[Point], cheats: Vec<Cheat>) -> Vec<usize> {
        let path_indices: HashMap<Point, usize> =
            path.iter().enumerate().map(|(i, p)| (*p, i)).collect();

        cheats
            .iter()
            .filter_map(|(p1, p2, skipped)| {
                let i1 = path_indices[p1];
                let i2 = path_indices[p2];

                if i1 > i2 {
                    None
                } else {
                    Some(i2 - i1 - *skipped as usize)
                }
            })
            .collect()
    }
}
