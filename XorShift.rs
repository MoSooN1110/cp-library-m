// source snippet: key=XorShift  prefix=lib_XorShift

#[derive(Debug)]
#[allow(dead_code)]
pub struct Xorshift {
    seed: u64,
}
impl Default for Xorshift {
    fn default() -> Self {
        Xorshift {
            seed: 0xf0fb_588c_a219_6dac,
        }
    }
}
impl Xorshift {
    #[allow(dead_code)]
    pub fn with_seed(seed: u64) -> Xorshift {
        Xorshift { seed }
    }
    #[inline(always)]
    #[allow(dead_code)]
    pub fn next_u64(&mut self) -> u64 {
        self.seed = self.seed ^ (self.seed << 13);
        self.seed = self.seed ^ (self.seed >> 7);
        self.seed = self.seed ^ (self.seed << 17);
        self.seed
    }
    #[inline(always)]
    #[allow(dead_code)]
    pub fn rand(&mut self, m: u64) -> u64 {
        self.next_u64() % m
    }
    #[inline(always)]
    #[allow(dead_code)]
    pub fn randf(&mut self) -> f64 {
        const UPPER_MASK: u64 = 0x3FF0_0000_0000_0000;
        const LOWER_MASK: u64 = 0xF_FFFF_FFFF_FFFF;
        let tmp = UPPER_MASK | (self.next_u64() & LOWER_MASK);
        let result: f64 = f64::from_bits(tmp);
        result - 1.0
    }
}
