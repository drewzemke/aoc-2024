pub mod puzzle03a;
pub mod puzzle03b;

#[derive(Debug, PartialEq, Eq)]
pub struct Mul(u64, u64);

#[derive(Debug, PartialEq, Eq)]
pub struct NotFound;

/// Extracts a `mul` expression from the start of a string.
fn parse_mul(input: &str) -> Result<(Mul, &str), NotFound> {
    // check prefix
    if !input.starts_with("mul(") {
        return Err(NotFound);
    }

    // parse everything from the '(' to the next comma into an int
    let Some((maybe_int, rest)) = input[4..].split_once(',') else {
        return Err(NotFound);
    };

    let left = maybe_int.parse::<u64>().map_err(|_| NotFound)?;

    // parse everything to the next ')' into an int
    let Some((maybe_int, rest)) = rest.split_once(')') else {
        return Err(NotFound);
    };

    let right = maybe_int.parse::<u64>().map_err(|_| NotFound)?;

    Ok((Mul(left, right), rest))
}

pub fn parse_all_muls(input: &str) -> Vec<Mul> {
    let mut out = vec![];

    let mut remainder = input;
    while !remainder.is_empty() {
        match parse_mul(remainder) {
            Ok((mul, rest)) => {
                out.push(mul);
                remainder = rest;
            }
            Err(_) => {
                // advance by one character
                remainder = &remainder[1..];
            }
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_mul_easy() {
        let result = parse_mul("mul(301,423)nice");
        assert_eq!(result, Ok((Mul(301, 423), "nice")));
    }

    #[test]
    fn parse_failure() {
        let result = parse_mul("mul(301423)nice");
        assert_eq!(result, Err(NotFound));
    }

    #[test]
    fn parse_two_muls() {
        let result = parse_all_muls("mul(mul(301,423)nicemul(1,,333)mul(1,3))");
        assert_eq!(result, vec![Mul(301, 423), Mul(1, 3)]);
    }
}
