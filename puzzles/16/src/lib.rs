use common::{dir::Dir, grid::Grid, grid_def, point::Point};
use std::collections::HashMap;

pub mod puzzle16a;
pub mod puzzle16b;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Tile {
    Wall,
    Nothing,
    Start,
    End,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '#' => Self::Wall,
            '.' => Self::Nothing,
            'S' => Self::Start,
            'E' => Self::End,
            _ => unreachable!(),
        }
    }
}

grid_def!(Maze, Tile);

impl Maze {
    pub fn parse(input: &str) -> Self {
        Self(Grid::parse(input))
    }

    pub fn shortest_path_score(&self) -> u64 {
        let start_pt = self.find_pt(|t| t == Tile::Start).unwrap();
        let end_pt = self.find_pt(|t| t == Tile::End).unwrap();

        let start_dir = Dir::East;

        let mut scores: HashMap<(Point, Dir), u64> = HashMap::new();
        scores.insert((start_pt, start_dir), 0);

        self.explore(start_pt, start_dir, &mut scores);

        *scores
            .keys()
            .filter(|(pt, _)| *pt == end_pt)
            .map(|e| scores.get(e).unwrap())
            .min()
            .unwrap()
    }

    fn explore(&self, pt: Point, dir: Dir, scores: &mut HashMap<(Point, Dir), u64>) {
        // we can possibly:
        // - move forward
        // - turn left then move forward
        // - turn right then move forward
        // try each of those things and see if the score we would get is an improvement over other
        // paths to those points previously. if we've found an improvement, make a note in the hashmap and
        // recurse on those new points

        let score_here = *scores.get(&(pt, dir)).unwrap_or(&u64::MAX);

        let forward_dir = dir;
        let forward_pt = pt + forward_dir.step();
        if self.at(forward_pt).is_some_and(|t| *t != Tile::Wall) {
            let score_there = *scores.get(&(forward_pt, forward_dir)).unwrap_or(&u64::MAX);
            let new_score = score_here + 1;

            if new_score < score_there {
                scores.insert((forward_pt, forward_dir), new_score);
                self.explore(forward_pt, forward_dir, scores);
            }
        }

        let right_dir = dir.turn_right();
        let right_pt = pt + right_dir.step();
        if self.at(right_pt).is_some_and(|t| *t != Tile::Wall) {
            let score_there = *scores.get(&(right_pt, right_dir)).unwrap_or(&u64::MAX);
            let new_score = score_here + 1001;

            if new_score < score_there {
                scores.insert((right_pt, right_dir), new_score);
                self.explore(right_pt, right_dir, scores);
            }
        }

        let left_dir = dir.turn_left();
        let left_pt = pt + left_dir.step();
        if self.at(left_pt).is_some_and(|t| *t != Tile::Wall) {
            let score_there = *scores.get(&(left_pt, left_dir)).unwrap_or(&u64::MAX);
            let new_score = score_here + 1001;

            if new_score < score_there {
                scores.insert((left_pt, left_dir), new_score);
                self.explore(left_pt, left_dir, scores);
            }
        }
    }
}

trait Turn {
    fn turn_right(&self) -> Self;
    fn turn_left(&self) -> Self;
}

impl Turn for Dir {
    fn turn_right(&self) -> Self {
        match self {
            Dir::North => Self::East,
            Dir::South => Self::West,
            Dir::East => Self::South,
            Dir::West => Self::North,
        }
    }

    fn turn_left(&self) -> Self {
        match self {
            Dir::North => Self::West,
            Dir::South => Self::East,
            Dir::East => Self::North,
            Dir::West => Self::South,
        }
    }
}
