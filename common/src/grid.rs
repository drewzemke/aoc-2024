use crate::point::Point;

mod a_star;

#[derive(Clone, Debug)]
/// Represents a 2D grid of tiles
pub struct Grid<T>(pub Vec<Vec<T>>);

/// Defines a wrapped `Grid` with the given name and tile type,
/// and implements some helper traits for the new type.
#[macro_export]
macro_rules! grid_def {
    ($grid_name:ident, $tile_type:ty) => {
        #[derive(Clone, Debug)]
        pub struct $grid_name(common::grid::Grid<$tile_type>);

        impl std::ops::Deref for $grid_name {
            type Target = common::grid::Grid<$tile_type>;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl std::ops::DerefMut for $grid_name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}

// TODO: more tests
impl<T> Grid<T> {
    pub fn parse_with<F>(input: &str, f: F) -> Grid<T>
    where
        F: FnMut(char) -> T + Copy,
    {
        let data = input
            .lines()
            .map(|line| line.chars().map(f).collect())
            .collect();
        Self(data)
    }

    pub fn parse(input: &str) -> Grid<T>
    where
        T: From<char>,
    {
        Self::parse_with(input, T::from)
    }

    pub fn at(&self, pt: Point) -> Option<&T> {
        if pt.row < 0 || pt.col < 0 {
            None
        } else {
            self.0
                .get(pt.row as usize)
                .and_then(|row| row.get(pt.col as usize))
        }
    }

    pub fn width(&self) -> usize {
        self.0[0].len()
    }

    pub fn height(&self) -> usize {
        self.0.len()
    }

    pub fn rows(&self) -> impl Iterator<Item = impl Iterator<Item = T> + '_> + '_
    where
        T: Clone,
    {
        self.0.iter().map(|row| row.iter().cloned())
    }

    pub fn contains(&self, Point { row, col }: Point) -> bool {
        row >= 0 && row < self.height() as i64 && col >= 0 && col < self.width() as i64
    }

    pub fn find_pt(&self, pred: impl Fn(T) -> bool) -> Option<Point>
    where
        T: Clone,
    {
        self.rows().enumerate().find_map(|(row_idx, row)| {
            row.enumerate().find_map(|(col_idx, tile)| {
                if pred(tile) {
                    Some((row_idx as i64, col_idx as i64).into())
                } else {
                    None
                }
            })
        })
    }

    pub fn put(&mut self, that: T, here: Point) {
        self.0[here.row as usize][here.col as usize] = that;
    }
}

impl<T> std::fmt::Display for Grid<T>
where
    T: Into<char> + Clone,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (idx, row) in self.rows().enumerate() {
            for tile in row {
                let c: char = tile.into();
                f.write_str(&c.to_string())?;
            }
            if idx < self.height() - 1 {
                f.write_str(&'\n'.to_string())?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_number_grid() {
        let input = "12\n34\n56";
        let grid = Grid::<char>::parse(input);

        assert_eq!(grid.at((1, 1).into()), Some(&'4'));
        assert_eq!(grid.width(), 2);
        assert_eq!(grid.height(), 3);
        assert_eq!(grid.to_string(), input)
    }
}
