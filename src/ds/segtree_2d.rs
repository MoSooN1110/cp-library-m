//! 2D Segment Tree（点更新・矩形区間積）。
//!
//! ```
//! use cplib::ds::segtree_2d::*;
//!
//! let a = vec![vec![1i64, 2, 3], vec![4, 5, 6]];
//! let mut seg = SegTree2D::from_vec(&a, 0, |x, y| x + y);
//! assert_eq!(seg.prod(0..2, 1..3), 16);
//! seg.set(1, 2, 10);
//! assert_eq!(seg.prod(0..2, 1..3), 20);
//! ```

pub struct SegTree2D<T: Copy> {
    h: usize,
    w: usize,
    size_h: usize,
    size_w: usize,
    d: Vec<T>,
    e: T,
    op: fn(T, T) -> T,
}

impl<T: Copy> SegTree2D<T> {
    pub fn new(h: usize, w: usize, e: T, op: fn(T, T) -> T) -> Self {
        let size_h = h.next_power_of_two();
        let size_w = w.next_power_of_two();
        Self {
            h,
            w,
            size_h,
            size_w,
            d: vec![e; 4 * size_h * size_w],
            e,
            op,
        }
    }

    pub fn from_vec(a: &[Vec<T>], e: T, op: fn(T, T) -> T) -> Self {
        let h = a.len();
        let w = a.first().map_or(0, Vec::len);
        assert!(a.iter().all(|row| row.len() == w));
        let mut seg = Self::new(h, w, e, op);
        for (i, row) in a.iter().enumerate() {
            for (j, &x) in row.iter().enumerate() {
                let idx = seg.idx(i + seg.size_h, j + seg.size_w);
                seg.d[idx] = x;
            }
        }
        for i in seg.size_h..2 * seg.size_h {
            for j in (1..seg.size_w).rev() {
                seg.pull_col(i, j);
            }
        }
        for i in (1..seg.size_h).rev() {
            for j in 1..2 * seg.size_w {
                seg.pull_row(i, j);
            }
        }
        seg
    }

    pub fn height(&self) -> usize {
        self.h
    }

    pub fn width(&self) -> usize {
        self.w
    }

    /// a[i][j] = x
    pub fn set(&mut self, i: usize, j: usize, x: T) {
        assert!(i < self.h && j < self.w);
        let mut row = i + self.size_h;
        let col = j + self.size_w;
        let idx = self.idx(row, col);
        self.d[idx] = x;
        let mut c = col >> 1;
        while c > 0 {
            self.pull_col(row, c);
            c >>= 1;
        }
        row >>= 1;
        while row > 0 {
            let mut c = col;
            self.pull_row(row, c);
            c >>= 1;
            while c > 0 {
                self.pull_col(row, c);
                c >>= 1;
            }
            row >>= 1;
        }
    }

    pub fn get(&self, i: usize, j: usize) -> T {
        assert!(i < self.h && j < self.w);
        self.d[self.idx(i + self.size_h, j + self.size_w)]
    }

    /// 半開矩形 [row.start, row.end) x [col.start, col.end) の積。
    pub fn prod(&self, row: std::ops::Range<usize>, col: std::ops::Range<usize>) -> T {
        assert!(row.start <= row.end && row.end <= self.h);
        assert!(col.start <= col.end && col.end <= self.w);
        if row.start == row.end || col.start == col.end {
            return self.e;
        }
        let mut upper = self.e;
        let mut lower = self.e;
        let mut l = row.start + self.size_h;
        let mut r = row.end + self.size_h;
        while l < r {
            if l & 1 == 1 {
                upper = (self.op)(upper, self.prod_col(l, col.clone()));
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                lower = (self.op)(self.prod_col(r, col.clone()), lower);
            }
            l >>= 1;
            r >>= 1;
        }
        (self.op)(upper, lower)
    }

    pub fn all_prod(&self) -> T {
        if self.h == 0 || self.w == 0 {
            self.e
        } else {
            self.d[self.idx(1, 1)]
        }
    }

    #[inline]
    fn idx(&self, i: usize, j: usize) -> usize {
        i * (2 * self.size_w) + j
    }

    #[inline]
    fn pull_col(&mut self, i: usize, j: usize) {
        let p = self.idx(i, j);
        let l = self.idx(i, 2 * j);
        let r = self.idx(i, 2 * j + 1);
        self.d[p] = (self.op)(self.d[l], self.d[r]);
    }

    #[inline]
    fn pull_row(&mut self, i: usize, j: usize) {
        let p = self.idx(i, j);
        let l = self.idx(2 * i, j);
        let r = self.idx(2 * i + 1, j);
        self.d[p] = (self.op)(self.d[l], self.d[r]);
    }

    fn prod_col(&self, row_node: usize, col: std::ops::Range<usize>) -> T {
        let mut left = self.e;
        let mut right = self.e;
        let mut l = col.start + self.size_w;
        let mut r = col.end + self.size_w;
        while l < r {
            if l & 1 == 1 {
                left = (self.op)(left, self.d[self.idx(row_node, l)]);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                right = (self.op)(self.d[self.idx(row_node, r)], right);
            }
            l >>= 1;
            r >>= 1;
        }
        (self.op)(left, right)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range_sum() {
        let a = vec![vec![1i64, 2, 3], vec![4, 5, 6]];
        let mut seg = SegTree2D::from_vec(&a, 0, |x, y| x + y);
        assert_eq!(seg.height(), 2);
        assert_eq!(seg.width(), 3);
        assert_eq!(seg.all_prod(), 21);
        assert_eq!(seg.prod(0..2, 1..3), 16);
        assert_eq!(seg.prod(1..2, 0..2), 9);
        assert_eq!(seg.prod(0..0, 0..3), 0);
        seg.set(1, 2, 10);
        assert_eq!(seg.get(1, 2), 10);
        assert_eq!(seg.prod(0..2, 1..3), 20);
        assert_eq!(seg.all_prod(), 25);
    }

    #[test]
    fn range_max() {
        let a = vec![vec![3i64, 1, 4, 1], vec![5, 9, 2, 6], vec![5, 3, 5, 8]];
        let mut seg = SegTree2D::from_vec(&a, i64::MIN, |x, y| x.max(y));
        assert_eq!(seg.prod(0..3, 0..4), 9);
        assert_eq!(seg.prod(1..3, 2..4), 8);
        seg.set(0, 1, 10);
        assert_eq!(seg.prod(0..1, 0..4), 10);
        assert_eq!(seg.prod(0..3, 0..4), 10);
    }

    #[test]
    fn random_vs_brute() {
        let h = 8;
        let w = 7;
        let mut a = vec![vec![0i64; w]; h];
        let mut seg = SegTree2D::new(h, w, 0, |x, y| x + y);
        let mut seed = 987654321u64;
        let mut rng = || {
            seed ^= seed << 7;
            seed ^= seed >> 9;
            seed
        };
        for _ in 0..500 {
            if rng() & 1 == 0 {
                let i = rng() as usize % h;
                let j = rng() as usize % w;
                let x = (rng() % 101) as i64 - 50;
                a[i][j] = x;
                seg.set(i, j, x);
            } else {
                let r1 = rng() as usize % (h + 1);
                let r2 = r1 + rng() as usize % (h + 1 - r1);
                let c1 = rng() as usize % (w + 1);
                let c2 = c1 + rng() as usize % (w + 1 - c1);
                let expected: i64 = a[r1..r2]
                    .iter()
                    .map(|row| row[c1..c2].iter().sum::<i64>())
                    .sum();
                assert_eq!(seg.prod(r1..r2, c1..c2), expected);
            }
        }
    }
}
