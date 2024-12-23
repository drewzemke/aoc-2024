use crate::{Racetrack, Tile};
use common::puzzle::PuzzlePart;

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
        let cheats = grid.cheats(&path, 2);

        grid.savings(&path, cheats)
            .into_iter()
            .filter(|s| *s >= threshold)
            .count()
            .to_string()
    }
}
