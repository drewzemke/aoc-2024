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
            OpTree::Node { op, left, right } => match op {
                Op::Add => left.eval() + right.eval(),
                Op::Mul => left.eval() * right.eval(),
            },
        }
    }

    pub fn all_left_assoc_trees(vals: &[i64]) -> Vec<Self> {
        if vals.is_empty() {
            return vec![];
        }

        if vals.len() == 1 {
            return vec![OpTree::Leaf(vals[0])];
        }

        let right = vals.last().unwrap();
        let rest = &vals[..vals.len() - 1];

        let rest_trees = OpTree::all_left_assoc_trees(rest);

        rest_trees
            .iter()
            .map(|rest_tree| OpTree::Node {
                op: Op::Add,
                left: Box::new(rest_tree.clone()),
                right: Box::new(OpTree::Leaf(*right)),
            })
            .chain(rest_trees.iter().map(|rest_tree| OpTree::Node {
                op: Op::Mul,
                left: Box::new(rest_tree.clone()),
                right: Box::new(OpTree::Leaf(*right)),
            }))
            .collect()
    }
}
