use crate::point::Point;

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
