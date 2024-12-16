use common::{grid::Grid, grid_def, point::Point};
use gcd::Gcd;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub mod puzzle08a;
pub mod puzzle08b;

#[derive(Debug, Clone, Copy)]
pub enum Tile {
    Nothing,
    Antenna(char),
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        if c == '.' {
            Self::Nothing
        } else {
            Self::Antenna(c)
        }
    }
}

grid_def!(AntennaGrid, Tile);

impl AntennaGrid {
    pub fn parse(input: &str) -> Self {
        Self(Grid::<Tile>::parse(input))
    }

    pub fn antennae(&self) -> HashMap<char, Vec<Point>> {
        let mut out = HashMap::<char, Vec<Point>>::new();

        self.rows().enumerate().for_each(|(row_idx, row)| {
            row.enumerate().for_each(|(col_idx, tile)| {
                if let Tile::Antenna(c) = tile {
                    let pt = (row_idx as i64, col_idx as i64).into();
                    match out.get_mut(&c) {
                        Some(vec) => vec.push(pt),
                        None => {
                            out.insert(c, vec![pt]);
                        }
                    };
                }
            })
        });

        out
    }

    pub fn all_simple_antinodes(&self) -> HashSet<Point> {
        let antennae = self.antennae();
        let mut out = HashSet::<Point>::new();

        for (_, ants) in antennae {
            for pair in ants.iter().combinations(2) {
                let [a, b] = Self::simple_antinodes(pair[0], pair[1]);
                out.insert(a);
                out.insert(b);
            }
        }

        out
    }

    /// the "simple antinodes" of two points are the two points that are twice
    /// as far away from one of the two points as they are from the other
    ///
    /// if u and v are points, then their simple antinodes are
    ///   u - 2 (u - v) = 2v - u
    /// and
    ///   v - 2 (v - u) = 2u - v
    pub fn simple_antinodes(u: &Point, v: &Point) -> [Point; 2] {
        [v + v - u, u + u - v]
    }

    pub fn all_general_antinodes(&self) -> HashSet<Point> {
        let antennae = self.antennae();
        let mut out = HashSet::<Point>::new();

        for (_, ants) in antennae {
            for pair in ants.iter().combinations(2) {
                for node in self.general_antinodes(pair[0], pair[1]) {
                    out.insert(node);
                }
            }
        }

        out
    }

    /// the "general" antinodes of two points are any grid point that
    /// lies on the line spanned by the two points
    pub fn general_antinodes(&self, u: &Point, v: &Point) -> Vec<Point> {
        let mut out = vec![];
        // we can find all grid points by computing the gcd of the x- and y-coords
        // of u-v, then marching by multiples of (u-v) / g starting from u in the directions
        // v and -v
        let diff = u - v;
        let gcd = diff.row.unsigned_abs().gcd(diff.col.unsigned_abs()) as i64;
        let step = Point::from((diff.row / gcd, diff.col / gcd));

        let mut pt = *u;

        // march until we're outside the grid
        while self.contains(pt) {
            out.push(pt);
            pt = pt + step;
        }

        // do it in the other direction, but don't count u again
        pt = u - step;

        while self.contains(pt) {
            out.push(pt);
            pt = pt - step;
        }

        out
    }
}
