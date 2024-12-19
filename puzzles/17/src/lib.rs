pub mod puzzle17a;
pub mod puzzle17b;

const DEBUG: bool = false;

fn debug(s: &str) {
    if DEBUG {
        println!("{s}");
    }
}

#[derive(Clone, Debug)]
pub struct Computer {
    reg_a: i64,
    reg_b: i64,
    reg_c: i64,
    ptr: usize,
    program: Vec<u8>,
    pub output: Vec<u8>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Instruction {
    /// The adv instruction (opcode 0) performs division. The
    /// numerator is the value in the A register. The denominator
    /// is found by raising 2 to the power of the instruction's
    /// combo operand. (So, an operand of 2 would divide A by 4
    /// (2^2); an operand of 5 would divide A by 2^B.) The result of
    /// the division operation is truncated to an integer and then
    /// written to the A register.
    Adv,

    /// The bxl instruction (opcode 1) calculates the bitwise XOR
    /// of register B and the instruction's literal operand, then
    /// stores the result in register B.
    Bxl,

    /// The bst instruction (opcode 2) calculates the value of its
    /// combo operand modulo 8 (thereby keeping only its lowest 3
    /// bits), then writes that value to the B register.
    Bst,

    /// The jnz instruction (opcode 3) does nothing if the A
    /// register is 0. However, if the A register is not zero, it
    /// jumps by setting the instruction pointer to the value of its
    /// literal operand; if this instruction jumps, the instruction
    /// pointer is not increased by 2 after this instruction.
    Jnz,

    /// The bxc instruction (opcode 4) calculates the bitwise XOR
    /// of register B and register C, then stores the result in
    /// register B. (For legacy reasons, this instruction reads an
    /// operand but ignores it.)
    Bxc,

    /// The out instruction (opcode 5) calculates the value of
    /// its combo operand modulo 8, then outputs that value. (If
    /// a program outputs multiple values, they are separated by
    /// commas.)
    Out,

    /// The bdv instruction (opcode 6) works exactly like the
    /// adv instruction except that the result is stored in the B
    /// register. (The numerator is still read from the A register.)
    Bdv,

    /// The cdv instruction (opcode 7) works exactly like the
    /// adv instruction except that the result is stored in the C
    /// register. (The numerator is still read from the A register.)
    /// Adv,
    Cdv,
}

impl From<u8> for Instruction {
    fn from(v: u8) -> Self {
        match v {
            0 => Self::Adv,
            1 => Self::Bxl,
            2 => Self::Bst,
            3 => Self::Jnz,
            4 => Self::Bxc,
            5 => Self::Out,
            6 => Self::Bdv,
            7 => Self::Cdv,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Operand {
    Literal(u8),
    Combo(u8),
}

impl Computer {
    pub fn parse(input: &str) -> Option<Self> {
        let mut lines = input.lines();

        let reg_a = lines.next()?.strip_prefix("Register A: ")?.parse().ok()?;
        let reg_b = lines.next()?.strip_prefix("Register B: ")?.parse().ok()?;
        let reg_c = lines.next()?.strip_prefix("Register C: ")?.parse().ok()?;

        let program = lines
            .nth(1)?
            .strip_prefix("Program: ")?
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();

        Some(Self {
            reg_a,
            reg_b,
            reg_c,
            program,
            ptr: 0,
            output: Vec::new(),
        })
    }

    fn op_value(&self, op: Operand) -> i64 {
        match op {
            Operand::Literal(n) => n as i64,
            Operand::Combo(n) if (0..=3).contains(&n) => n as i64,
            Operand::Combo(4) => self.reg_a,
            Operand::Combo(5) => self.reg_b,
            Operand::Combo(6) => self.reg_c,
            _ => unreachable!(),
        }
    }

    fn run(&mut self) {
        while self.ptr < self.program.len() {
            let instruction = self.program[self.ptr].into();
            let input = self.program[self.ptr + 1];
            self.apply(instruction, input);
            if instruction != Instruction::Jnz {
                self.ptr += 2;
            }
        }
    }

    fn apply(&mut self, instruction: Instruction, input: u8) {
        debug(&format!("{:?}", &self));
        debug(&format!("Applying {instruction:?} with input {input}"));

        match instruction {
            Instruction::Adv => {
                let num = self.reg_a;
                let denom = 2i64.pow(self.op_value(Operand::Combo(input)) as u32);
                self.reg_a = num / denom;
                debug(&format!("Set A to {num}/{denom} = {}", self.reg_a));
            }

            Instruction::Bxl => {
                let left = self.reg_b;
                let right = self.op_value(Operand::Literal(input));
                self.reg_b = left ^ right;
                debug(&format!("Set B to {left} ^ {right} = {}", self.reg_b));
            }

            Instruction::Bst => {
                let op_value = self.op_value(Operand::Combo(input));
                self.reg_b = op_value % 8;
                debug(&format!("Set B to {op_value} %8 = {}", self.reg_b));
            }

            Instruction::Jnz => {
                if self.reg_a != 0 {
                    self.ptr = self.op_value(Operand::Literal(input)) as usize;
                    debug(&format!("Jumped to {}", self.ptr));
                } else {
                    self.ptr += 2;
                }
            }

            Instruction::Bxc => {
                let left = self.reg_b;
                let right = self.reg_c;
                self.reg_b = left ^ right;
                debug(&format!("Set B to {left} ^ {right} = {}", self.reg_b));
            }

            Instruction::Out => {
                let out = self.op_value(Operand::Combo(input)) % 8;
                self.output.push(out as u8);
                debug(&format!("Output {out}"));
            }

            Instruction::Bdv => {
                let num = self.reg_a;
                let denom = 2i64.pow(self.op_value(Operand::Combo(input)) as u32);
                self.reg_b = num / denom;
                debug(&format!("Set B to {num}/{denom} = {}", self.reg_b));
            }

            Instruction::Cdv => {
                let num = self.reg_a;
                let denom = 2i64.pow(self.op_value(Operand::Combo(input)) as u32);
                self.reg_c = num / denom;
                debug(&format!("Set C to {num}/{denom} = {}", self.reg_c));
            }
        }

        debug("-------------");
    }

    /// Runs the specific program given by:
    ///   2,4,1,3,7,5,0,3,1,5,4,1,5,5,3,0
    /// and checks if the output is equal to the program itself
    ///
    /// NOTE: this program follows a compute/output/jump loop that is calculable
    /// from the initial A register only; the logic here is very specific to the
    /// input puzzle
    pub fn has_output(&self, target: &[u8]) -> bool {
        let mut a = self.reg_a;
        let mut idx = 0;
        let mut output = vec![];

        loop {
            if a == 0 {
                break;
            }

            // from analyzing the program, we can directly compute the output at the end of
            // one loop iteration. it's:
            //   ( (a % 8) △ 6 △ (a / 2 ^ (3 △ (a % 8))) ) % 8
            let a_mod_8 = a % 8;
            let out = (a_mod_8 ^ 6 ^ (a / 2i64.pow(3 ^ a_mod_8 as u32))) % 8;

            // check if this output matches the program at the current index
            if target[idx] as i64 != out {
                return false;
            }
            output.push(out as u8);
            idx += 1;

            // during the loop, a is replaced by a/8;
            a /= 8;
        }

        // if we got here, and the next index would be just off the end of the program, we win!
        idx == target.len()
    }
}
