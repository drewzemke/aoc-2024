pub mod puzzle22a;
pub mod puzzle22b;

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct PriceDifferenceIterator {
    inner: SecretNumberIterator,
    prev_price: u64,
}

impl PriceDifferenceIterator {
    pub fn new(initial: u64) -> Self {
        Self {
            inner: SecretNumberIterator::new(initial),
            prev_price: initial % 10,
        }
    }

    pub fn price_after_seq(&self, seq: [i64; 4]) -> Option<u64> {
        self.clone()
            .take(2000)
            .scan([0; 4], |state, n| {
                state.copy_within(1.., 0);
                state[3] = n;
                Some(*state)
            })
            .skip(3)
            .position(|arr| arr == seq)
            .and_then(|idx| self.inner.clone().nth(idx + 3))
            .map(|n| n % 10)
    }
}

impl Iterator for PriceDifferenceIterator {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        let next_price = self.inner.next()? % 10;

        let diff = next_price as i64 - self.prev_price as i64;
        self.prev_price = next_price;
        Some(diff)
    }
}

#[derive(Debug, Clone)]
pub struct DiffSeqIterator {
    prev: [i64; 4],
}

impl Default for DiffSeqIterator {
    fn default() -> Self {
        Self {
            prev: [-10, -9, -9, -9],
        }
    }
}

impl DiffSeqIterator {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Iterator for DiffSeqIterator {
    type Item = [i64; 4];

    fn next(&mut self) -> Option<Self::Item> {
        if self.prev == [9, 9, 9, 9] {
            return None;
        }

        let mut next = self.prev;
        next[0] += 1;
        let mut idx = 0;
        while next[idx] == 10 {
            next[idx] = -9;
            idx += 1;
            next[idx] += 1;
        }

        self.prev = next;
        Some(next)
    }
}
