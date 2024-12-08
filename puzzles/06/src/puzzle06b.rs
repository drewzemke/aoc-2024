use crate::GuardGrid;
use common::puzzle::PuzzlePart;

pub struct Puzzle06b {}

impl PuzzlePart for Puzzle06b {
    fn description() -> &'static str {
        "Count the number of positions in which an added obstruction would cause the guard to walk in a loop."
    }

    fn solve(input: &str) -> String {
        let (grid, start, dir) = GuardGrid::parse(input);

        grid.loop_causing_obstacles(start, dir).to_string()
    }
}
