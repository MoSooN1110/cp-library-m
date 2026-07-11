//! 1 次元累積和（区間和 O(1)）。ジェネリック。
//!
//! ```
//! use cplib::ds::cumsum::*;
//! let cs = CumSum::new(&[1i64, 2, 3, 4, 5]);
//! assert_eq!(cs.sum(1..4), 9);   // 2+3+4
//! assert_eq!(cs.sum(0..5), 15);
//! ```
use std::ops::{Add, Range, Sub};

pub struct CumSum<T> {
    s: Vec<T>,
}

impl<T> CumSum<T>
where
    T: Copy + Default + Add<Output = T> + Sub<Output = T>,
{
    pub fn new(a: &[T]) -> Self {
        let mut s = vec![T::default(); a.len() + 1];
        for i in 0..a.len() {
            s[i + 1] = s[i] + a[i];
        }
        CumSum { s }
    }
    /// 半開区間 [l, r) の和
    pub fn sum(&self, range: Range<usize>) -> T {
        self.s[range.end] - self.s[range.start]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic() {
        let cs = CumSum::new(&[3i64, 1, 4, 1, 5, 9]);
        assert_eq!(cs.sum(0..6), 23);
        assert_eq!(cs.sum(2..5), 10);
        assert_eq!(cs.sum(3..3), 0);
    }
}
