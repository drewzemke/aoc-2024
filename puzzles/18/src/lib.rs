use std::collections::HashMap;

use common::{dir::Dir, grid::Grid, grid_def, point::Point};

pub mod puzzle18a;
pub mod puzzle18b;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tile {
    Byte,
    Nothing,
}

impl From<Tile> for char {
    fn from(value: Tile) -> Self {
        match value {
            Tile::Byte => '#',
            Tile::Nothing => '.',
        }
    }
}

grid_def!(ByteGrid, Tile);

pub fn make_coords(line: &str) -> Option<(i64, i64)> {
    let (rows_str, cols_str) = line.split_once(',')?;
    Some((rows_str.parse::<i64>().ok()?, cols_str.parse::<i64>().ok()?))
}

impl ByteGrid {
    pub fn parse(input: &str, max_bytes: usize) -> Option<Self> {
        let mut lines = input.lines();

        // the first line has the size
        let (num_rows, num_cols) = lines.next().and_then(make_coords)?;

        // the rest are coordinates
        let coords = lines
            .take(max_bytes)
            .map(make_coords)
            .map(Option::unwrap)
            .collect::<Vec<_>>();

        // make a grid
        let grid = (0..num_rows)
            .map(|row_idx| {
                (0..num_cols)
                    .map(|col_idx| {
                        if coords.contains(&(row_idx, col_idx)) {
                            Tile::Byte
                        } else {
                            Tile::Nothing
                        }
                    })
                    .collect()
            })
            .collect();

        Some(Self(Grid(grid)))
    }

    pub fn add_byte(&mut self, pt: Point) {
        self.put(Tile::Byte, pt);
    }

    // TODO: try A*?
    pub fn least_steps(&self, start: Point, end: Point) -> Option<usize> {
        let mut least_step_map = HashMap::<Point, usize>::new();

        self.explore(start, 0, &mut least_step_map);

        least_step_map.get(&end).cloned()
    }

    // dumb algorithm that just checks in every possible direction, updating the map if we
    // find our way to a point we've already visited with a better score
    fn explore(&self, pt: Point, steps: usize, data: &mut HashMap<Point, usize>) {
        data.insert(pt, steps);

        for dir in Dir::all() {
            let neighbor = pt + dir.step();

            if self.at(neighbor).is_some_and(|t| *t != Tile::Byte)
                && (data.get(&neighbor).is_none()
                    || data.get(&neighbor).is_some_and(|v| steps + 1 < *v))
            {
                self.explore(neighbor, steps + 1, data);
            }
        }
    }
}
