//! Fenwick Tree (BIT)。点加算・区間和。0-indexed。
//!
//! ```
//! use cplib::ds::fenwick::*;
//! let mut bit = Fenwick::<i64>::new(5);
//! bit.add(0, 3);
//! bit.add(2, 5);
//! assert_eq!(bit.sum(0..3), 8);   // [0,3)
//! ```
use std::ops::{Add, AddAssign, Range, Sub};

pub struct Fenwick<T> {
    n: usize,
    d: Vec<T>,
}

impl<T> Fenwick<T>
where
    T: Copy + Default + AddAssign + Add<Output = T> + Sub<Output = T>,
{
    pub fn new(n: usize) -> Self {
        Fenwick {
            n,
            d: vec![T::default(); n + 1],
        }
    }
    /// a[p] += x
    pub fn add(&mut self, p: usize, x: T) {
        assert!(p < self.n);
        let mut i = p + 1;
        while i <= self.n {
            self.d[i] += x;
            i += i & i.wrapping_neg();
        }
    }
    /// [0, r) の総和
    fn prefix(&self, r: usize) -> T {
        let mut s = T::default();
        let mut i = r;
        while i > 0 {
            s += self.d[i];
            i -= i & i.wrapping_neg();
        }
        s
    }
    /// 半開区間 range の総和
    pub fn sum(&self, range: Range<usize>) -> T {
        assert!(range.end <= self.n);
        self.prefix(range.end) - self.prefix(range.start)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic() {
        let mut bit = Fenwick::<i64>::new(6);
        for i in 0..6 {
            bit.add(i, (i as i64) + 1); // 1..=6
        }
        assert_eq!(bit.sum(0..6), 21);
        assert_eq!(bit.sum(2..5), 3 + 4 + 5);
        bit.add(3, 10);
        assert_eq!(bit.sum(0..4), 1 + 2 + 3 + 14);
    }
}
