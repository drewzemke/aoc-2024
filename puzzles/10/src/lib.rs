use common::{dir::Dir, grid::Grid, grid_def, point::Point};
use std::collections::HashSet;

pub mod puzzle10a;
pub mod puzzle10b;

grid_def!(TrailGrid, u64);

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
            if self.at(neighbor).is_some_and(|v| *v == val + 1) {
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
            if self.at(neighbor).is_some_and(|v| *v == val + 1) {
                sum += self.distinct_trails(neighbor, val + 1);
            }
        }

        sum
    }
}
