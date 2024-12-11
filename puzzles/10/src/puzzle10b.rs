use common::puzzle::PuzzlePart;

use crate::TrailGrid;

pub struct Puzzle10b {}

impl PuzzlePart for Puzzle10b {
    fn description() -> &'static str {
        "Compute the sum of the 'ratings' of trailheads in a map."
    }

    fn solve(input: &str) -> String {
        TrailGrid::parse(input).trailhead_count(true).to_string()
    }
}
