use crate::NumpadButton;
use common::puzzle::PuzzlePart;

pub struct Puzzle21a {}

impl PuzzlePart for Puzzle21a {
    fn description() -> &'static str {
        "Find the sums of 'complexities' of codes that need to be input by a sequence of robotic arms."
    }

    fn solve(input: &str) -> String {
        input
            .lines()
            .map(|line| {
                let buttons = line.chars().map(NumpadButton::from).collect::<Vec<_>>();
                (line, buttons)
            })
            // "complexity" is the product of the length of the shortest sequence of buttons
            // (in a 3-layer machine) and the number in the first three characters of the
            // sequence
            .map(|(line, seq)| {
                let num = &line[0..3].parse().unwrap();
                let buttons = NumpadButton::seq_to_press(&seq, 3);
                buttons.len() * num
            })
            .sum::<usize>()
            .to_string()
    }
}
