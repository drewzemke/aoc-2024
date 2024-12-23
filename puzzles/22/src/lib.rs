pub mod puzzle22a;
pub mod puzzle22b;

pub struct SecretNumberIterator {
    prev: u64,
}

impl SecretNumberIterator {
    pub fn new(initial: u64) -> Self {
        Self { prev: initial }
    }
}

impl Iterator for SecretNumberIterator {
    type Item = u64;

    /// To generate the next number
    /// 1. Calculate the result of multiplying the secret number by 64.
    ///    Then, mix this result into the secret number. Finally, prune
    ///    the secret number.
    /// 2. Calculate the result of dividing the secret number by 32.
    ///    Round the result down to the nearest integer. Then, mix this
    ///    result into the secret number. Finally, prune the secret
    ///    number.
    /// 3. Calculate the result of multiplying the secret number by
    ///    2048. Then, mix this result into the secret number. Finally,
    ///    prune the secret number.
    ///
    /// Each step of the above process involves mixing and pruning:
    /// - To mix a value into the secret number, calculate the bitwise
    ///   XOR of the given value and the secret number. Then, the
    ///   secret number becomes the result of that operation.
    /// - To prune the secret number, calculate the value of the
    ///   secret number modulo 16777216. Then, the secret number
    ///   becomes the result of that operation.
    fn next(&mut self) -> Option<Self::Item> {
        let mut s = self.prev;

        // step 1
        let s1 = 64 * s;
        s ^= s1;
        s %= 16777216;

        // step 2
        let s2 = s / 32;
        s ^= s2;
        s %= 16777216;

        // step 3
        let s3 = 2048 * s;
        s ^= s3;
        s %= 16777216;

        self.prev = s;
        Some(s)
    }
}
