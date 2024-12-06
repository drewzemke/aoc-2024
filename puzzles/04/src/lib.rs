pub mod puzzle04a;
pub mod puzzle04b;

#[derive(Debug)]
pub struct Grid(Vec<Vec<char>>);

impl Grid {
    pub fn parse(input: &str) -> Self {
        let data = input.lines().map(|line| line.chars().collect()).collect();
        Self(data)
    }

    pub fn at(&self, pt: (usize, usize)) -> char {
        self.0[pt.0][pt.1]
    }

    pub fn width(&self) -> usize {
        self.0[0].len()
    }

    pub fn height(&self) -> usize {
        self.0.len()
    }

    fn matches(&self, pattern: &Grid, start: (usize, usize)) -> bool {
        for row_offset in 0..pattern.height() {
            for col_offset in 0..pattern.width() {
                let pattern_char = pattern.at((row_offset, col_offset));
                let self_char = self.at((start.0 + row_offset, start.1 + col_offset));

                if pattern_char != '.' && pattern_char != self_char {
                    return false;
                }
            }
        }

        true
    }

    fn count_matches(&self, pattern: &Grid) -> usize {
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
