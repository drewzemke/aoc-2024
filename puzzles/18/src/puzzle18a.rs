use crate::ByteGrid;
use common::puzzle::PuzzlePart;

pub struct Puzzle18a {}

impl PuzzlePart for Puzzle18a {
    fn description() -> &'static str {
        "Find the minimum number of steps needed to reach from one corner of a maze of fallen bytes to the other."
    }

    fn solve(input: &str) -> String {
        // just to make both the example and the normal input work together
        let max_bytes = if input.lines().count() > 1000 {
            1024
        } else {
            12
        };

        let grid = ByteGrid::parse(input, max_bytes).unwrap();

        let start = (0, 0).into();
        let end = (grid.height() as i64 - 1, grid.width() as i64 - 1).into();
        (grid.a_star(start, end).unwrap().len() - 1).to_string()
    }
}
