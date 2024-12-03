pub mod puzzle03a;
pub mod puzzle03b;

#[derive(Debug, PartialEq, Eq)]
pub struct Mul(u64, u64);

#[derive(Debug, PartialEq, Eq)]
pub struct NotFound;

#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    Mul(Mul),
    Do,
    Dont,
}

/// Extracts a `mul` expression from the start of a string.
fn parse_mul(input: &str) -> Result<(Mul, &str), NotFound> {
    let stripped = input.strip_prefix("mul(").ok_or(NotFound)?;
    let (left_str, rest) = stripped.split_once(',').ok_or(NotFound)?;
    let (right_str, rest) = rest.split_once(')').ok_or(NotFound)?;

    let left = left_str.parse().map_err(|_| NotFound)?;
    let right = right_str.parse().map_err(|_| NotFound)?;

    Ok((Mul(left, right), rest))
}

pub fn parse_all_muls(input: &str) -> Vec<Mul> {
    let mut out = vec![];

    let mut remainder = input;
    while !remainder.is_empty() {
        if let Ok((mul, rest)) = parse_mul(remainder) {
            out.push(mul);
            remainder = rest;
        } else {
            // advance by one character
            remainder = &remainder[1..];
        }
    }

    out
}

fn parse_do(input: &str) -> Result<((), &str), NotFound> {
    if let Some(rest) = input.strip_prefix("do") {
        Ok(((), rest))
    } else {
        Err(NotFound)
    }
}

fn parse_dont(input: &str) -> Result<((), &str), NotFound> {
    if let Some(rest) = input.strip_prefix("don't") {
        Ok(((), rest))
    } else {
        Err(NotFound)
    }
}

pub fn parse_instructions(input: &str) -> Vec<Instruction> {
    let mut out = vec![];

    let mut remainder = input;
    while !remainder.is_empty() {
        // IMPORTANT: check for "don't" first because "do" is a prefix of it
        if let Ok((_, rest)) = parse_dont(remainder) {
            out.push(Instruction::Dont);
            remainder = rest;
        } else if let Ok((_, rest)) = parse_do(remainder) {
            out.push(Instruction::Do);
            remainder = rest;
        } else if let Ok((mul, rest)) = parse_mul(remainder) {
            out.push(Instruction::Mul(mul));
            remainder = rest;
        } else {
            // advance by one character
            remainder = &remainder[1..];
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_mul() {
        let result = parse_mul("mul(301,423)nice");
        assert_eq!(result, Ok((Mul(301, 423), "nice")));
    }

    #[test]
    fn test_parse_failure() {
        let result = parse_mul("mul(301423)nice");
        assert_eq!(result, Err(NotFound));
    }

    #[test]
    fn test_parse_two_muls() {
        let result = parse_all_muls("mul(mul(301,423)nicemul(1,,333)mul(1,3))");
        assert_eq!(result, vec![Mul(301, 423), Mul(1, 3)]);
    }

    #[test]
    fn test_parse_instructions() {
        let result = parse_instructions("mul(domul(301,423)nidon'tcemul(1,,333)domul(1,3))");
        assert_eq!(
            result,
            vec![
                Instruction::Do,
                Instruction::Mul(Mul(301, 423)),
                Instruction::Dont,
                Instruction::Do,
                Instruction::Mul(Mul(1, 3))
            ]
        );
    }
}
