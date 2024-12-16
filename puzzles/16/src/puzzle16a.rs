use crate::Maze;
use common::puzzle::PuzzlePart;

pub struct Puzzle16a {}

impl PuzzlePart for Puzzle16a {
    fn description() -> &'static str {
        "Find the lowest 'score' of a path through a maze."
    }

    fn solve(input: &str) -> String {
        Maze::parse(input).shortest_path_score().to_string()
    }
}
