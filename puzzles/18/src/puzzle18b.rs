use crate::{make_coords, ByteGrid};
use common::{point::Point, puzzle::PuzzlePart};

pub struct Puzzle18b {}

impl PuzzlePart for Puzzle18b {
    fn description() -> &'static str {
        "Find the coordinates of the first point (after the initial maze) that blocks off the exit."
    }

    fn solve(input: &str) -> String {
        // just to make both the example and the normal input work together
        let max_bytes = if input.lines().count() > 1000 {
            1024
        } else {
            12
        };

        let mut grid = ByteGrid::parse(input, max_bytes).unwrap();

        let remaining_pts = input
            .lines()
            .skip(max_bytes + 1)
            .map(make_coords)
            .map(Option::unwrap)
            .map(Point::from);

        let from = (0, 0).into();
        let to = (grid.height() as i64 - 1, grid.width() as i64 - 1).into();

        for pt in remaining_pts {
            grid.add_byte(pt);

            if grid.a_star(from, to).is_none() {
                return format!("{},{}", pt.row, pt.col);
            }
        }

        String::from("Something went wrong!")
    }
}
