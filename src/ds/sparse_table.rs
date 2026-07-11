//! Sparse Table（冪等モノイドの静的区間クエリ O(1)）。min/max/gcd 等。
//!
//! ```
//! use cplib::ds::sparse_table::*;
//! let st = SparseTable::new(&[3i64, 1, 4, 1, 5, 9, 2], |a, b| a.min(b));
//! assert_eq!(st.query(1..5), 1);   // min of [1,4,1,5]
//! assert_eq!(st.query(4..7), 2);
//! ```

pub struct SparseTable<T: Copy> {
    table: Vec<Vec<T>>,
    op: fn(T, T) -> T,
}

impl<T: Copy> SparseTable<T> {
    /// `op` は結合的かつ冪等（op(x,x)=x）であること。
    pub fn new(v: &[T], op: fn(T, T) -> T) -> Self {
        let n = v.len();
        let mut log = 1;
        while (1 << log) <= n {
            log += 1;
        }
        let mut table = vec![v.to_vec()];
        for k in 1..log {
            let len = 1 << k;
            let prev = &table[k - 1];
            let mut cur = Vec::with_capacity(n);
            for i in 0..n {
                if i + len <= n {
                    cur.push(op(prev[i], prev[i + (len >> 1)]));
                } else {
                    cur.push(prev[i]);
                }
            }
            table.push(cur);
        }
        SparseTable { table, op }
    }

    /// 半開区間 [l, r) のクエリ（l < r 必須）。
    pub fn query(&self, range: std::ops::Range<usize>) -> T {
        let (l, r) = (range.start, range.end);
        assert!(l < r);
        let len = r - l;
        let k = (usize::BITS - 1 - len.leading_zeros()) as usize; // floor(log2(len))
        (self.op)(self.table[k][l], self.table[k][r - (1 << k)])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn gcd(a: u64, b: u64) -> u64 {
        if b == 0 {
            a
        } else {
            gcd(b, a % b)
        }
    }
    #[test]
    fn min_max_gcd() {
        let a = [5i64, 3, 8, 1, 9, 2, 7];
        let st_min = SparseTable::new(&a, |x, y| x.min(y));
        let st_max = SparseTable::new(&a, |x, y| x.max(y));
        for l in 0..a.len() {
            for r in l + 1..=a.len() {
                let mn = *a[l..r].iter().min().unwrap();
                let mx = *a[l..r].iter().max().unwrap();
                assert_eq!(st_min.query(l..r), mn);
                assert_eq!(st_max.query(l..r), mx);
            }
        }
        let g = [12u64, 18, 24, 30];
        let st_g = SparseTable::new(&g, gcd);
        assert_eq!(st_g.query(0..4), 6);
        assert_eq!(st_g.query(1..3), 6);
    }
}
