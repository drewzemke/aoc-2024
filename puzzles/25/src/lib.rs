pub mod puzzle25a;
pub mod puzzle25b;

#[derive(Debug, Clone)]
pub struct Key([u8; 5]);

impl Key {
    fn parse(input: &str) -> Self {
        let mut vals = [0; 5];
        input.lines().skip(1).take(5).for_each(|line| {
            line.chars().enumerate().for_each(|(idx, c)| {
                if c == '#' {
                    vals[idx] += 1;
                }
            })
        });

        Self(vals)
    }

    pub fn loosely_fits(&self, lock: &Lock) -> bool {
        for idx in 0..5 {
            if self.0[idx] + lock.0[idx] > 5 {
                return false;
            }
        }

        true
    }
}

#[derive(Debug, Clone)]
pub struct Lock([u8; 5]);

impl Lock {
    fn parse(input: &str) -> Self {
        let mut vals = [0; 5];
        input.lines().skip(1).take(5).for_each(|line| {
            line.chars().enumerate().for_each(|(idx, c)| {
                if c == '#' {
                    vals[idx] += 1;
                }
            })
        });

        Self(vals)
    }
}

pub fn parse_locks_and_keys(input: &str) -> (Vec<Lock>, Vec<Key>) {
    let mut locks = vec![];
    let mut keys = vec![];

    input.split("\n\n").for_each(|chunk| {
        // determine if this is a lock or a key by looking at the first
        // character, since the first row of a lock is ##### and
        // the first row of a key is .....
        if chunk.chars().next().is_some_and(|c| c == '#') {
            locks.push(Lock::parse(chunk));
        } else {
            keys.push(Key::parse(chunk));
        }
    });

    (locks, keys)
}
