use std::collections::HashSet;

use common::grid::Grid;

pub mod puzzle06a;
pub mod puzzle06b;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Tile {
    Nothing,
    Guard,
    Obstacle,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        // possibilities are '.' '^' and '#'
        match c {
            '#' => Tile::Obstacle,
            '^' => Tile::Guard,
            _ => Tile::Nothing,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Dir {
    North,
    South,
    East,
    West,
}

impl Dir {
    pub fn turn_right(&mut self) {
        *self = match self {
            Dir::North => Self::East,
            Dir::South => Self::West,
            Dir::East => Self::South,
            Dir::West => Self::North,
        }
    }

    pub fn step(&self) -> (i64, i64) {
        match self {
            Dir::North => (-1, 0),
            Dir::South => (1, 0),
            Dir::East => (0, 1),
            Dir::West => (0, -1),
        }
    }
}

pub struct GuardGrid(Grid<Tile>);

impl std::ops::Deref for GuardGrid {
    type Target = Grid<Tile>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl GuardGrid {
    pub fn parse(input: &str) -> (Self, (i64, i64), Dir) {
        let grid = Grid::<Tile>::parse(input);

        // find the starting tile
        let (row_idx, col_idx) = grid
            .rows()
            .enumerate()
            .find_map(|(row_idx, row)| {
                row.enumerate().find_map(|(col_idx, tile)| {
                    if tile == Tile::Guard {
                        Some((row_idx, col_idx))
                    } else {
                        None
                    }
                })
            })
            .unwrap();

        (
            GuardGrid(grid),
            (row_idx as i64, col_idx as i64),
            Dir::North,
        )
    }

    /// walk the grid from the starting point/direction, turning right
    /// at every obstacle, until we exit the grid. return all of the
    /// points we walked over
    pub fn walk(&self, start: (i64, i64), dir: Dir) -> HashSet<(i64, i64)> {
        let mut out = HashSet::new();

        let mut pt = start;
        let mut dir = dir;
        while self.contains(pt) {
            out.insert(pt);

            let step = dir.step();
            let next_pt = (pt.0 + step.0, pt.1 + step.1);

            // turn right if we're facing an obstacle, otherwise advance
            if self.contains(next_pt) && *self.at_unchecked(next_pt) == Tile::Obstacle {
                dir.turn_right();
            } else {
                pt = next_pt
            }
        }

        out
    }

    fn will_it_loop(&self, start: (i64, i64), dir: Dir) -> bool {
        let mut out = HashSet::new();

        let mut pt = start;
        let mut dir = dir;
        while self.contains(pt) {
            let new_entry = (pt, dir.clone());
            if out.contains(&new_entry) {
                return true;
            }
            out.insert(new_entry);

            let step = dir.step();
            let next_pt = (pt.0 + step.0, pt.1 + step.1);

            // turn right if we're facing an obstacle
            if self.contains(next_pt) && *self.at_unchecked(next_pt) == Tile::Obstacle {
                dir.turn_right();
            } else {
                pt = next_pt
            }
        }

        false
    }

    fn put_that_here(&mut self, that: Tile, here: (i64, i64)) {
        self.0 .0[here.0 as usize][here.1 as usize] = that;
    }

    pub fn too_many_obstacles(&mut self, start: (i64, i64), dir: Dir) -> usize {
        let mut new_obstacles = HashSet::new();

        let mut pt = start;
        let mut dir = dir;
        while self.contains(pt) {
            let step = dir.step();
            let next_pt = (pt.0 + step.0, pt.1 + step.1);

            if self.contains(next_pt)
                && *self.at_unchecked(next_pt) != Tile::Obstacle
                && !new_obstacles.contains(&next_pt)
                && next_pt != start
            {
                // put an obstacle at next point
                self.put_that_here(Tile::Obstacle, next_pt);

                // check if it loops with that new obstacle
                if self.will_it_loop(pt, dir.clone()) {
                    new_obstacles.insert(next_pt);
                }

                // restore the original
                self.put_that_here(Tile::Nothing, next_pt);
            }

            // turn right if we're facing an obstacle
            if self.contains(next_pt) && *self.at_unchecked(next_pt) == Tile::Obstacle {
                dir.turn_right();
            } else {
                pt = next_pt
            }
        }

        new_obstacles.len()
    }
}
