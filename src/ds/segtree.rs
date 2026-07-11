//! Segment Tree（モノイド、点更新・区間積）。op/e は関数ポインタで与える。
//!
//! ```
//! use cplib::ds::segtree::*;
//! // 区間最大
//! let mut seg = SegTree::from_slice(&[1i64, 3, 2, 5, 4], i64::MIN, |a, b| a.max(b));
//! assert_eq!(seg.prod(1..4), 5);
//! seg.set(3, 0);
//! assert_eq!(seg.prod(1..4), 3);
//! ```

pub struct SegTree<T: Copy> {
    n: usize,
    size: usize,
    d: Vec<T>,
    op: fn(T, T) -> T,
    e: T,
}

impl<T: Copy> SegTree<T> {
    pub fn new(n: usize, e: T, op: fn(T, T) -> T) -> Self {
        let mut size = 1;
        while size < n {
            size <<= 1;
        }
        SegTree {
            n,
            size,
            d: vec![e; 2 * size],
            op,
            e,
        }
    }
    pub fn from_slice(v: &[T], e: T, op: fn(T, T) -> T) -> Self {
        let mut seg = Self::new(v.len(), e, op);
        for (i, &x) in v.iter().enumerate() {
            seg.d[seg.size + i] = x;
        }
        for i in (1..seg.size).rev() {
            seg.d[i] = (seg.op)(seg.d[2 * i], seg.d[2 * i + 1]);
        }
        seg
    }
    /// a[p] = x
    pub fn set(&mut self, p: usize, x: T) {
        assert!(p < self.n);
        let mut i = p + self.size;
        self.d[i] = x;
        i >>= 1;
        while i >= 1 {
            self.d[i] = (self.op)(self.d[2 * i], self.d[2 * i + 1]);
            i >>= 1;
        }
    }
    pub fn get(&self, p: usize) -> T {
        assert!(p < self.n);
        self.d[p + self.size]
    }
    /// 半開区間 [l, r) の積
    pub fn prod(&self, range: std::ops::Range<usize>) -> T {
        let (mut l, mut r) = (range.start + self.size, range.end + self.size);
        assert!(range.end <= self.n);
        let mut sl = self.e;
        let mut sr = self.e;
        while l < r {
            if l & 1 == 1 {
                sl = (self.op)(sl, self.d[l]);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                sr = (self.op)(self.d[r], sr);
            }
            l >>= 1;
            r >>= 1;
        }
        (self.op)(sl, sr)
    }
    pub fn all_prod(&self) -> T {
        if self.n == 0 {
            self.e
        } else {
            self.d[1]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn range_max() {
        let mut seg = SegTree::from_slice(&[1i64, 3, 2, 5, 4], i64::MIN, |a, b| a.max(b));
        assert_eq!(seg.prod(0..5), 5);
        assert_eq!(seg.prod(1..4), 5);
        assert_eq!(seg.prod(0..2), 3);
        seg.set(3, 0);
        assert_eq!(seg.prod(1..4), 3);
        assert_eq!(seg.get(3), 0);
        assert_eq!(seg.all_prod(), 4);
    }
    #[test]
    fn range_sum() {
        let seg = SegTree::from_slice(&[1i64, 2, 3, 4, 5], 0, |a, b| a + b);
        assert_eq!(seg.prod(0..5), 15);
        assert_eq!(seg.prod(1..3), 5);
    }
}
