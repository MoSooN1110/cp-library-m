//! 2 次元累積和（矩形和 O(1)）。
//!
//! ```
//! use cplib::ds::cumsum_2d::*;
//! let a = vec![vec![1i64, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
//! let cs = CumSum2D::new(&a);
//! // 行 [0,2), 列 [1,3) = 2+3+5+6 = 16
//! assert_eq!(cs.sum(0..2, 1..3), 16);
//! ```
use std::ops::Range;

pub struct CumSum2D {
    s: Vec<Vec<i64>>,
}

impl CumSum2D {
    pub fn new(a: &[Vec<i64>]) -> Self {
        let h = a.len();
        let w = if h == 0 { 0 } else { a[0].len() };
        let mut s = vec![vec![0i64; w + 1]; h + 1];
        for i in 0..h {
            for j in 0..w {
                s[i + 1][j + 1] = s[i][j + 1] + s[i + 1][j] - s[i][j] + a[i][j];
            }
        }
        CumSum2D { s }
    }
    /// 行 `rows` × 列 `cols`（いずれも半開区間）の矩形和。
    pub fn sum(&self, rows: Range<usize>, cols: Range<usize>) -> i64 {
        let (r0, r1) = (rows.start, rows.end);
        let (c0, c1) = (cols.start, cols.end);
        self.s[r1][c1] - self.s[r0][c1] - self.s[r1][c0] + self.s[r0][c0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic() {
        let a = vec![
            vec![1i64, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
        ];
        let cs = CumSum2D::new(&a);
        assert_eq!(cs.sum(0..3, 0..4), 78);
        assert_eq!(cs.sum(1..3, 1..3), 6 + 7 + 10 + 11);
        assert_eq!(cs.sum(0..1, 0..1), 1);
        assert_eq!(cs.sum(2..2, 0..4), 0);
    }
}
