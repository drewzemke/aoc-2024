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

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
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

#[derive(Clone)]
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

    fn walk(&self, start: (i64, i64), dir: Dir) -> impl Iterator<Item = ((i64, i64), Dir)> + '_ {
        GuardGridWalker::new(self, start, dir)
    }

    /// walk the grid from the starting point/direction, turning right
    /// at every obstacle, until we exit the grid. return all of the
    /// points we walked over
    pub fn walked_points(&self, start: (i64, i64), dir: Dir) -> HashSet<(i64, i64)> {
        self.walk(start, dir).map(|(pt, _)| pt).collect()
    }

    /// determines if a walked path through the grid will loop or not
    fn will_it_loop(&self, start: (i64, i64), dir: Dir) -> bool {
        let mut visited: HashSet<((i64, i64), Dir)> = HashSet::new();

        for (pt, dir) in self.walk(start, dir) {
            if visited.contains(&(pt, dir)) {
                return true;
            }

            visited.insert((pt, dir));
        }

        false
    }

    fn put(&mut self, that: Tile, here: (i64, i64)) {
        self.0 .0[here.0 as usize][here.1 as usize] = that;
    }

    /// computes all obstacles that, if added (individually) to the grid,
    /// would cause the guard to get stuck in a loop
    pub fn loop_causing_obstacles(&self, start: (i64, i64), dir: Dir) -> usize {
        let mut obstacles: HashSet<(i64, i64)> = HashSet::new();

        // used to test for looping after placing an obstacle
        let mut base_grid = self.clone();

        for (pt, _) in self.walk(start, dir) {
            if !obstacles.contains(&pt) && pt != start {
                // put an obstacle at next point
                base_grid.put(Tile::Obstacle, pt);

                // check if it loops with that new obstacle
                if base_grid.will_it_loop(start, dir) {
                    obstacles.insert(pt);
                }

                // restore the original
                base_grid.put(Tile::Nothing, pt);
            }
        }

        obstacles.len()
    }
}

struct GuardGridWalker<'a> {
    grid: &'a GuardGrid,
    pt: (i64, i64),
    dir: Dir,
}

impl<'a> GuardGridWalker<'a> {
    fn new(grid: &'a GuardGrid, start: (i64, i64), dir: Dir) -> Self {
        Self {
            grid,
            pt: start,
            dir,
        }
    }
}

impl<'a> Iterator for GuardGridWalker<'a> {
    type Item = ((i64, i64), Dir);

    fn next(&mut self) -> Option<Self::Item> {
        // determine the to-be-yielded point before updating
        let out = if self.grid.contains(self.pt) {
            Some((self.pt, self.dir))
        } else {
            None
        };

        let step = self.dir.step();
        let mut next_pt = (self.pt.0 + step.0, self.pt.1 + step.1);

        // turn right until we're not facing an obstacle
        while self.grid.contains(next_pt) && *self.grid.at_unchecked(next_pt) == Tile::Obstacle {
            self.dir.turn_right();
            let step = self.dir.step();
            next_pt = (self.pt.0 + step.0, self.pt.1 + step.1);
        }

        // advance
        self.pt = next_pt;

        out
    }
}
