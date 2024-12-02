pub mod puzzle02a;
pub mod puzzle02b;

#[derive(Debug)]
pub struct Report(Vec<i32>);

impl Report {
    pub fn parse_from_str(line: &str) -> Report {
        let vec = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        Report(vec)
    }

    /// A Report is "safe" if:
    /// - The levels are either all increasing or all decreasing.
    /// - Any two adjacent levels differ by at least one and at most three.
    pub fn is_safe(&self) -> bool {
        let diffs = self.0.windows(2).map(|window| window[1] - window[0]);

        diffs.clone().all(|x| (1..=3).contains(&x)) || diffs.clone().all(|x| (-3..=-1).contains(&x))
    }

    /// A Report is "almost safe" if it's either safe (as above)
    /// or would be safe if one level was removed
    pub fn is_almost_safe(&self) -> bool {
        if self.is_safe() {
            return true;
        }

        // no need to do anything fancy here -- try removing each element
        // and see if that creates a safe report
        for index in 0..self.0.len() {
            let modified_vec = [&self.0[0..index], &self.0[index + 1..]].concat();
            if Report(modified_vec).is_safe() {
                return true;
            }
        }

        false
    }
}
