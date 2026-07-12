//! 遅延伝播セグメント木（ACL 準拠、モノイド作用）。関数ポインタで与える。
//! `i64` の区間加算/区間更新 × max/min/sum は専用ラッパーも用意している。
//!
//! ```
//! use cplib::ds::lazy_segtree::*;
//!
//! let mut seg = RangeAddSum::from_slice_range_add_sum(&[1, 2, 3, 4, 5]);
//! seg.add(1..4, 10);
//! assert_eq!(seg.sum(0..5), 45);
//!
//! let mut max = RangeAssignMax::from_slice_range_assign_max(&[1, 2, 3, 4, 5]);
//! max.assign(1..4, 0);
//! assert_eq!(max.prod(0..5), 5);
//! ```

pub struct LazySegTree<S: Copy, F: Copy> {
    n: usize,
    size: usize,
    log: u32,
    d: Vec<S>,
    lz: Vec<F>,
    op: fn(S, S) -> S,
    e: S,
    mapping: fn(F, S) -> S,
    composition: fn(F, F) -> F,
    id: F,
}

impl<S: Copy, F: Copy> LazySegTree<S, F> {
    pub fn new(
        n: usize,
        e: S,
        op: fn(S, S) -> S,
        id: F,
        mapping: fn(F, S) -> S,
        composition: fn(F, F) -> F,
    ) -> Self {
        Self::from_slice(&vec![e; n], e, op, id, mapping, composition)
    }

    pub fn from_slice(
        v: &[S],
        e: S,
        op: fn(S, S) -> S,
        id: F,
        mapping: fn(F, S) -> S,
        composition: fn(F, F) -> F,
    ) -> Self {
        let n = v.len();
        let mut size = 1;
        let mut log = 0u32;
        while size < n {
            size <<= 1;
            log += 1;
        }
        let mut d = vec![e; 2 * size];
        d[size..size + n].copy_from_slice(v);
        let mut seg = LazySegTree {
            n,
            size,
            log,
            d,
            lz: vec![id; size],
            op,
            e,
            mapping,
            composition,
            id,
        };
        for i in (1..size).rev() {
            seg.update(i);
        }
        seg
    }

    #[inline]
    fn update(&mut self, k: usize) {
        self.d[k] = (self.op)(self.d[2 * k], self.d[2 * k + 1]);
    }
    #[inline]
    fn all_apply(&mut self, k: usize, f: F) {
        self.d[k] = (self.mapping)(f, self.d[k]);
        if k < self.size {
            self.lz[k] = (self.composition)(f, self.lz[k]);
        }
    }
    #[inline]
    fn push(&mut self, k: usize) {
        let f = self.lz[k];
        self.all_apply(2 * k, f);
        self.all_apply(2 * k + 1, f);
        self.lz[k] = self.id;
    }

    /// a[p] = x
    pub fn set(&mut self, p: usize, x: S) {
        assert!(p < self.n);
        let p = p + self.size;
        for i in (1..=self.log).rev() {
            self.push(p >> i);
        }
        self.d[p] = x;
        for i in 1..=self.log {
            self.update(p >> i);
        }
    }

    pub fn get(&mut self, p: usize) -> S {
        assert!(p < self.n);
        let p = p + self.size;
        for i in (1..=self.log).rev() {
            self.push(p >> i);
        }
        self.d[p]
    }

    /// [l, r) の積
    pub fn prod(&mut self, range: std::ops::Range<usize>) -> S {
        let (l, r) = (range.start, range.end);
        assert!(l <= r && r <= self.n);
        if l == r {
            return self.e;
        }
        let mut l = l + self.size;
        let mut r = r + self.size;
        for i in (1..=self.log).rev() {
            if ((l >> i) << i) != l {
                self.push(l >> i);
            }
            if ((r >> i) << i) != r {
                self.push((r - 1) >> i);
            }
        }
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

    pub fn all_prod(&self) -> S {
        self.d[1]
    }

    /// a[p] に作用 f
    pub fn apply(&mut self, p: usize, f: F) {
        assert!(p < self.n);
        let p = p + self.size;
        for i in (1..=self.log).rev() {
            self.push(p >> i);
        }
        self.d[p] = (self.mapping)(f, self.d[p]);
        for i in 1..=self.log {
            self.update(p >> i);
        }
    }

    /// [l, r) に作用 f
    pub fn apply_range(&mut self, range: std::ops::Range<usize>, f: F) {
        let (l, r) = (range.start, range.end);
        assert!(l <= r && r <= self.n);
        if l == r {
            return;
        }
        let l0 = l + self.size;
        let r0 = r + self.size;
        for i in (1..=self.log).rev() {
            if ((l0 >> i) << i) != l0 {
                self.push(l0 >> i);
            }
            if ((r0 >> i) << i) != r0 {
                self.push((r0 - 1) >> i);
            }
        }
        {
            let mut l = l0;
            let mut r = r0;
            while l < r {
                if l & 1 == 1 {
                    self.all_apply(l, f);
                    l += 1;
                }
                if r & 1 == 1 {
                    r -= 1;
                    self.all_apply(r, f);
                }
                l >>= 1;
                r >>= 1;
            }
        }
        for i in 1..=self.log {
            if ((l0 >> i) << i) != l0 {
                self.update(l0 >> i);
            }
            if ((r0 >> i) << i) != r0 {
                self.update((r0 - 1) >> i);
            }
        }
    }
}

pub type RangeAddMax = LazySegTree<i64, i64>;
pub type RangeAddMin = LazySegTree<i64, i64>;
pub type RangeAssignMax = LazySegTree<i64, Option<i64>>;
pub type RangeAssignMin = LazySegTree<i64, Option<i64>>;
pub type RangeAddSum = LazySegTree<(i64, i64), i64>;
pub type RangeAssignSum = LazySegTree<(i64, i64), Option<i64>>;

#[inline]
fn max_i64(a: i64, b: i64) -> i64 {
    a.max(b)
}

#[inline]
fn min_i64(a: i64, b: i64) -> i64 {
    a.min(b)
}

#[inline]
fn sum_pair(a: (i64, i64), b: (i64, i64)) -> (i64, i64) {
    (a.0 + b.0, a.1 + b.1)
}

#[inline]
fn add_i64(f: i64, x: i64) -> i64 {
    x + f
}

#[inline]
fn add_pair(f: i64, x: (i64, i64)) -> (i64, i64) {
    (x.0 + f * x.1, x.1)
}

#[inline]
fn compose_add(f: i64, g: i64) -> i64 {
    f + g
}

#[inline]
fn assign_i64(f: Option<i64>, x: i64) -> i64 {
    f.unwrap_or(x)
}

#[inline]
fn assign_pair(f: Option<i64>, x: (i64, i64)) -> (i64, i64) {
    match f {
        Some(v) => (v * x.1, x.1),
        None => x,
    }
}

#[inline]
fn compose_assign(f: Option<i64>, g: Option<i64>) -> Option<i64> {
    f.or(g)
}

fn sum_nodes(v: &[i64]) -> Vec<(i64, i64)> {
    v.iter().map(|&x| (x, 1)).collect()
}

impl RangeAddMax {
    pub fn range_add_max(n: usize) -> Self {
        Self::from_slice_range_add_max(&vec![0; n])
    }

    pub fn from_slice_range_add_max(v: &[i64]) -> Self {
        Self::from_slice(v, i64::MIN, max_i64, 0, add_i64, compose_add)
    }

    pub fn add(&mut self, range: std::ops::Range<usize>, x: i64) {
        self.apply_range(range, x);
    }
}

impl RangeAddMin {
    pub fn range_add_min(n: usize) -> Self {
        Self::from_slice_range_add_min(&vec![0; n])
    }

    pub fn from_slice_range_add_min(v: &[i64]) -> Self {
        Self::from_slice(v, i64::MAX, min_i64, 0, add_i64, compose_add)
    }
}

impl RangeAddSum {
    pub fn range_add_sum(n: usize) -> Self {
        Self::from_slice_range_add_sum(&vec![0; n])
    }

    pub fn from_slice_range_add_sum(v: &[i64]) -> Self {
        let nodes = sum_nodes(v);
        Self::from_slice(&nodes, (0, 0), sum_pair, 0, add_pair, compose_add)
    }

    pub fn add(&mut self, range: std::ops::Range<usize>, x: i64) {
        self.apply_range(range, x);
    }

    pub fn sum(&mut self, range: std::ops::Range<usize>) -> i64 {
        self.prod(range).0
    }
}

impl RangeAssignMax {
    pub fn range_assign_max(n: usize) -> Self {
        Self::from_slice_range_assign_max(&vec![0; n])
    }

    pub fn from_slice_range_assign_max(v: &[i64]) -> Self {
        Self::from_slice(v, i64::MIN, max_i64, None, assign_i64, compose_assign)
    }

    pub fn assign(&mut self, range: std::ops::Range<usize>, x: i64) {
        self.apply_range(range, Some(x));
    }
}

impl RangeAssignMin {
    pub fn range_assign_min(n: usize) -> Self {
        Self::from_slice_range_assign_min(&vec![0; n])
    }

    pub fn from_slice_range_assign_min(v: &[i64]) -> Self {
        Self::from_slice(v, i64::MAX, min_i64, None, assign_i64, compose_assign)
    }
}

impl RangeAssignSum {
    pub fn range_assign_sum(n: usize) -> Self {
        Self::from_slice_range_assign_sum(&vec![0; n])
    }

    pub fn from_slice_range_assign_sum(v: &[i64]) -> Self {
        let nodes = sum_nodes(v);
        Self::from_slice(&nodes, (0, 0), sum_pair, None, assign_pair, compose_assign)
    }

    pub fn assign(&mut self, range: std::ops::Range<usize>, x: i64) {
        self.apply_range(range, Some(x));
    }

    pub fn sum(&mut self, range: std::ops::Range<usize>) -> i64 {
        self.prod(range).0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // 区間加算・区間和
    fn build(v: &[i64]) -> LazySegTree<(i64, i64), i64> {
        let s: Vec<(i64, i64)> = v.iter().map(|&x| (x, 1)).collect();
        LazySegTree::from_slice(
            &s,
            (0, 0),
            |a, b| (a.0 + b.0, a.1 + b.1),
            0,
            |f, x| (x.0 + f * x.1, x.1),
            |f, g| f + g,
        )
    }

    #[test]
    fn range_add_range_sum() {
        let mut seg = build(&[1, 2, 3, 4, 5]);
        assert_eq!(seg.prod(0..5).0, 15);
        seg.apply_range(1..4, 10); // [1,12,13,14,5]
        assert_eq!(seg.prod(0..5).0, 45);
        assert_eq!(seg.prod(1..4).0, 39);
        assert_eq!(seg.get(2).0, 13);
        seg.set(0, (100, 1));
        assert_eq!(seg.prod(0..1).0, 100);
    }

    #[test]
    fn range_affine_range_sum() {
        // S=(sum,len), F=(mul,add): x -> mul*x + add
        let v: Vec<(i64, i64)> = [1i64, 2, 3, 4, 5].iter().map(|&x| (x, 1)).collect();
        let mut seg = LazySegTree::from_slice(
            &v,
            (0, 0),
            |a: (i64, i64), b: (i64, i64)| (a.0 + b.0, a.1 + b.1),
            (1i64, 0i64), // id: x -> x
            |f: (i64, i64), x: (i64, i64)| (f.0 * x.0 + f.1 * x.1, x.1),
            |f: (i64, i64), g: (i64, i64)| (f.0 * g.0, f.0 * g.1 + f.1), // f after g
        );
        assert_eq!(seg.prod(0..5).0, 15);
        seg.apply_range(1..4, (2, 1)); // [2,3,4]->[5,7,9]
        assert_eq!(seg.prod(0..5).0, 1 + 5 + 7 + 9 + 5);
        seg.apply_range(0..5, (1, 10)); // 全体 +10
        assert_eq!(
            seg.prod(0..5).0,
            (1 + 10) + (5 + 10) + (7 + 10) + (9 + 10) + (5 + 10)
        );
    }

    #[test]
    fn brute_random() {
        // 決定的擬似乱数でナイーブ比較
        let n = 30;
        let mut a = vec![0i64; n];
        let mut seg = build(&a);
        let mut x: u64 = 12345;
        let mut rng = || {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            x
        };
        for _ in 0..300 {
            let l = (rng() as usize) % n;
            let r = l + 1 + (rng() as usize) % (n - l);
            if rng() % 2 == 0 {
                let f = (rng() % 21) as i64 - 10;
                for i in l..r {
                    a[i] += f;
                }
                seg.apply_range(l..r, f);
            } else {
                let s: i64 = a[l..r].iter().sum();
                assert_eq!(seg.prod(l..r).0, s);
            }
        }
    }

    #[test]
    fn builtin_range_add_max_min_sum() {
        let mut max = RangeAddMax::from_slice_range_add_max(&[1, 2, 3, 4, 5]);
        let mut min = RangeAddMin::from_slice_range_add_min(&[1, 2, 3, 4, 5]);
        let mut sum = RangeAddSum::from_slice_range_add_sum(&[1, 2, 3, 4, 5]);
        max.add(1..4, 10);
        min.add(1..4, 10);
        sum.add(1..4, 10);
        assert_eq!(max.prod(0..5), 14);
        assert_eq!(min.prod(0..5), 1);
        assert_eq!(sum.sum(0..5), 45);
        assert_eq!(sum.get(2).0, 13);
    }

    #[test]
    fn builtin_range_assign_max_min_sum() {
        let mut max = RangeAssignMax::from_slice_range_assign_max(&[1, 2, 3, 4, 5]);
        let mut min = RangeAssignMin::from_slice_range_assign_min(&[1, 2, 3, 4, 5]);
        let mut sum = RangeAssignSum::from_slice_range_assign_sum(&[1, 2, 3, 4, 5]);
        max.assign(1..4, 0);
        min.assign(1..4, 0);
        sum.assign(1..4, 0);
        assert_eq!(max.prod(0..5), 5);
        assert_eq!(min.prod(0..5), 0);
        assert_eq!(sum.sum(0..5), 6);
        max.assign(0..5, -7);
        min.assign(0..5, -7);
        sum.assign(0..5, -7);
        assert_eq!(max.prod(0..5), -7);
        assert_eq!(min.prod(0..5), -7);
        assert_eq!(sum.sum(0..5), -35);
    }
}
