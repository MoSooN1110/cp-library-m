//! Dynamic Segment Tree（疎な点更新・区間積）。
//!
//! 添字範囲が大きく、実際に更新する点が少ないときに使う。
//!
//! ```
//! use cplib::ds::dynamic_segtree::*;
//!
//! let mut seg = DynamicSegTree::new(1_000_000_000, 0i64, |a, b| a + b);
//! seg.set(123, 5);
//! seg.set(999_999_999, 7);
//! assert_eq!(seg.get(123), 5);
//! assert_eq!(seg.prod(0..124), 5);
//! assert_eq!(seg.prod(0..1_000_000_000), 12);
//! ```

pub struct DynamicSegTree<T: Clone> {
    n: usize,
    root: Option<usize>,
    nodes: Vec<Node<T>>,
    e: T,
    op: fn(T, T) -> T,
}

struct Node<T: Clone> {
    left: Option<usize>,
    right: Option<usize>,
    val: T,
}

impl<T: Clone> DynamicSegTree<T> {
    pub fn new(n: usize, e: T, op: fn(T, T) -> T) -> Self {
        Self {
            n,
            root: None,
            nodes: Vec::new(),
            e,
            op,
        }
    }

    pub fn len(&self) -> usize {
        self.n
    }

    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    /// a[p] = x
    pub fn set(&mut self, p: usize, x: T) {
        assert!(p < self.n);
        let root = self.ensure_node(self.root);
        self.root = Some(root);
        self.set_rec(root, 0, self.n, p, x);
    }

    pub fn get(&self, p: usize) -> T {
        assert!(p < self.n);
        self.get_rec(self.root, 0, self.n, p)
    }

    /// 半開区間 [l, r) の積。
    pub fn prod(&self, range: std::ops::Range<usize>) -> T {
        assert!(range.start <= range.end && range.end <= self.n);
        self.prod_rec(self.root, 0, self.n, range.start, range.end)
    }

    pub fn all_prod(&self) -> T {
        self.root
            .map(|i| self.nodes[i].val.clone())
            .unwrap_or_else(|| self.e.clone())
    }

    fn ensure_node(&mut self, idx: Option<usize>) -> usize {
        if let Some(idx) = idx {
            idx
        } else {
            let idx = self.nodes.len();
            self.nodes.push(Node {
                left: None,
                right: None,
                val: self.e.clone(),
            });
            idx
        }
    }

    fn set_rec(&mut self, idx: usize, l: usize, r: usize, p: usize, x: T) {
        if r - l == 1 {
            self.nodes[idx].val = x;
            return;
        }
        let m = l + (r - l) / 2;
        if p < m {
            let child = self.ensure_node(self.nodes[idx].left);
            self.nodes[idx].left = Some(child);
            self.set_rec(child, l, m, p, x);
        } else {
            let child = self.ensure_node(self.nodes[idx].right);
            self.nodes[idx].right = Some(child);
            self.set_rec(child, m, r, p, x);
        }
        let left = self.nodes[idx]
            .left
            .map(|i| self.nodes[i].val.clone())
            .unwrap_or_else(|| self.e.clone());
        let right = self.nodes[idx]
            .right
            .map(|i| self.nodes[i].val.clone())
            .unwrap_or_else(|| self.e.clone());
        self.nodes[idx].val = (self.op)(left, right);
    }

    fn get_rec(&self, idx: Option<usize>, l: usize, r: usize, p: usize) -> T {
        let Some(idx) = idx else {
            return self.e.clone();
        };
        if r - l == 1 {
            return self.nodes[idx].val.clone();
        }
        let m = l + (r - l) / 2;
        if p < m {
            self.get_rec(self.nodes[idx].left, l, m, p)
        } else {
            self.get_rec(self.nodes[idx].right, m, r, p)
        }
    }

    fn prod_rec(&self, idx: Option<usize>, l: usize, r: usize, ql: usize, qr: usize) -> T {
        if ql == qr || qr <= l || r <= ql {
            return self.e.clone();
        }
        let Some(idx) = idx else {
            return self.e.clone();
        };
        if ql <= l && r <= qr {
            return self.nodes[idx].val.clone();
        }
        let m = l + (r - l) / 2;
        let left = self.prod_rec(self.nodes[idx].left, l, m, ql, qr);
        let right = self.prod_rec(self.nodes[idx].right, m, r, ql, qr);
        (self.op)(left, right)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sparse_sum() {
        let mut seg = DynamicSegTree::new(1_000_000_000, 0i64, |a, b| a + b);
        assert_eq!(seg.len(), 1_000_000_000);
        assert_eq!(seg.all_prod(), 0);
        seg.set(10, 3);
        seg.set(100_000_000, 5);
        seg.set(999_999_999, 7);
        assert_eq!(seg.get(9), 0);
        assert_eq!(seg.get(10), 3);
        assert_eq!(seg.prod(0..11), 3);
        assert_eq!(seg.prod(11..999_999_999), 5);
        assert_eq!(seg.prod(0..1_000_000_000), 15);
        seg.set(10, -4);
        assert_eq!(seg.prod(0..101_000_000), 1);
    }

    #[test]
    fn max_and_empty_ranges() {
        let mut seg = DynamicSegTree::new(8, i64::MIN, |a, b| a.max(b));
        assert_eq!(seg.prod(3..3), i64::MIN);
        seg.set(2, 10);
        seg.set(5, 4);
        seg.set(7, 20);
        assert_eq!(seg.prod(0..8), 20);
        assert_eq!(seg.prod(0..6), 10);
        assert_eq!(seg.prod(3..7), 4);
        seg.set(2, -1);
        assert_eq!(seg.prod(0..6), 4);
    }

    #[test]
    fn random_vs_vec() {
        let n = 64;
        let mut seg = DynamicSegTree::new(n, 0i64, |a, b| a + b);
        let mut a = vec![0i64; n];
        let mut x = 246813579u64;
        let mut rng = || {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            x
        };
        for _ in 0..500 {
            if rng() & 1 == 0 {
                let p = rng() as usize % n;
                let v = (rng() % 101) as i64 - 50;
                a[p] = v;
                seg.set(p, v);
            } else {
                let l = rng() as usize % (n + 1);
                let r = l + rng() as usize % (n + 1 - l);
                let expected: i64 = a[l..r].iter().sum();
                assert_eq!(seg.prod(l..r), expected);
            }
        }
        assert_eq!(seg.all_prod(), a.iter().sum());
    }
}
