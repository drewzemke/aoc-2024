use std::collections::HashMap;

pub mod puzzle11a;
pub mod puzzle11b;

pub fn parse(input: &str) -> Vec<u64> {
    input
        .lines()
        .next()
        .unwrap()
        .split(' ')
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

/// start from one number and generate descendents as follows (whichever applies first)
/// - 0 -> 1
/// - even number of digits -> split into two numbers (each with half the digits)
/// - n -> 2024 * n
pub fn count_descs(num: u64, steps: u64) -> u64 {
    count_descs_memo(num, steps, &mut HashMap::new())
}

/// memoized helper function
fn count_descs_memo(num: u64, steps: u64, memo: &mut HashMap<(u64, u64), u64>) -> u64 {
    if let Some(v) = memo.get(&(num, steps)) {
        return *v;
    }

    let result = if steps == 0 {
        1
    } else if num == 0 {
        count_descs_memo(1, steps - 1, memo)
    } else {
        match split_digits(num) {
            Some((left, right)) => {
                count_descs_memo(left, steps - 1, memo) + count_descs_memo(right, steps - 1, memo)
            }
            None => count_descs_memo(2024 * num, steps - 1, memo),
        }
    };

    memo.insert((num, steps), result);

    result
}

/// if `num` has evenly-many digits, this returns two numbers each with half of the
/// digits. otherwise returns none
fn split_digits(num: u64) -> Option<(u64, u64)> {
    let s = num.to_string();
    if s.len() % 2 != 0 {
        return None;
    }

    let mid = s.len() / 2;
    let left = s[0..mid].parse().ok()?;
    let right = s[mid..].parse().ok()?;

    Some((left, right))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_split_even_length_nums() {
        assert_eq!(split_digits(1234), Some((12, 34)));
        assert_eq!(split_digits(1200), Some((12, 0)));
    }

    #[test]
    fn should_not_split_odd_length_nums() {
        assert_eq!(split_digits(1), None);
        assert_eq!(split_digits(123), None);
    }
}
