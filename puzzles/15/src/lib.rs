use common::{dir::Dir, grid::Grid, point::Point};

pub mod puzzle15a;
pub mod puzzle15b;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Wall,
    Robot,
    Box,
    Nothing,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '#' => Self::Wall,
            '@' => Self::Robot,
            'O' => Self::Box,
            _ => Self::Nothing,
        }
    }
}

#[derive(Debug)]
pub struct WarehouseGrid(Grid<Tile>);

impl std::ops::Deref for WarehouseGrid {
    type Target = Grid<Tile>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl WarehouseGrid {
    pub fn parse(input: &str) -> Self {
        Self(Grid::parse(input))
    }

    pub fn move_robot(&mut self, instructions: Instructions) {
        // TODO: extract a "find" function for grid
        let mut robot_pos: Point = self
            .rows()
            .enumerate()
            .find_map(|(row_idx, row)| {
                row.enumerate().find_map(|(col_idx, tile)| {
                    if tile == Tile::Robot {
                        Some((row_idx as i64, col_idx as i64).into())
                    } else {
                        None
                    }
                })
            })
            .unwrap();

        for instruction in &instructions.0 {
            // look ahead from the guard's position until we find either a space or a wall
            let mut probe_pt = robot_pos;
            let ahead_tile = loop {
                probe_pt = probe_pt + instruction.step();
                if let Some(t) = self.at(probe_pt) {
                    if *t != Tile::Box {
                        break *t;
                    }
                }
            };

            // if it's a wall, do nothing, just move on to the next instruction
            // if it's an empty space, and it's not the space directly in front
            // of the robot, "push" all the blocks between here and there forward.
            // btw, that's the same as:
            //  - remove the box directly in front of the robot
            //  - replace the blank space found with a box
            //  - move the robot forward one step
            let next_pt = robot_pos + instruction.step();
            if ahead_tile != Tile::Wall {
                if next_pt != probe_pt {
                    self.put(Tile::Nothing, next_pt);
                    self.put(Tile::Box, probe_pt);
                }
                robot_pos = next_pt;
            }
        }
    }

    // TODO: extract to `Grid`
    fn put(&mut self, that: Tile, here: Point) {
        self.0 .0[here.row as usize][here.col as usize] = that;
    }

    /// the "gps coordinate" of a box is 100 times its distance from the top
    /// edge of the map plus its distance from the left edge of the map
    pub fn box_gps_coord_sum(&self) -> usize {
        let mut sum = 0;

        for (row_idx, row) in self.rows().enumerate() {
            for (col_idx, tile) in row.enumerate() {
                if tile == Tile::Box {
                    sum += 100 * row_idx + col_idx;
                }
            }
        }

        sum
    }
}

#[derive(Debug)]
pub struct Instructions(Vec<Dir>);

impl Instructions {
    pub fn parse(input: &str) -> Self {
        let dirs = input
            .chars()
            .filter_map(|c| match c {
                '^' => Some(Dir::North),
                'v' => Some(Dir::South),
                '>' => Some(Dir::East),
                '<' => Some(Dir::West),
                _ => None,
            })
            .collect();

        Self(dirs)
    }
}
