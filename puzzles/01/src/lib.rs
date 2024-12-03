pub mod puzzle01a;
pub mod puzzle01b;

pub fn parse_two_lists(input: &str) -> (Vec<u64>, Vec<u64>) {
    let (mut left, mut right) = (vec![], vec![]);

    input.lines().for_each(|line| {
        let mut nums = line.split_whitespace();
        let left_num = nums.next().and_then(|x| x.parse().ok()).unwrap();
        let right_num = nums.next().and_then(|x| x.parse().ok()).unwrap();
        left.push(left_num);
        right.push(right_num);
    });

    (left, right)
}

pub fn count_occurrences(target: u64, list: &[u64]) -> usize {
    list.iter().filter(|x| **x == target).count()
}
