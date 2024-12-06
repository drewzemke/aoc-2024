use std::cmp::Ordering;

pub mod puzzle05a;
pub mod puzzle05b;

#[derive(Debug)]
pub struct OrderedPair(u64, u64);

#[derive(Debug)]
pub struct Update(Vec<u64>);

impl Update {
    pub fn middle(&self) -> u64 {
        // length is always odd
        self.0[(self.0.len() - 1) / 2]
    }

    pub fn is_sorted_by(&self, pairs: &[OrderedPair]) -> bool {
        self.0
            .windows(2)
            .all(|w| pairs.iter().any(|pair| pair.0 == w[0] && pair.1 == w[1]))
    }

    pub fn sort_by(&mut self, pairs: &[OrderedPair]) {
        self.0.sort_unstable_by(|x, y| {
            if x == y {
                Ordering::Equal
            } else if pairs.iter().any(|pair| pair.0 == *x && pair.1 == *y) {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        })
    }
}

pub fn parse_input(input: &str) -> (Vec<OrderedPair>, Vec<Update>) {
    let Some((pair_str, updates_str)) = input.split_once("\n\n") else {
        return (vec![], vec![]);
    };

    let pairs = pair_str
        .lines()
        .map(|line| {
            let (left, right) = line.split_once('|').unwrap();
            let left = left.parse().unwrap();
            let right = right.parse().unwrap();
            OrderedPair(left, right)
        })
        .collect();

    let updates = updates_str
        .lines()
        .map(|line| {
            line.split(',')
                .map(str::parse)
                .map(Result::unwrap)
                .collect()
        })
        .map(Update)
        .collect();

    (pairs, updates)
}
