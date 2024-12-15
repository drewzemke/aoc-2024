use common::{grid::Grid, point::Point};

pub mod puzzle04a;
pub mod puzzle04b;

#[derive(Debug)]
pub struct XmasGrid(Grid<char>);

impl std::ops::Deref for XmasGrid {
    type Target = Grid<char>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl XmasGrid {
    pub fn parse(input: &str) -> Self {
        Self(Grid::parse(input))
    }

    fn matches(&self, pattern: &Grid<char>, start: Point) -> bool {
        for row_offset in 0..pattern.height() {
            for col_offset in 0..pattern.width() {
                let offset = (row_offset as i64, col_offset as i64).into();
                let pattern_char = pattern.at(offset).expect("point should be in grid");
                let self_char = self
                    .at(start + offset)
                    .expect("point should be in the grid");

                if *pattern_char != '.' && pattern_char != self_char {
                    return false;
                }
            }
        }

        true
    }

    fn count_matches(&self, pattern: &Grid<char>) -> usize {
        let mut out = 0;

        for row in 0..=(self.height() - pattern.height()) {
            for col in 0..=(self.width() - pattern.width()) {
                if self.matches(pattern, (row as i64, col as i64).into()) {
                    out += 1;
                }
            }
        }

        out
    }
}
