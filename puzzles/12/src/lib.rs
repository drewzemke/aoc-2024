use std::collections::HashSet;

use common::{grid::Grid, point::Point};

pub mod puzzle12a;
pub mod puzzle12b;

// TODO: move this to common!
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Dir {
    North,
    South,
    East,
    West,
}

impl Dir {
    pub fn all() -> Vec<Self> {
        vec![Self::North, Self::South, Self::East, Self::West]
    }

    pub fn step(&self) -> Point {
        match self {
            Dir::North => (-1, 0),
            Dir::South => (1, 0),
            Dir::East => (0, 1),
            Dir::West => (0, -1),
        }
        .into()
    }
}

#[derive(Debug)]
pub struct GardenGrid(Grid<char>);

impl std::ops::Deref for GardenGrid {
    type Target = Grid<char>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl GardenGrid {
    pub fn parse(input: &str) -> Self {
        Self(Grid::parse(input))
    }

    pub fn regions(&self) -> Vec<Region> {
        let mut visited: HashSet<Point> = HashSet::new();
        let mut regions = vec![];

        for (row_idx, row) in self.rows().enumerate() {
            for (col_idx, char) in row.enumerate() {
                let pt = (row_idx as i64, col_idx as i64).into();

                if visited.contains(&pt) {
                    continue;
                }

                let mut points = vec![];
                self.compute_region_from(&pt, char, &mut points, &mut visited);

                regions.push(Region { char, points });
            }
        }

        regions
    }

    fn compute_region_from(
        &self,
        pt: &Point,
        char: char,
        rgn_pts: &mut Vec<Point>,
        visited: &mut HashSet<Point>,
    ) {
        visited.insert(*pt);
        rgn_pts.push(*pt);

        // check every direction from this point;
        // if the neighboring point matches the character, add to the set
        // and recurse on that point
        for dir in Dir::all() {
            let neighbor = pt + dir.step();
            if self.contains(neighbor) && *self.at(neighbor) == char && !visited.contains(&neighbor)
            {
                self.compute_region_from(&neighbor, char, rgn_pts, visited);
            }
        }
    }
}

#[derive(Debug)]
pub struct Region {
    #[expect(dead_code)]
    char: char,
    points: Vec<Point>,
}

impl Region {
    pub fn area(&self) -> usize {
        self.points.len()
    }

    pub fn perimeter(&self) -> usize {
        let mut perimeter = 0;
        let mut visited: HashSet<Point> = HashSet::new();

        let Some(start) = self.points.first() else {
            return 0;
        };

        self.compute_perim_from(start, &mut perimeter, &mut visited);

        perimeter
    }

    fn compute_perim_from(&self, pt: &Point, perim: &mut usize, visited: &mut HashSet<Point>) {
        visited.insert(*pt);

        // check every direction from this point.
        // - recurse if the adjacent point is in the region,
        // - otherwise add one to the perimeter
        for dir in Dir::all() {
            let neighbor = pt + dir.step();
            if self.points.contains(&neighbor) {
                if !visited.contains(&neighbor) {
                    self.compute_perim_from(&neighbor, perim, visited);
                }
            } else {
                *perim += 1;
            }
        }
    }
}

#[cfg(test)]
mod region_tests {
    use super::*;

    #[test]
    fn should_compute_area() {
        let region = Region {
            char: 'A',
            points: vec![(0, 0).into(), (0, 1).into(), (1, 1).into()],
        };

        assert_eq!(region.area(), 3);
    }

    #[test]
    fn should_compute_perimeter() {
        let region = Region {
            char: 'A',
            points: vec![(0, 0).into(), (0, 1).into(), (1, 1).into()],
        };

        assert_eq!(region.perimeter(), 8);
    }
}
