use crate::{Racetrack, Tile};
use common::{point::Point, puzzle::PuzzlePart};
use std::collections::HashMap;

pub struct Puzzle20a {}

impl PuzzlePart for Puzzle20a {
    fn description() -> &'static str {
        "Count the number of 2-second 'cheats' that would save at least 100 steps in a shortest path maze."
    }

    fn solve(input: &str) -> String {
        // just to make this work with both inputs
        let threshold = if input.lines().count() > 20 { 100 } else { 20 };

        let grid = Racetrack::parse(input);
        let start = grid.find_pt(|t| t == Tile::Start).unwrap();
        let end = grid.find_pt(|t| t == Tile::End).unwrap();

        let path = grid.shortest_path(start, end, Tile::is_space).unwrap();

        let path_indices: HashMap<Point, usize> =
            path.iter().enumerate().map(|(i, p)| (*p, i)).collect();

        let cheats = grid.small_cheats(&path);

        cheats
            .iter()
            .filter_map(|(p1, p2)| {
                let i1 = path_indices[p1];
                let i2 = path_indices[p2];

                if i1 > i2 {
                    None
                } else {
                    Some(i2 - i1 - 2)
                }
            })
            .filter(|s| *s >= threshold)
            .count()
            .to_string()
    }
}
