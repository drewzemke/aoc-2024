use common::grid::Grid;

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

    fn matches(&self, pattern: &Grid<char>, start: (usize, usize)) -> bool {
        for row_offset in 0..pattern.height() {
            for col_offset in 0..pattern.width() {
                let pattern_char = pattern.at((row_offset, col_offset));
                let self_char = self.at((start.0 + row_offset, start.1 + col_offset));

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
                if self.matches(pattern, (row, col)) {
                    out += 1;
                }
            }
        }

        out
    }
}
