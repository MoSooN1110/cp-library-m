// source snippet: key=lib_rolling_hash  prefix=lib_rolling_hash

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Xorshift {
    seed: u64,
}

impl Xorshift {
    #[allow(dead_code)]
    pub fn new() -> Xorshift {
        Xorshift {
            seed: 0xf0fb588ca2196dac,
        }
    }

    #[allow(dead_code)]
    pub fn with_seed(seed: u64) -> Xorshift {
        Xorshift { seed: seed }
    }

    #[inline(always)]
    #[allow(dead_code)]
    pub fn next(&mut self) -> u64 {
        self.seed = self.seed ^ (self.seed << 13);
        self.seed = self.seed ^ (self.seed >> 7);
        self.seed = self.seed ^ (self.seed << 17);
        self.seed
    }

    #[inline(always)]
    #[allow(dead_code)]
    pub fn rand(&mut self, m: u64) -> u64 {
        self.next() % m
    }

    #[inline(always)]
    #[allow(dead_code)]
    pub fn randf(&mut self) -> f64 {
        use std::mem;
        const UPPER_MASK: u64 = 0x3FF0000000000000;
        const LOWER_MASK: u64 = 0xFFFFFFFFFFFFF;
        let tmp = UPPER_MASK | (self.next() & LOWER_MASK);
        let result: f64 = unsafe { mem::transmute(tmp) };
        result - 1.0
    }
}

struct RoLiHa {
    powMemo: Vec<u64>,
    hash: Vec<u64>,
}
const ROLIHA_MASK30: u64 = (1 << 30) - 1;
const ROLIHA_MASK31: u64 = (1 << 31) - 1;
const ROLIHA_MOD: u64 = (1 << 61) - 1;
const ROLIHA_P: u64 = ROLIHA_MOD * ((1 << 3) - 1);
impl RoLiHa {
    #[doc = "caution: the value should not contain 0"]
    fn new(s: &[u64]) -> Self {
        let mut randgen = Xorshift::new();
        let rand = randgen.rand(std::i64::MAX as u64);
        let base = rand + 129;

        let mut powMemo = vec![0; s.len() + 1];
        powMemo[0] = 1;
        for i in 1..powMemo.len() {
            powMemo[i] = Self::calcmod(Self::mul(powMemo[i - 1], base));
        }

        let mut hash = vec![0; s.len() + 1];
        for i in 0..s.len() {
            hash[i + 1] = Self::calcmod(Self::mul(hash[i], base) + s[i]);
        }

        RoLiHa {
            powMemo: powMemo,
            hash: hash,
        }
    }

    // [l,r)
    pub fn get(&self, l: usize, r: usize) -> u64 {
        return Self::calcmod(
            self.hash[r] + ROLIHA_P - Self::mul(self.hash[l], self.powMemo[r - l]),
        );
    }

    pub fn connect(&self, h1: u64, h2: u64, h2len: usize) -> u64 {
        return Self::calcmod(Self::mul(h1, self.powMemo[h2len]) + h2);
    }

    fn mul(l: u64, r: u64) -> u64 {
        let lu = l >> 31;
        let ld = l & ROLIHA_MASK31;
        let ru = r >> 31;
        let rd = r & ROLIHA_MASK31;
        let middle_bit = ld * ru + lu * rd;
        ((lu * ru) << 1) + ld * rd + ((middle_bit & ROLIHA_MASK30) << 31) + (middle_bit >> 30)
    }

    fn calcmod(x: u64) -> u64 {
        let mut x = (x & ROLIHA_MOD) + (x >> 61);
        if x > ROLIHA_MOD {
            x -= ROLIHA_MOD;
        }
        x
    }
}

// #[test]
// fn test_roliha_get() {
//     let seq: Vec<u64> = "abcabc".chars().map(|c| c as u64).collect();
//     let rh = RoLiHa::new(&seq);
//     assert_eq!(rh.get(0, 3), rh.get(3, 6));
//     assert_ne!(rh.get(0, 4), rh.get(3, 6));
//     assert_ne!(rh.get(0, 3), rh.get(2, 5));
// }
