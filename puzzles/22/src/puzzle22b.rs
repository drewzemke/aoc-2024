use crate::{DiffSeqIterator, PriceDifferenceIterator};
use common::puzzle::PuzzlePart;
use rayon::prelude::*;

pub struct Puzzle22b {}

impl PuzzlePart for Puzzle22b {
    fn description() -> &'static str {
        "Find the most number of bananas that can be obtained by giving a monkey a sequence of four changes to look for in some hashing prices."
    }

    fn solve(input: &str) -> String {
        let inputs = input
            .lines()
            .map(|line| line.parse().unwrap())
            .collect::<Vec<u64>>();
        let inputs = inputs.iter();

        DiffSeqIterator::new()
            .par_bridge()
            .map(|seq| {
                // println!("{seq:?}");

                inputs
                    .clone()
                    .filter_map(|val| PriceDifferenceIterator::new(*val).price_after_seq(seq))
                    .sum::<u64>()
            })
            .max()
            .unwrap()
            .to_string()
    }
}
