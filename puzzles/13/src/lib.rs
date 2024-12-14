pub mod puzzle13a;
pub mod puzzle13b;

const EMBIGGEN_FACTOR: i64 = 10_000_000_000_000;

#[derive(Debug)]
struct ClawMachine {
    button_a: (i64, i64),
    button_b: (i64, i64),
    prize: (i64, i64),
}

impl ClawMachine {
    pub fn parse(input: &str) -> Option<Self> {
        let rest = input.strip_prefix("Button A: X+")?;
        let (ax, rest) = rest.split_once(", Y+")?;
        let (ay, rest) = rest.split_once("\n")?;

        let rest = rest.strip_prefix("Button B: X+")?;
        let (bx, rest) = rest.split_once(", Y+")?;
        let (by, rest) = rest.split_once("\n")?;

        let rest = rest.strip_prefix("Prize: X=")?;
        let (px, py) = rest.split_once(", Y=")?;

        Some(Self {
            button_a: (ax.parse().ok()?, ay.parse().ok()?),
            button_b: (bx.parse().ok()?, by.parse().ok()?),
            prize: (px.parse().ok()?, py.parse().ok()?),
        })
    }

    // we're looking for the values of a and b that solve this linear system:
    //   | ax bx | * | a | = | px |
    //   | ay by |   | b |   | py |
    // if D = ax * by - ay * bx, then this has a solution that looks like this:
    //   | a | = (1/D) |  by -bx | * | px |
    //   | b |         | -ay  ax |   | py |
    // expand that product to find a and b:
    //   a = (1/D) ( by*px - bx*py )
    //   b = (1/D) ( -ay*px + ax*py )
    pub fn solve(&self) -> Option<(i64, i64)> {
        let (ax, ay) = self.button_a;
        let (bx, by) = self.button_b;
        let (px, py) = self.prize;

        let det = ax * by - ay * bx;

        let a = by * px - bx * py;
        if a % det != 0 {
            return None;
        }
        let a = a / det;

        let b = -ay * px + ax * py;
        if b % det != 0 {
            return None;
        }
        let b = b / det;

        Some((a, b))
    }

    pub fn embiggen(&mut self) {
        let (px, py) = self.prize;
        self.prize = (EMBIGGEN_FACTOR + px, EMBIGGEN_FACTOR + py);
    }
}
