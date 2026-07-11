//! 中央値集合（2 ヒープ）。追加と、中央値・中央値までの絶対偏差和を取得。
//!
//! ```
//! use cplib::ds::median_set::*;
//! let mut s = MedianSet::new();
//! for x in [1, 5, 2, 8, 7] { s.add(x); }
//! assert_eq!(s.median(), 5);
//! // |1-5|+|5-5|+|2-5|+|8-5|+|7-5| = 4+0+3+3+2 = 12
//! assert_eq!(s.abs_deviation(), 12);
//! ```
use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct MedianSet {
    lo: BinaryHeap<i64>,          // 下半分（最大ヒープ）
    hi: BinaryHeap<Reverse<i64>>, // 上半分（最小ヒープ）
    lo_sum: i64,
    hi_sum: i64,
}

impl Default for MedianSet {
    fn default() -> Self {
        Self::new()
    }
}

impl MedianSet {
    pub fn new() -> Self {
        MedianSet {
            lo: BinaryHeap::new(),
            hi: BinaryHeap::new(),
            lo_sum: 0,
            hi_sum: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.lo.len() + self.hi.len()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, x: i64) {
        if self.lo.is_empty() || x <= *self.lo.peek().unwrap() {
            self.lo.push(x);
            self.lo_sum += x;
        } else {
            self.hi.push(Reverse(x));
            self.hi_sum += x;
        }
        // lo.len() == hi.len() または lo.len() == hi.len()+1 に保つ
        if self.lo.len() > self.hi.len() + 1 {
            let t = self.lo.pop().unwrap();
            self.lo_sum -= t;
            self.hi.push(Reverse(t));
            self.hi_sum += t;
        } else if self.hi.len() > self.lo.len() {
            let Reverse(t) = self.hi.pop().unwrap();
            self.hi_sum -= t;
            self.lo.push(t);
            self.lo_sum += t;
        }
    }

    /// 下側中央値（要素数が偶数のとき小さい方）。
    pub fn median(&self) -> i64 {
        *self.lo.peek().expect("empty")
    }

    /// sum_i |a_i - median|。
    pub fn abs_deviation(&self) -> i64 {
        let m = self.median();
        (m * self.lo.len() as i64 - self.lo_sum) + (self.hi_sum - m * self.hi.len() as i64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn matches_brute() {
        let mut x: u64 = 2024;
        let mut rng = || {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            x
        };
        for _ in 0..200 {
            let n = 1 + (rng() as usize) % 30;
            let mut vals = vec![];
            let mut s = MedianSet::new();
            for _ in 0..n {
                let v = (rng() % 100) as i64;
                vals.push(v);
                s.add(v);
            }
            vals.sort();
            let med = vals[(vals.len() - 1) / 2];
            assert_eq!(s.median(), med);
            let dev: i64 = vals.iter().map(|&v| (v - med).abs()).sum();
            assert_eq!(s.abs_deviation(), dev);
        }
    }
}
