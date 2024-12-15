use common::{dir::Dir, grid::Grid, point::Point};
use std::collections::HashSet;

pub mod puzzle12a;
pub mod puzzle12b;

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

    // count sides by counting corners
    pub fn num_sides(&self) -> usize {
        // iterate over a grid that contains this region, along with some padding
        let rows = self.points.iter().map(|p| p.row);
        let cols = self.points.iter().map(|p| p.col);
        let min_row = rows.clone().min().unwrap();
        let max_row = rows.max().unwrap() + 1;
        let min_col = cols.clone().min().unwrap() - 1;
        let max_col = cols.max().unwrap() + 1;

        let mut sides = 0;

        for row in min_row..=max_row {
            for col in min_col..=max_col {
                // look at the points around each point in order to figure out if there's a corner here
                let here = self.points.contains(&(row, col).into());
                let above = self.points.contains(&(row - 1, col).into());
                let left = self.points.contains(&(row, col - 1).into());
                let above_left = self.points.contains(&(row - 1, col - 1).into());

                //  _
                // _x
                //  ^
                let outer_top_left_corner = !above && !left && here;

                // _
                // x_
                //  ^
                let outer_top_right_corner = !above_left && left && !here;

                // x_
                // _
                //  ^
                let outer_bottom_right_corner = above_left && !above && !left;

                // _x
                //  _
                //  ^
                let outer_bottom_left_corner = !above_left && above && !here;

                //  x
                // x_
                //  ^
                let inner_top_left_corner = left && above && !here;

                // x
                // _x
                //  ^
                let inner_top_right_corner = above_left && !left && here;

                // _x
                // x
                //  ^
                let inner_bottom_right_corner = !above_left && above && left;

                // x_
                //  x
                //  ^
                let inner_bottom_left_corner = above_left && !above && here;

                if outer_top_left_corner
                    || outer_top_right_corner
                    || outer_bottom_right_corner
                    || outer_bottom_left_corner
                    || inner_top_left_corner
                    || inner_top_right_corner
                    || inner_bottom_right_corner
                    || inner_bottom_left_corner
                {
                    sides += 1;
                }

                // these two patterns are two corners at the same point,
                // so we need to double count them:
                //
                // _x
                // x_
                //  ^
                let double_corner1 = !above_left && above && left && !here;

                // x_
                // _x
                //  ^
                let double_corner2 = above_left && !above && !left && here;

                if double_corner1 || double_corner2 {
                    sides += 1;
                }
            }
        }

        sides
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

    #[test]
    fn should_compute_num_sides() {
        // x
        //
        let region = Region {
            char: 'A',
            points: vec![(10, 29).into()],
        };

        assert_eq!(region.num_sides(), 4);

        // xx
        //  x
        let region = Region {
            char: 'A',
            points: vec![(2, 2).into(), (2, 3).into(), (3, 3).into()],
        };

        assert_eq!(region.num_sides(), 6);

        //  x
        // xxx
        //  x
        let region = Region {
            char: 'A',
            points: vec![
                (0, 1).into(),
                (1, 0).into(),
                (1, 1).into(),
                (1, 2).into(),
                (2, 1).into(),
            ],
        };
        assert_eq!(region.num_sides(), 12);

        // xxx
        // x x
        // xx
        let region = Region {
            char: 'A',
            points: vec![
                (0, 0).into(),
                (0, 1).into(),
                (0, 2).into(),
                (1, 0).into(),
                (1, 2).into(),
                (2, 0).into(),
                (2, 1).into(),
            ],
        };

        // 6 outside + 4 inside
        assert_eq!(region.num_sides(), 10);
    }
}
