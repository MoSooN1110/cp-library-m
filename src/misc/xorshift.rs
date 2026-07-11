//! 軽量擬似乱数（xorshift64）。決定的でシード再現可能。
//!
//! ```
//! use cplib::misc::xorshift::*;
//! let mut rng = XorShift::new(12345);
//! let a = rng.next_range(100);
//! assert!(a < 100);
//! ```

pub struct XorShift {
    s: u64,
}

impl XorShift {
    pub fn new(seed: u64) -> Self {
        XorShift {
            s: if seed == 0 { 0x9e3779b97f4a7c15 } else { seed },
        }
    }
    #[inline]
    pub fn next_u64(&mut self) -> u64 {
        let mut x = self.s;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.s = x;
        x
    }
    /// [0, n) の一様乱数
    #[inline]
    pub fn next_range(&mut self, n: u64) -> u64 {
        self.next_u64() % n
    }
    /// [0, 1) の乱数
    #[inline]
    pub fn next_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }
    /// スライスをシャッフル（Fisher-Yates）
    pub fn shuffle<T>(&mut self, a: &mut [T]) {
        for i in (1..a.len()).rev() {
            let j = self.next_range(i as u64 + 1) as usize;
            a.swap(i, j);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn deterministic_and_range() {
        let mut r1 = XorShift::new(42);
        let mut r2 = XorShift::new(42);
        for _ in 0..100 {
            assert_eq!(r1.next_u64(), r2.next_u64());
        }
        let mut r = XorShift::new(7);
        for _ in 0..1000 {
            assert!(r.next_range(10) < 10);
            let f = r.next_f64();
            assert!((0.0..1.0).contains(&f));
        }
        // shuffle は要素を保存
        let mut v: Vec<i32> = (0..20).collect();
        r.shuffle(&mut v);
        v.sort();
        assert_eq!(v, (0..20).collect::<Vec<_>>());
    }
}
