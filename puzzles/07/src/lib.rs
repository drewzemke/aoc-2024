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
}

#[derive(Clone, Debug)]
enum Op {
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

#[derive(Clone, Debug)]
enum OpTree {
    Leaf(i64),
    Node {
        op: Op,
        left: Box<OpTree>,
        right: Box<OpTree>,
    },
}

impl OpTree {
    pub fn eval(&self) -> i64 {
        match self {
            OpTree::Leaf(v) => *v,
            OpTree::Node { op, left, right } => op.eval(left.eval(), right.eval()),
        }
    }

    pub fn all_left_assoc_trees(vals: &[i64], ops: &[Op]) -> Vec<Self> {
        if vals.is_empty() {
            return vec![];
        }

        if vals.len() == 1 {
            return vec![OpTree::Leaf(vals[0])];
        }

        let right = vals.last().unwrap();
        let rest = &vals[..vals.len() - 1];

        let rest_trees = OpTree::all_left_assoc_trees(rest, ops);

        rest_trees
            .iter()
            .flat_map(|rest_tree| {
                ops.iter().map(|op| OpTree::Node {
                    op: op.clone(),
                    left: Box::new(rest_tree.clone()),
                    right: Box::new(OpTree::Leaf(*right)),
                })
            })
            .collect()
    }
}
