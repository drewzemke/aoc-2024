use common::puzzle::PuzzlePart;

use crate::TrailGrid;

pub struct Puzzle10a {}

impl PuzzlePart for Puzzle10a {
    fn description() -> &'static str {
        "Compute the sum of the 'scores' of trailheads in a map."
    }

    fn solve(input: &str) -> String {
        TrailGrid::parse(input).trailhead_count(false).to_string()
    }
}
