use crate::{ComputeQuadrant, Quadrant, RayRobot};
use common::{point::Point, puzzle::PuzzlePart};

pub struct Puzzle14a {}

impl PuzzlePart for Puzzle14a {
    fn description() -> &'static str {
        "Find the configuration of a bunch of robots after moving in lines over a torus for 100 seconds."
    }

    fn solve(input: &str) -> String {
        let mut lines = input.lines();

        // the first line of input contains the size
        let size = lines
            .next()
            .and_then(|line| line.split_once(','))
            .and_then(|(w, h)| Some((w.parse().ok()?, h.parse().ok()?)))
            .unwrap();

        let robots = lines
            .map(RayRobot::parse)
            .map(Option::unwrap)
            .collect::<Vec<_>>();

        let destinations = robots
            .iter()
            // move for 100 seconds:
            .map(|r| r.eval(100))
            // map back into the main rectangle:
            .map(|pt| (pt.row.rem_euclid(size.0), pt.col.rem_euclid(size.1)).into())
            .collect::<Vec<Point>>();

        let (i, ii, iii, iv) = destinations
            .iter()
            .fold((0, 0, 0, 0), |(i, ii, iii, iv), pt| {
                match pt.quadrant(size) {
                    Some(Quadrant::I) => (i + 1, ii, iii, iv),
                    Some(Quadrant::II) => (i, ii + 1, iii, iv),
                    Some(Quadrant::III) => (i, ii, iii + 1, iv),
                    Some(Quadrant::IV) => (i, ii, iii, iv + 1),
                    None => (i, ii, iii, iv),
                }
            });

        (i * ii * iii * iv).to_string()
    }
}
