use common::grid::Grid;
use gcd::Gcd;
use std::collections::{HashMap, HashSet};

pub mod puzzle08a;
pub mod puzzle08b;

type Point = (i64, i64);

#[derive(Debug, Clone, Copy)]
enum Tile {
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

struct AntennaGrid(Grid<Tile>);

impl std::ops::Deref for AntennaGrid {
    type Target = Grid<Tile>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AntennaGrid {
    pub fn parse(input: &str) -> Self {
        Self(Grid::<Tile>::parse(input))
    }

    pub fn antennae(&self) -> HashMap<char, Vec<Point>> {
        let mut out = HashMap::<char, Vec<Point>>::new();

        self.rows().enumerate().for_each(|(row_idx, row)| {
            row.enumerate().for_each(|(col_idx, tile)| {
                if let Tile::Antenna(c) = tile {
                    let pt = (row_idx as i64, col_idx as i64);
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
            // iterate through all the pairs of antennae
            for (idx, u) in ants.iter().enumerate() {
                for v in &ants[idx + 1..] {
                    let [a, b] = Self::simple_antinodes(u, v);
                    out.insert(a);
                    out.insert(b);
                }
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
        [
            (2 * u.0 - v.0, 2 * u.1 - v.1),
            (2 * v.0 - u.0, 2 * v.1 - u.1),
        ]
    }

    pub fn all_general_antinodes(&self) -> HashSet<Point> {
        let antennae = self.antennae();
        let mut out = HashSet::<Point>::new();

        for (_, ants) in antennae {
            for (idx, u) in ants.iter().enumerate() {
                for v in &ants[idx + 1..] {
                    for node in self.general_antinodes(u, v) {
                        out.insert(node);
                    }
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
        let diff = (u.0 - v.0, u.1 - v.1);
        let gcd = diff.0.unsigned_abs().gcd(diff.1.unsigned_abs()) as i64;
        let step = (diff.0 / gcd, diff.1 / gcd);

        let mut pt = *u;

        // march until we're outside the grid
        while self.contains(pt) {
            out.push(pt);
            pt = (pt.0 + step.0, pt.1 + step.1);
        }

        // do it in the other direction, but don't count u again
        pt = (u.0 - step.0, u.1 - step.1);

        while self.contains(pt) {
            out.push(pt);
            pt = (pt.0 - step.0, pt.1 - step.1);
        }

        out
    }
}
