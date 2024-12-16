use crate::{Instructions, WarehouseGrid};
use common::puzzle::PuzzlePart;

pub struct Puzzle15a {}

impl PuzzlePart for Puzzle15a {
    fn description() -> &'static str {
        "Find the coordinates of some boxes after they are pushed around by a robot."
    }

    fn solve(input: &str) -> String {
        let (grid_str, instructions_str) = input.split_once("\n\n").unwrap();

        let mut grid = WarehouseGrid::parse(grid_str);
        let instructions = Instructions::parse(instructions_str);

        grid.move_robot(instructions);

        grid.box_gps_coord_sum().to_string()
    }
}
