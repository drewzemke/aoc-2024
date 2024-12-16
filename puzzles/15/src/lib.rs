use common::{dir::Dir, grid::Grid, grid_def, point::Point};

pub mod puzzle15a;
pub mod puzzle15b;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Wall,
    Robot,
    SmallBox,
    BigBoxLeft,
    BigBoxRight,
    Nothing,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '#' => Self::Wall,
            '@' => Self::Robot,
            'O' => Self::SmallBox,
            _ => Self::Nothing,
        }
    }
}

impl From<Tile> for char {
    fn from(t: Tile) -> Self {
        match t {
            Tile::Wall => '#',
            Tile::Robot => '@',
            Tile::SmallBox => 'O',
            Tile::BigBoxLeft => '[',
            Tile::BigBoxRight => ']',
            Tile::Nothing => '.',
        }
    }
}

impl Tile {
    pub fn is_box(&self) -> bool {
        *self == Self::SmallBox || *self == Self::BigBoxLeft || *self == Self::BigBoxRight
    }

    pub fn is_space(&self) -> bool {
        *self == Self::Nothing || *self == Self::Robot
    }
}

grid_def!(WarehouseGrid, Tile);

impl WarehouseGrid {
    pub fn parse(input: &str) -> Self {
        Self(Grid::parse(input))
    }

    pub fn move_robot(&mut self, instructions: Instructions) {
        let mut robot_pos = self.find_pt(|t| t == Tile::Robot).unwrap();

        // println!("start at {robot_pos:?}");
        // println!("{}", self.0);

        for dir in &instructions.0 {
            // println!("\nMoving {dir:?}");

            // check if theres's a box in front of the robot
            let next_pt = robot_pos + dir.step();
            let advance = self.push_box(next_pt, *dir, true);

            // move forward if possible
            if advance {
                robot_pos = next_pt;
            }
            // println!("now at {robot_pos:?}\n{}", self.0);
        }
    }

    /// pushes the box at `pt` in the direction `dir`, if possible,
    /// including all subsequent boxes that would be pushed by that box.
    /// returns whether or not the box was able to moved
    pub fn push_box(&mut self, pt: Point, dir: Dir, commit: bool) -> bool {
        // if there's no box, there's nothing to push
        if self.at(pt).is_some_and(Tile::is_space) {
            return true;
        }
        if self.at(pt) == Some(&Tile::Wall) {
            return false;
        }

        if self.at(pt) == Some(&Tile::SmallBox) {
            // for the small box, we just look ahead from this box
            // to find the first space that isn't a box
            let mut probe_pt = pt;

            let ahead_tile = loop {
                probe_pt = probe_pt + dir.step();
                if let Some(t) = self.at(probe_pt) {
                    if !t.is_box() {
                        break *t;
                    }
                }
            };

            if ahead_tile.is_space() {
                self.put(Tile::Nothing, pt);
                self.put(Tile::SmallBox, probe_pt);
                true
            } else {
                false
            }
        } else {
            // this must be the left or right half of a large box.
            // check if it *and its other half* can move forward,
            // either by moving into the space in front of it
            // or by pushing other blocks
            let (left_pt, right_pt): (Point, Point) = match self.at(pt) {
                Some(Tile::BigBoxLeft) => (pt, (pt.row, pt.col + 1).into()),
                Some(Tile::BigBoxRight) => ((pt.row, pt.col - 1).into(), pt),
                _ => unreachable!(),
            };

            // if we're moving north or south, check that the spaces above/below both blocks
            // are either empty or are moveable blocks (and move them if so)
            if dir == Dir::North || dir == Dir::South {
                let next_left = left_pt + dir.step();
                let next_right = right_pt + dir.step();

                if self.push_box(next_left, dir, false) && self.push_box(next_right, dir, false) {
                    if commit {
                        self.push_box(next_left, dir, true);
                        self.push_box(next_right, dir, true);
                        self.put(Tile::Nothing, left_pt);
                        self.put(Tile::Nothing, right_pt);
                        self.put(Tile::BigBoxLeft, next_left);
                        self.put(Tile::BigBoxRight, next_right);
                    }
                    true
                } else {
                    false
                }

            // if we're moving west or east, we only need to check the block to the
            // left of the left block or to the right of the right block
            } else {
                let next_pt = if dir == Dir::West {
                    left_pt + dir.step()
                } else {
                    right_pt + dir.step()
                };

                if self.push_box(next_pt, dir, false) {
                    if commit {
                        self.push_box(next_pt, dir, true);
                        if dir == Dir::West {
                            self.put(Tile::BigBoxLeft, next_pt);
                            self.put(Tile::BigBoxRight, left_pt);
                            self.put(Tile::Nothing, right_pt);
                        } else {
                            self.put(Tile::BigBoxRight, next_pt);
                            self.put(Tile::BigBoxLeft, right_pt);
                            self.put(Tile::Nothing, left_pt);
                        }
                    }
                    true
                } else {
                    false
                }
            }
        }
    }

    pub fn embiggen(&mut self) {
        let big_tiles = self
            .rows()
            .map(|row| {
                row.flat_map(|tile| match tile {
                    Tile::Wall => [Tile::Wall; 2],
                    Tile::Robot => [Tile::Robot, Tile::Nothing],
                    Tile::SmallBox => [Tile::BigBoxLeft, Tile::BigBoxRight],
                    Tile::Nothing => [Tile::Nothing; 2],
                    _ => unreachable!(),
                })
                .collect()
            })
            .collect();

        self.0 .0 = big_tiles;
    }

    /// the "gps coordinate" of a box is 100 times its distance from the top
    /// edge of the map plus its distance from the left edge of the map
    pub fn box_gps_coord_sum(&self) -> usize {
        let mut sum = 0;

        for (row_idx, row) in self.rows().enumerate() {
            for (col_idx, tile) in row.enumerate() {
                if tile == Tile::SmallBox || tile == Tile::BigBoxLeft {
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
