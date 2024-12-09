use common::grid::Grid;
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

    pub fn all_antinodes(&self) -> HashSet<Point> {
        let antennae = self.antennae();
        let mut out = HashSet::<Point>::new();

        for (_, ants) in antennae.iter() {
            // iterate through all the pairs of antennae
            for (idx, u) in ants.iter().enumerate() {
                for v in &ants[idx + 1..] {
                    let [a, b] = Self::antinodes(u, v);
                    out.insert(a);
                    out.insert(b);
                }
            }
        }

        out
    }

    /// the antinodes of two points are the two points that are twice
    /// as far away from one of the two points as they are from the other
    ///
    /// if u and v are points, then their antinodes are
    ///   u - 2 (u - v) = 2v - u
    /// and
    ///   v - 2 (v - u) = 2u - v
    pub fn antinodes(u: &Point, v: &Point) -> [Point; 2] {
        [
            (2 * u.0 - v.0, 2 * u.1 - v.1),
            (2 * v.0 - u.0, 2 * v.1 - u.1),
        ]
    }
}
