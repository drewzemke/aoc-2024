use clap::{Parser, ValueEnum};
use colored::Colorize;
use std::{
    fs,
    path::Path,
    time::{Duration, Instant},
};

#[derive(ValueEnum, Clone)]
enum PuzzlePartName {
    A,
    B,
}

#[derive(Parser)]
struct PuzzleArgs {
    /// Which part of the puzzle to solve
    #[arg(long, short)]
    part: Option<PuzzlePartName>,

    /// Whether to use example input instead of the main input
    #[arg(long, short = 'e')]
    example: bool,
}

pub trait PuzzlePart {
    /// A description of the value(s) computed for this puzzle,
    /// just for the sake of more meaningful output.
    fn description() -> &'static str;

    /// Do all of the work necessary to transform the input text into
    /// the solution text.
    fn solve(input: &str) -> String;
}

pub trait Puzzle {
    type PartA: PuzzlePart;
    type PartB: PuzzlePart;

    /// The name of the puzzle, usually just the number (eg. "01")
    fn name() -> &'static str;

    /// Based on command line args, this executes the solver for one or both
    /// parts of a day's puzzles, using either the primary input or the example input.
    fn run(input: &str, example: &str) {
        let args = PuzzleArgs::parse();

        let input = if args.example { example } else { input };

        match args.part {
            Some(PuzzlePartName::A) => Self::process::<Self::PartA>("A", input, args.example, true),
            Some(PuzzlePartName::B) => Self::process::<Self::PartB>("B", input, args.example, true),
            None => {
                Self::process::<Self::PartA>("A", input, args.example, false);
                println!("{}", "---".dimmed());
                Self::process::<Self::PartB>("B", input, args.example, false);
            }
        };
    }

    fn process<P: PuzzlePart>(part_name: &str, input: &str, is_example: bool, write_to_file: bool) {
        print_puzzle_info::<P>(Self::name(), part_name, is_example);

        let start_time = Instant::now();
        let solution = <P as PuzzlePart>::solve(input);
        let duration = start_time.elapsed();

        print_results(&solution, duration);

        if write_to_file && Path::new("data").exists() {
            fs::write(format!("data/soln-{}", part_name.to_lowercase()), solution).unwrap();
        }
    }
}

fn print_puzzle_info<P: PuzzlePart>(puzzle_name: &str, part_name: &str, is_example: bool) {
    let puzzle_name = format!("Puzzle {} Part {part_name}", puzzle_name).blue();
    let example_note = if is_example {
        format!(" ({})", "example input".yellow())
    } else {
        "".into()
    };
    println!("Solving {puzzle_name}{example_note}:");

    let description = format!("\"{}\"", <P as PuzzlePart>::description()).dimmed();
    println!("{description}");
}

fn print_results(solution: &str, duration: Duration) {
    println!(
        "{} {}",
        format!("Solution: {}", solution.magenta()).bold(),
        format!("(in {})", format!("{duration:?}").green()).dimmed()
    );
}
