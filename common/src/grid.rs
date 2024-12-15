use crate::point::Point;

#[derive(Clone, Debug)]
/// Represents a 2D grid of tiles
pub struct Grid<T>(pub Vec<Vec<T>>);

impl<T> Grid<T> {
    pub fn parse(input: &str) -> Grid<T>
    where
        T: From<char>,
    {
        let data = input
            .lines()
            .map(|line| line.chars().map(T::from).collect())
            .collect();
        Self(data)
    }

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

    pub fn contains(&self, Point { row, col }: Point) -> bool {
        row >= 0 && row < self.height() as i64 && col >= 0 && col < self.width() as i64
    }

    pub fn rows(&self) -> impl Iterator<Item = impl Iterator<Item = T> + '_> + '_
    where
        T: Clone,
    {
        self.0.iter().map(|row| row.iter().cloned())
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
    }
}
