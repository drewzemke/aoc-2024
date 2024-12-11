use std::collections::HashSet;

use common::{grid::Grid, point::Point};

pub mod puzzle10a;
pub mod puzzle10b;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Dir {
    North,
    South,
    East,
    West,
}

impl Dir {
    pub fn all() -> Vec<Self> {
        vec![Self::North, Self::South, Self::East, Self::West]
    }

    pub fn step(&self) -> Point {
        match self {
            Dir::North => (-1, 0),
            Dir::South => (1, 0),
            Dir::East => (0, 1),
            Dir::West => (0, -1),
        }
        .into()
    }
}

#[derive(Debug)]
pub struct TrailGrid(Grid<u64>);

impl std::ops::Deref for TrailGrid {
    type Target = Grid<u64>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TrailGrid {
    pub fn parse(input: &str) -> Self {
        let grid = Grid::parse_with(input, |x| x.to_digit(10).unwrap() as u64);
        Self(grid)
    }

    pub fn trailhead_count(&self, distinct: bool) -> u64 {
        let mut sum = 0;

        for (row_idx, row) in self.rows().enumerate() {
            for (col_idx, val) in row.enumerate() {
                if val == 0 {
                    let start = (row_idx as i64, col_idx as i64).into();

                    if distinct {
                        sum += self.distinct_trails(start, 0);
                    } else {
                        sum += self.trail_endpts(start, 0).len() as u64;
                    }
                }
            }
        }

        sum
    }

    fn trail_endpts(&self, start: Point, val: u64) -> HashSet<Point> {
        // if this val is 9, we win!
        if val == 9 {
            // println!("nice!");
            return HashSet::from([start]);
        }

        // otherwise, check all neighbors to see if they have the next value
        // needed to proceed
        let mut found = HashSet::new();

        for dir in Dir::all() {
            let neighbor = start + dir.step();
            if self.contains(neighbor) && *self.at(neighbor) == val + 1 {
                found.extend(self.trail_endpts(neighbor, val + 1));
            }
        }

        found
    }

    fn distinct_trails(&self, start: Point, val: u64) -> u64 {
        // if this val is 9, there's only one way to get here
        if val == 9 {
            return 1;
        }

        let mut sum = 0;

        for dir in Dir::all() {
            let neighbor = start + dir.step();
            if self.contains(neighbor) && *self.at(neighbor) == val + 1 {
                sum += self.distinct_trails(neighbor, val + 1);
            }
        }

        sum
    }
}
