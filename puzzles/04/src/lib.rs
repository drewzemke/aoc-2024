pub mod puzzle04a;
pub mod puzzle04b;

#[derive(Debug)]
pub struct Xmas {
    start: (i32, i32),
    dir: Direction,
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    North,
    South,
    East,
    West,
    Northeast,
    Southeast,
    Northwest,
    Southwest,
}

impl Direction {
    pub const fn all() -> [Self; 8] {
        [
            Self::North,
            Self::South,
            Self::East,
            Self::West,
            Self::Northeast,
            Self::Southeast,
            Self::Northwest,
            Self::Southwest,
        ]
    }

    pub fn offset(&self) -> (i32, i32) {
        match self {
            Direction::North => (-1, 0),
            Direction::South => (1, 0),
            Direction::East => (0, 1),
            Direction::West => (0, -1),
            Direction::Northeast => (-1, 1),
            Direction::Southeast => (1, 1),
            Direction::Northwest => (-1, -1),
            Direction::Southwest => (1, -1),
        }
    }
}

#[derive(Debug)]
pub struct Grid(Vec<Vec<char>>);

impl Grid {
    pub fn parse(input: &str) -> Self {
        let data = input.lines().map(|line| line.chars().collect()).collect();
        Self(data)
    }

    pub fn at(&self, pt: (i32, i32)) -> char {
        self.0[pt.0 as usize][pt.1 as usize]
    }

    pub fn width(&self) -> usize {
        self.0[0].len()
    }

    pub fn height(&self) -> usize {
        self.0.len()
    }

    fn has_substring(&self, string: &str, start: (i32, i32), dir: Direction) -> bool {
        let Some(first) = string.chars().next() else {
            // empty string means we've matched the whole string
            return true;
        };

        // check for out of bounds
        if start.0 < 0
            || start.0 >= self.height() as i32
            || start.1 < 0
            || start.1 >= self.width() as i32
        {
            return false;
        }

        if first != self.at(start) {
            return false;
        }

        let offset = dir.offset();
        let next = (start.0 + offset.0, start.1 + offset.1);

        self.has_substring(&string[1..], next, dir)
    }

    pub fn find_xmases(&self) -> Vec<Xmas> {
        let mut out = vec![];

        for row_index in 0..self.height() {
            for col_index in 0..self.width() {
                for dir in Direction::all() {
                    let start = (row_index as i32, col_index as i32);
                    if self.has_substring("XMAS", start, dir) {
                        out.push(Xmas { start, dir });
                    }
                }
            }
        }

        out
    }

    pub fn print_xmases(&self, xmases: Vec<Xmas>) -> String {
        let mut out = String::new();

        // DEBUG: walk each xmas so we can print them
        let mut pts = vec![];
        for xmas in &xmases {
            let mut pt = xmas.start;
            for _ in "XMAS".chars() {
                pts.push(pt);
                let offset = xmas.dir.offset();
                pt = ((pt.0 + offset.0), (pt.1 + offset.1));
            }
        }

        for row_index in 0..self.height() {
            for col_index in 0..self.width() {
                let pt = (row_index as i32, col_index as i32);
                if pts.iter().any(|p| *p == pt) {
                    out.push(self.at(pt));
                } else {
                    out.push('.')
                }
            }
            out.push('\n');
        }

        out
    }
}
