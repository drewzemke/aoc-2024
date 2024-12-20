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

        // the program has 16 elements, so we'll start at the last one and work our way forwards
        let start_idx = 15;
        let target = &computer.program[start_idx..];

        // start with A at 0 and increase until we find a program that outputs the last
        // element of the program.
        // if we find one, check all "preimages" under integer division by 8 for a value of A
        // that yields the last two elements of the program, then repeat
        computer.reg_a = 0;

        loop {
            computer.reg_a += 1;

            if computer.has_output(target) {
                if let Some(res) = check_preimages(&computer, start_idx - 1, computer.reg_a) {
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
