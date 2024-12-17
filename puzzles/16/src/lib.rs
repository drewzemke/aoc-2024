use common::{dir::Dir, grid::Grid, grid_def, point::Point};
use std::collections::{HashMap, HashSet};

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
        let (score, _) = self.compute_shortest_paths();
        score
    }

    fn compute_shortest_paths(&self) -> (u64, HashMap<(Point, Dir), u64>) {
        let start_pt = self.find_pt(|t| t == Tile::Start).unwrap();
        let end_pt = self.find_pt(|t| t == Tile::End).unwrap();

        let start_dir = Dir::East;

        let mut scores: HashMap<(Point, Dir), u64> = HashMap::new();
        scores.insert((start_pt, start_dir), 0);

        self.explore_forwards(start_pt, start_dir, &mut scores);

        let score = *scores
            .keys()
            .filter(|(pt, _)| *pt == end_pt)
            .map(|e| scores.get(e).unwrap())
            .min()
            .unwrap();

        (score, scores)
    }

    // we can possibly:
    // - move forward
    // - turn left then move forward
    // - turn right then move forward
    // try each of those things and see if the score we would get is an improvement over other
    // paths to those points previously. if we've found an improvement, make a note in the hashmap and
    // recurse on those new points
    fn explore_forwards(&self, pt: Point, dir: Dir, scores: &mut HashMap<(Point, Dir), u64>) {
        let score_here = *scores.get(&(pt, dir)).unwrap_or(&u64::MAX);
        let forward = dir;

        for dir in [dir, dir.turn_right(), dir.turn_left()] {
            let next_pt = pt + dir.step();
            if self.at(next_pt).is_some_and(|t| *t != Tile::Wall) {
                let score_there = *scores.get(&(next_pt, dir)).unwrap_or(&u64::MAX);
                let new_score = if dir == forward {
                    score_here + 1
                } else {
                    score_here + 1001
                };

                if new_score < score_there {
                    scores.insert((next_pt, dir), new_score);
                    self.explore_forwards(next_pt, dir, scores);
                }
            }
        }
    }

    pub fn shortest_path_tiles(&self) -> usize {
        let (score, scores) = self.compute_shortest_paths();
        let end_pt = self.find_pt(|t| t == Tile::End).unwrap();

        let mut shortest_path_tiles: HashSet<Point> = HashSet::new();

        Self::explore_backwards(end_pt, score, &scores, &mut shortest_path_tiles);

        shortest_path_tiles.len()
    }

    // strat:
    //  start at the end tile and work our way backwards, collecting tiles that lie along
    //  shortest paths to the end. since we know the score at the end, we can figure out
    //  that a tile next to the end tile (for instance) must have a score either 1 or 1001
    //  less than the ending score to be on a shortest path
    fn explore_backwards(
        pt: Point,
        score: u64,
        scores: &HashMap<(Point, Dir), u64>,
        tiles: &mut HashSet<Point>,
    ) {
        // add this point to the list of tiles
        tiles.insert(pt);

        // if we reached the start, we're done
        if score == 0 {
            return;
        }

        // find all the ways we could have reached this point with the current score
        for ((p, dir), s) in scores.iter() {
            if !(*p == pt && *s == score) {
                continue;
            }

            // find the points that must have come before this one by matching
            // what the score would have had to have been at those points
            // in order to get here
            let prev_pt = pt + dir.reverse().step();
            let forward = *dir;

            // did we come from that point going in the same direction, or did
            // we turn left or right to get here?
            for prev_dir in [*dir, dir.turn_right(), dir.turn_left()] {
                if prev_dir != forward && score < 1001 {
                    continue;
                }

                let prev_score = if prev_dir == forward {
                    score - 1
                } else {
                    score - 1001
                };

                if scores
                    .get(&(prev_pt, prev_dir))
                    .is_some_and(|s| *s == prev_score)
                {
                    Self::explore_backwards(prev_pt, prev_score, scores, tiles);
                }
            }
        }
    }
}

trait DirMoves {
    fn turn_right(&self) -> Self;
    fn turn_left(&self) -> Self;
    fn reverse(&self) -> Self;
}

impl DirMoves for Dir {
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

    fn reverse(&self) -> Self {
        match self {
            Dir::North => Self::South,
            Dir::South => Self::North,
            Dir::East => Self::West,
            Dir::West => Self::East,
        }
    }
}
