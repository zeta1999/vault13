use log::*;
use std::cmp;

/// Pseudo-random generator based on Minimal Standard by Lewis, Goodman, and Miller in 1969.
pub struct Random {
    seed: i32,
    y: i32,
    table: [i32; 32],
}

impl Random {
    pub fn new() -> Self {
        Self::with_seed(rand::random())
    }

    // generate_seed()
    pub fn with_seed(seed: u32) -> Self {
        let mut table = [0; 32];
        let mut seed = cmp::max(seed as i32, 1);
        for i in (0..=39).rev() {
            seed = 16807 * (seed % 127773) - 2836 * (seed / 127773);
            if seed < 0 {
                seed += i32::max_value();
            }
            if i < 32 {
                table[i] = seed;
            }
        }
        Self {
            seed,
            y: table[0],
            table,
        }
    }

    // roll_random()
    pub fn gen(&mut self, from_inclusive: i32, to_inclusive: i32) -> i32 {
        let r = if from_inclusive <= to_inclusive {
            from_inclusive + self.gen0(to_inclusive + 1 - from_inclusive)
        } else {
            to_inclusive + self.gen0(from_inclusive + 1 - to_inclusive)
        };

        if r >= from_inclusive && r <= to_inclusive {
            r
        } else {
            warn!("generated random {} is not in bounds [{}..{}]", r, from_inclusive, to_inclusive);
            from_inclusive
        }
    }

    // ran1()
    fn gen0(&mut self, upper_bound: i32) -> i32 {
        let mut next_seed = 16807 * (self.seed % 127773) - 2836 * (self.seed / 127773);
        if next_seed < 0 {
            next_seed += 0x7FFFFFFF;
        }
        let i = (self.y % self.table.len() as i32) as usize;
        let next_y = self.table[i];
        self.table[i] = next_seed;
        self.y = next_y;
        self.seed = next_seed;
        next_y % upper_bound
    }
}

pub fn check_chi_square() {
    const ITER_COUNT: usize = 100_000;
    const RANGE: usize = 25;
    const MAX: f64 = 36.42;
    const EXPECTED: f64 = 4000.0;

    let mut freqs = [0i32; RANGE];

    let mut rand = Random::new();
    for _ in 0..ITER_COUNT {
        let i = rand.gen(1, RANGE as i32) - 1;
        assert!(i >= 0 && i < RANGE as i32);
        freqs[i as usize] += 1;
    }

    let actual: f64 = freqs.iter()
        .map(|&f| (f as f64 - EXPECTED) * (f as f64 - EXPECTED) / EXPECTED)
        .sum();
    if actual <= MAX {
        debug!("RNG is good: {} <= {}", actual, MAX);
    } else {
        warn!("RNG is bad: {} > {}", actual, MAX);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new() {
        let r = Random::with_seed(0x031006DE);
        assert_eq!(r.seed, r.table[0]);
        assert_eq!(r.y, r.table[0]);
        assert_eq!(r.table[0], 0x668007);
        assert_eq!(r.table[11], 0x3B9CFF87);
        assert_eq!(r.table[20], 0x528FF6CE);
        assert_eq!(r.table[31], 0x59432445);
    }

    #[test]
    fn gen() {
        let mut r = Random::with_seed(0x031006DE);
        assert_eq!(r.gen(1, 25), 22);
        assert_eq!(r.gen(1, 25), 19);
    }
}