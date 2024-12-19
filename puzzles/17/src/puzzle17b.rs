use crate::Computer;
use common::puzzle::PuzzlePart;

pub struct Puzzle17b {}

impl PuzzlePart for Puzzle17b {
    fn description() -> &'static str {
        "Find the smallest possible initial value in register A so that the program outputs itself."
    }

    /// strat: search for a value of A that prints a certain suffix of the progam
    /// then, since A is divided by 8 between loops of the program,
    /// check all "preimages" of the found value under division by 8 in order
    /// to find a value of A that leads to a longer suffix. repeat until you win
    fn solve(input: &str) -> String {
        let mut computer = Computer::parse(input).unwrap();

        let target = &computer.program[4..];

        // start at 8^11, since we're initially trying to match a sequence of length 12,
        // and we divide by 8 between iterations
        // (this takes quite a while to run, so go get a snack or something)
        computer.reg_a = 8589934592;

        // starting here makes it finish immediately but its' sorta cheating because
        // I found this value experimentally
        // (or maybe it isn't because it took me several hours of tinkering and experimenting)
        // computer.reg_a = 52770590484;

        loop {
            computer.reg_a += 1;

            if computer.has_output(target) {
                if let Some(res) = check_preimages(&computer, 3, computer.reg_a) {
                    break res.to_string();
                }
            }
        }
    }
}

fn check_preimages(computer: &Computer, cursor: usize, base: i64) -> Option<i64> {
    let mut comp = computer.clone();
    let target = &computer.program[cursor..];

    for offset in 0..8 {
        let a = 8 * base + offset;
        comp.reg_a = a;
        if comp.has_output(target) {
            if cursor == 0 {
                return Some(a);
            }

            let next_check = check_preimages(&comp, cursor - 1, a);
            if next_check.is_some() {
                return next_check;
            }
        }
    }

    None
}
