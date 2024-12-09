pub mod puzzle07a;
pub mod puzzle07b;

#[derive(Debug)]
pub struct Equation {
    lhs: i64,
    rhs: Vec<i64>,
}

impl Equation {
    pub fn parse(input: &str) -> Self {
        let (left, right) = input.split_once(": ").unwrap();

        let lhs = left.parse().unwrap();

        let rhs = right
            .split(' ')
            .map(str::parse)
            .map(Result::unwrap)
            .collect();

        Equation { lhs, rhs }
    }

    fn apply_ops(&self, ops: &[Op]) -> i64 {
        let mut val = self.rhs[0];

        for (idx, op) in ops.iter().enumerate() {
            val = op.eval(val, self.rhs[idx + 1]);
        }

        val
    }

    pub fn is_equalable(&self, ops: &[Op]) -> bool {
        for ops in Op::all_combos(ops, self.rhs.len() - 1) {
            if self.lhs == self.apply_ops(&ops) {
                return true;
            }
        }

        false
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Op {
    Add,
    Mul,
    Concat,
}

impl Op {
    pub fn eval(&self, left: i64, right: i64) -> i64 {
        match self {
            Op::Add => left + right,
            Op::Mul => left * right,
            Op::Concat => left * (10_u32.pow(right.ilog10() + 1) as i64) + right,
        }
    }

    pub fn all_combos(ops: &[Self], size: usize) -> Vec<Vec<Self>> {
        if size == 0 {
            return vec![vec![]];
        }

        let rest = Self::all_combos(ops, size - 1);

        rest.iter()
            .flat_map(|list| {
                ops.iter().map(|op| {
                    let mut list = list.clone();
                    list.push(*op);
                    list
                })
            })
            .collect()
    }
}

#[cfg(test)]
mod op_tests {
    use super::*;

    #[test]
    fn should_concat() {
        assert_eq!(Op::Concat.eval(12, 345), 12345);
        assert_eq!(Op::Concat.eval(345, 12), 34512);
        assert_eq!(Op::Concat.eval(10, 1000), 101000);
        assert_eq!(Op::Concat.eval(1000, 1), 10001);
    }
}
