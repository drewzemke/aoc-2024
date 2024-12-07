use std::collections::HashSet;

pub mod puzzle06a;
pub mod puzzle06b;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
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

pub struct Grid(Vec<Vec<Tile>>);

impl Grid {
    pub fn parse(input: &str) -> (Grid, (i64, i64), Dir) {
        let data = input
            .lines()
            .map(|line| line.chars().map(Tile::from).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        // find the starting tile
        let (row_idx, col_idx) = data
            .iter()
            .enumerate()
            .find_map(|(row_idx, row)| {
                row.iter().enumerate().find_map(|(col_idx, tile)| {
                    if *tile == Tile::Guard {
                        Some((row_idx, col_idx))
                    } else {
                        None
                    }
                })
            })
            .unwrap();

        (Grid(data), (row_idx as i64, col_idx as i64), Dir::North)
    }

    fn at(&self, (row, col): (i64, i64)) -> Tile {
        self.0[row as usize][col as usize]
    }

    fn height(&self) -> usize {
        self.0.len()
    }

    fn width(&self) -> usize {
        self.0[0].len()
    }

    fn is_in_bounds(&self, (row, col): (i64, i64)) -> bool {
        row >= 0 && row < self.height() as i64 && col >= 0 && col < self.width() as i64
    }

    /// walk the grid from the starting point/direction, turning right
    /// at every obstacle, until we exit the grid. return all of the
    /// points we walked over
    pub fn walked_points(&self, start: (i64, i64), dir: Dir) -> HashSet<(i64, i64)> {
        let mut out = HashSet::new();

        let mut pt = start;
        let mut dir = dir;
        while self.is_in_bounds(pt) {
            out.insert(pt);

            let step = dir.step();
            let mut next_pt = (pt.0 + step.0, pt.1 + step.1);

            // turn right if we're facing an obstacle
            if self.is_in_bounds(next_pt) && self.at(next_pt) == Tile::Obstacle {
                dir.turn_right();
                let step = dir.step();
                next_pt = (pt.0 + step.0, pt.1 + step.1);
            }

            pt = next_pt
        }

        out
    }
}
