use crate::GardenGrid;
use common::puzzle::PuzzlePart;

pub struct Puzzle12a {}

impl PuzzlePart for Puzzle12a {
    fn description() -> &'static str {
        "Find the sum of the 'prices' (area * perimeter) of a bunch of regions of a grid"
    }

    fn solve(input: &str) -> String {
        GardenGrid::parse(input)
            .regions()
            .iter()
            .map(|r| r.area() * r.perimeter())
            .sum::<usize>()
            .to_string()
    }
}
