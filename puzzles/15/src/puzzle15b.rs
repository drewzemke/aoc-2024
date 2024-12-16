use crate::{Instructions, WarehouseGrid};
use common::puzzle::PuzzlePart;

pub struct Puzzle15b {}

impl PuzzlePart for Puzzle15b {
    fn description() -> &'static str {
        "Find the coordinates of some boxes after they are pushed around by a robot, but on a horizontally-scaled map."
    }

    fn solve(input: &str) -> String {
        let (grid_str, instructions_str) = input.split_once("\n\n").unwrap();

        let mut grid = WarehouseGrid::parse(grid_str);
        let instructions = Instructions::parse(instructions_str);

        grid.embiggen();
        grid.move_robot(instructions);
        grid.box_gps_coord_sum().to_string()
    }
}
