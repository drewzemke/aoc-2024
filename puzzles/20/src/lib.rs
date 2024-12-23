use common::{dir::Dir, grid::Grid, grid_def, point::Point};

pub mod puzzle20a;
pub mod puzzle20b;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tile {
    Nothing,
    Wall,
    Start,
    End,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Tile::Nothing,
            '#' => Tile::Wall,
            'S' => Tile::Start,
            'E' => Tile::End,
            _ => unreachable!(),
        }
    }
}

impl Tile {
    pub fn is_space(&self) -> bool {
        *self != Tile::Wall
    }
}

grid_def!(Racetrack, Tile);

impl Racetrack {
    pub fn parse(input: &str) -> Self {
        Racetrack(Grid::parse(input))
    }

    /// A "small cheat" is two points on the track (ie. not walls)
    /// that are exactly distance 2 from each other in the grid,
    /// with a piece of wall between them
    pub fn small_cheats(&self, path: &[Point]) -> Vec<(Point, Point)> {
        // strat: check every point along the path. from each point, look in every possible
        // direction that there could be to find a cheat from here; if there
        // is a wall on either of two spaces between the two cheats, add it to the set
        let mut out = Vec::<(Point, Point)>::new();

        for pt in path {
            for dir in Dir::all() {
                let step1 = pt + dir.step();
                if self.at(step1).is_some_and(Tile::is_space) {
                    continue;
                }

                let step2 = step1 + dir.step();
                if !self.at(step2).is_some_and(Tile::is_space) {
                    continue;
                }

                out.push((*pt, step2));
            }
        }

        out
    }
}
