use common::point::Point;

pub mod puzzle14a;
pub mod puzzle14b;

#[derive(Debug, PartialEq, Eq)]
struct RayRobot {
    start: Point,
    vel: Point,
}

impl RayRobot {
    /// parse a string into a robot
    ///
    /// input looks like
    ///   p=x,y v=a,b
    /// where x,y,a,b are ints and x,y are nonnegative
    pub fn parse(line: &str) -> Option<Self> {
        let rest = line.strip_prefix("p=")?;
        let (x, rest) = rest.split_once(',')?;
        let (y, rest) = rest.split_once(" v=")?;
        let (a, b) = rest.split_once(',')?;

        Some(Self {
            start: (x.parse::<i64>().ok()?, y.parse::<i64>().ok()?).into(),
            vel: (a.parse::<i64>().ok()?, b.parse::<i64>().ok()?).into(),
        })
    }

    /// determines where the robot will be after `s` seconds
    ///
    /// computed as
    ///   P = start + s * vel
    pub fn eval(&self, s: i64) -> Point {
        self.start + Point::from((s * self.vel.row, s * self.vel.col))
    }
}

#[cfg(test)]
mod ray_robot_tests {
    use super::*;

    #[test]
    fn should_parse() {
        let line = "p=0,4 v=3,-3";

        assert_eq!(
            RayRobot::parse(line).unwrap(),
            RayRobot {
                start: (0, 4).into(),
                vel: (3, -3).into()
            }
        );
    }

    #[test]
    fn should_eval() {
        let bot = RayRobot {
            start: (0, 4).into(),
            vel: (3, -3).into(),
        };

        assert_eq!(bot.eval(1), (3, 1).into());
        assert_eq!(bot.eval(100), (300, -296).into());
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Quadrant {
    I,
    II,
    #[expect(clippy::upper_case_acronyms)]
    III,
    IV,
}

trait ComputeQuadrant {
    /// computes which "quadrant" a point would be in a grid of a given size.
    /// (quadrants are obtained by splitting the grid in half horizontally and vertially)
    /// if it matters, the quadrants are arranged in the usual way:
    ///   II | I
    ///  --------
    ///  III | IV
    /// also, this should return `None` for points lying directly on the borders
    /// between adjacent qudrants
    ///
    /// NOTE: This assumes the coordinates of `size` are positive odd integers
    fn quadrant(&self, size: (i64, i64)) -> Option<Quadrant>;
}

impl ComputeQuadrant for Point {
    fn quadrant(&self, size: (i64, i64)) -> Option<Quadrant> {
        let half_x = (size.0) / 2;
        let half_y = (size.1 - 1) / 2;

        if self.row == half_x || self.col == half_y {
            return None;
        }

        match (self.row < half_x, self.col < half_y) {
            (true, true) => Some(Quadrant::II),
            (true, false) => Some(Quadrant::III),
            (false, true) => Some(Quadrant::I),
            (false, false) => Some(Quadrant::IV),
        }
    }
}

#[cfg(test)]
mod quadrant_tests {
    use super::*;

    #[test]
    fn should_compute_quadrant() {
        assert_eq!(Point::from((0, 2)).quadrant((11, 7)), Some(Quadrant::II));
        assert_eq!(Point::from((10, 2)).quadrant((11, 7)), Some(Quadrant::I));
        assert_eq!(Point::from((5, 4)).quadrant((11, 7)), None);
    }
}
