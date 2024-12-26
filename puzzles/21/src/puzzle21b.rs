use crate::{Memo, NumpadButton};
use common::puzzle::PuzzlePart;

pub struct Puzzle21b {}

impl PuzzlePart for Puzzle21b {
    fn description() -> &'static str {
        "Find the sums of 'complexities' of codes that need to be input by a sequence of even more robotic arms."
    }

    fn solve(input: &str) -> String {
        let mut memo: Memo = Memo::new();

        input
            .lines()
            .map(|line| {
                let buttons = line.chars().map(NumpadButton::from).collect::<Vec<_>>();
                (line, buttons)
            })
            .map(|(line, seq)| {
                let num = &line[0..3].parse().unwrap();
                let num_button = NumpadButton::num_buttons_to_press(&seq, 26, &mut memo);
                num_button * num
            })
            .sum::<usize>()
            .to_string()
    }
}
