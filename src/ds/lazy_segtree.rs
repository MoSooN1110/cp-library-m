//! 遅延伝播セグメント木（ACL 準拠、モノイド作用）。関数ポインタで与える。
//!
//! ```
//! use cplib::ds::lazy_segtree::*;
//! // 区間加算・区間和。S=(sum,len), F=add
//! let mut seg = LazySegTree::new(
//!     5,
//!     (0i64, 1i64),                                   // e: 単位元（len=1 は from_slice 用）
//!     |a: (i64, i64), b: (i64, i64)| (a.0 + b.0, a.1 + b.1),
//!     0i64,                                           // id: 恒等作用
//!     |f: i64, x: (i64, i64)| (x.0 + f * x.1, x.1),   // mapping
//!     |f: i64, g: i64| f + g,                         // composition
//! );
//! seg.apply_range(1..4, 3);
//! assert_eq!(seg.prod(0..5).0, 9);
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
}
