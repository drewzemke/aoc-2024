use std::collections::VecDeque;

pub mod puzzle24a;
pub mod puzzle24b;

#[derive(Debug, Clone)]
pub struct Wire<'a> {
    name: &'a str,
    value: Option<bool>,
}

#[derive(Debug, Clone)]
pub enum Op {
    And,
    Or,
    Xor,
}

impl TryFrom<&str> for Op {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "AND" => Ok(Self::And),
            "OR" => Ok(Self::Or),
            "XOR" => Ok(Self::Xor),
            _ => Err(String::from("uh oh!")),
        }
    }
}

impl Op {
    pub fn eval(&self, in1: bool, in2: bool) -> bool {
        match self {
            Op::And => in1 && in2,
            Op::Or => in1 || in2,
            Op::Xor => in1 ^ in2,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Input {
    output: usize,
}

#[derive(Debug, Clone)]
pub struct Gate {
    op: Op,
    input: (usize, usize),
    output: usize,
}

#[derive(Debug)]
pub struct Device<'a> {
    inputs: Vec<Input>,
    gates: Vec<Gate>,
    wires: Vec<Wire<'a>>,
}

impl<'a> Device<'a> {
    pub fn parse(input: &'a str) -> Option<Self> {
        // two chunks: first inputs, then gates
        let (inputs_str, gates_str) = input.split_once("\n\n")?;

        let pre_inputs = inputs_str
            .lines()
            .map(|line| {
                let (name, val_str) = line.split_once(": ")?;
                Some((name, val_str == "1"))
            })
            .collect::<Option<Vec<_>>>()?;

        let pre_gates = gates_str
            .lines()
            .map(|line| {
                let (in_wire1, rest) = line.split_once(' ')?;
                let (op_str, rest) = rest.split_once(' ')?;
                let (in_wire2, out_wire) = rest.split_once(" -> ")?;
                let op = Op::try_from(op_str).ok()?;

                Some((in_wire1, op, in_wire2, out_wire))
            })
            .collect::<Option<Vec<_>>>()?;

        // start list of wires from inputs
        let mut wires = pre_inputs
            .iter()
            .map(|(name, value)| Wire {
                name,
                value: Some(*value),
            })
            .collect::<Vec<_>>();

        // add any wires mentioned in gate data that we don't already have
        for (wire1, _, wire2, wire3) in &pre_gates {
            for name in [*wire1, *wire2, *wire3] {
                if !wires.iter().any(|w| w.name == name) {
                    wires.push(Wire { name, value: None })
                }
            }
        }

        // create inputs
        let inputs = pre_inputs
            .into_iter()
            .map(|(name, _)| {
                let wire_idx = wires.iter().position(|w| w.name == name)?;
                Some(Input { output: wire_idx })
            })
            .collect::<Option<Vec<_>>>()?;

        // create gates
        let gates = pre_gates
            .into_iter()
            .map(|(in_wire1, op, in_wire2, out_wire)| {
                let in_idx1 = wires.iter().position(|w| w.name == in_wire1)?;
                let in_idx2 = wires.iter().position(|w| w.name == in_wire2)?;
                let out_idx = wires.iter().position(|w| w.name == out_wire)?;
                Some(Gate {
                    op,
                    input: (in_idx1, in_idx2),
                    output: out_idx,
                })
            })
            .collect::<Option<Vec<_>>>()?;

        Some(Self {
            inputs,
            gates,
            wires,
        })
    }

    pub fn compute(&mut self) {
        // keep track of a list of which wires (by index in `self.wires`) have values.
        // to start, those are exactly the indices that correspond to inputs
        let mut powered_wires = self.inputs.iter().map(|i| i.output).collect::<Vec<_>>();

        // also keep track of a deque of gates that we can compute values for.
        // to start, it's the set of gates whose two inputs are powered.
        let (mut gates_to_check, mut unchecked_gates): (VecDeque<&Gate>, VecDeque<&Gate>) =
            self.gates.iter().partition(|g| {
                powered_wires.contains(&g.input.0) && powered_wires.contains(&g.input.1)
            });

        while let Some(gate) = gates_to_check.pop_front() {
            // compute the value of the output, then set that wire
            let input1 = self.wires[gate.input.0].value.unwrap();
            let input2 = self.wires[gate.input.1].value.unwrap();
            let output = gate.op.eval(input1, input2);
            self.wires[gate.output].value = Some(output);
            powered_wires.push(gate.output);

            // look for any unchecked gates that become powered with this new additon
            let mut gates_to_add;
            (gates_to_add, unchecked_gates) = unchecked_gates.iter().partition(|g| {
                g.input.0 == gate.output && powered_wires.contains(&g.input.1)
                    || g.input.1 == gate.output && powered_wires.contains(&g.input.0)
            });
            gates_to_check.append(&mut gates_to_add);
        }
    }

    pub fn output(&self) -> usize {
        // get all of the wires whose names start with 'z', sort them by name,
        // then parse their as a binary number
        let mut z_wires = self
            .wires
            .iter()
            .filter(|w| w.name.starts_with('z'))
            .collect::<Vec<_>>();

        z_wires.sort_by_cached_key(|w| w.name);

        z_wires
            .iter()
            .enumerate()
            .map(|(idx, w)| {
                if w.value.is_some_and(|v| v) {
                    2_usize.pow(idx as u32)
                } else {
                    0
                }
            })
            .sum()
    }
}
