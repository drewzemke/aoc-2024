#[derive(Debug)]
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

    pub fn at(&self, pt: (usize, usize)) -> &T {
        &self.0[pt.0][pt.1]
    }

    pub fn at_unchecked(&self, pt: (i64, i64)) -> &T {
        &self.0[pt.0 as usize][pt.1 as usize]
    }

    pub fn width(&self) -> usize {
        self.0[0].len()
    }

    pub fn height(&self) -> usize {
        self.0.len()
    }

    pub fn contains(&self, (row, col): (i64, i64)) -> bool {
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
    fn parse_number_grid() {
        let input = "12\n34\n56";
        let grid = Grid::<char>::parse(input);

        assert_eq!(*grid.at((1, 1)), '4');
        assert_eq!(grid.width(), 2);
        assert_eq!(grid.height(), 3);
    }
}
