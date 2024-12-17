use crate::Maze;
use common::puzzle::PuzzlePart;

pub struct Puzzle16b {}

impl PuzzlePart for Puzzle16b {
    fn description() -> &'static str {
        "Find the number of locations that are in one of the lowest-scoring paths."
    }

    fn solve(input: &str) -> String {
        Maze::parse(input).shortest_path_tiles().to_string()
    }
}
