use crate::GardenGrid;
use common::puzzle::PuzzlePart;

pub struct Puzzle12b {}

impl PuzzlePart for Puzzle12b {
    fn description() -> &'static str {
        "Find the sum of the 'prices' (area * number of sides) of a bunch of regions of a grid."
    }

    fn solve(input: &str) -> String {
        GardenGrid::parse(input)
            .regions()
            .iter()
            .map(|r| r.area() * r.num_sides())
            .sum::<usize>()
            .to_string()
    }
}
