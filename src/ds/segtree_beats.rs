//! Segment Tree Beats（区間 chmin・区間 chmax・区間加算・区間和、`i64`）。
//!
//! 「Ji Driver のアルゴリズム」。ならし O(log^2 n) で区間 chmin/chmax/加算・区間和を扱える。
//!
//! ```
//! use cplib::ds::segtree_beats::*;
//! let mut seg = SegTreeBeats::new(&[4i64, 1, 5, 9, 2, 6]);
//! seg.range_chmin(0, 5, 4); // [4,1,4,4,2,6]
//! seg.range_add(0, 6, 1); // [5,2,5,5,3,7]
//! assert_eq!(seg.range_sum(0, 6), 5 + 2 + 5 + 5 + 3 + 7);
//! seg.range_chmax(0, 3, 5); // [5,5,5,5,3,7]
//! assert_eq!(seg.range_sum(0, 3), 15);
//! ```

const INF: i64 = i64::MAX / 4;

#[derive(Clone, Copy, Debug)]
struct Node {
    sum: i64,
    len: i64,
    max1: i64,
    max2: i64,
    max_cnt: i64,
    min1: i64,
    min2: i64,
    min_cnt: i64,
    add: i64,
}

impl Node {
    fn leaf(v: i64) -> Self {
        Node {
            sum: v,
            len: 1,
            max1: v,
            max2: -INF,
            max_cnt: 1,
            min1: v,
            min2: INF,
            min_cnt: 1,
            add: 0,
        }
    }
    fn identity() -> Self {
        Node {
            sum: 0,
            len: 0,
            max1: -INF,
            max2: -INF,
            max_cnt: 0,
            min1: INF,
            min2: INF,
            min_cnt: 0,
            add: 0,
        }
    }
}

fn merge(l: Node, r: Node) -> Node {
    // 片方が空（padding）のときはもう片方をそのまま返すが、add は
    // 「このノードから子への未反映分」を意味するので 0 にリセットする
    // （相手側の add をそのまま持ち越すと、後で push_down 時に二重適用されてしまう）。
    if l.len == 0 {
        let mut r = r;
        r.add = 0;
        return r;
    }
    if r.len == 0 {
        let mut l = l;
        l.add = 0;
        return l;
    }
    let (max1, max_cnt, max2) = match l.max1.cmp(&r.max1) {
        std::cmp::Ordering::Equal => (l.max1, l.max_cnt + r.max_cnt, l.max2.max(r.max2)),
        std::cmp::Ordering::Greater => (l.max1, l.max_cnt, l.max2.max(r.max1)),
        std::cmp::Ordering::Less => (r.max1, r.max_cnt, r.max2.max(l.max1)),
    };
    let (min1, min_cnt, min2) = match l.min1.cmp(&r.min1) {
        std::cmp::Ordering::Equal => (l.min1, l.min_cnt + r.min_cnt, l.min2.min(r.min2)),
        std::cmp::Ordering::Less => (l.min1, l.min_cnt, l.min2.min(r.min1)),
        std::cmp::Ordering::Greater => (r.min1, r.min_cnt, r.min2.min(l.min1)),
    };
    Node {
        sum: l.sum + r.sum,
        len: l.len + r.len,
        max1,
        max2,
        max_cnt,
        min1,
        min2,
        min_cnt,
        add: 0,
    }
}

/// Segment Tree Beats 本体。要素は `i64`。
pub struct SegTreeBeats {
    n: usize,
    size: usize,
    node: Vec<Node>,
}

impl SegTreeBeats {
    pub fn new(a: &[i64]) -> Self {
        let n = a.len();
        let mut size = 1usize;
        while size < n.max(1) {
            size <<= 1;
        }
        let mut node = vec![Node::identity(); 2 * size];
        for (i, &v) in a.iter().enumerate() {
            node[size + i] = Node::leaf(v);
        }
        let mut s = Self { n, size, node };
        for i in (1..size).rev() {
            s.pull(i);
        }
        s
    }

    pub fn len(&self) -> usize {
        self.n
    }

    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    #[inline]
    fn pull(&mut self, k: usize) {
        self.node[k] = merge(self.node[2 * k], self.node[2 * k + 1]);
    }

    fn apply_add(&mut self, k: usize, x: i64) {
        let nd = &mut self.node[k];
        nd.sum += x * nd.len;
        nd.max1 += x;
        if nd.max2 > -INF {
            nd.max2 += x;
        }
        nd.min1 += x;
        if nd.min2 < INF {
            nd.min2 += x;
        }
        nd.add += x;
    }

    fn apply_chmin(&mut self, k: usize, x: i64) {
        let nd = &mut self.node[k];
        if nd.max1 <= x {
            return;
        }
        nd.sum -= (nd.max1 - x) * nd.max_cnt;
        if nd.min1 == nd.max1 {
            nd.min1 = x;
        } else if nd.min2 == nd.max1 {
            nd.min2 = x;
        }
        nd.max1 = x;
    }

    fn apply_chmax(&mut self, k: usize, x: i64) {
        let nd = &mut self.node[k];
        if nd.min1 >= x {
            return;
        }
        nd.sum += (x - nd.min1) * nd.min_cnt;
        if nd.max1 == nd.min1 {
            nd.max1 = x;
        } else if nd.max2 == nd.min1 {
            nd.max2 = x;
        }
        nd.min1 = x;
    }

    fn push_down(&mut self, k: usize) {
        let add = self.node[k].add;
        let max1 = self.node[k].max1;
        let min1 = self.node[k].min1;
        for c in [2 * k, 2 * k + 1] {
            if add != 0 {
                self.apply_add(c, add);
            }
            if self.node[c].max1 > max1 {
                self.apply_chmin(c, max1);
            }
            if self.node[c].min1 < min1 {
                self.apply_chmax(c, min1);
            }
        }
        self.node[k].add = 0;
    }

    /// [l, r) の各要素を min(a[i], x) に更新する。
    pub fn range_chmin(&mut self, l: usize, r: usize, x: i64) {
        assert!(l <= r && r <= self.n);
        if l < r {
            self.chmin_rec(1, 0, self.size, l, r, x);
        }
    }
    fn chmin_rec(&mut self, k: usize, nl: usize, nr: usize, l: usize, r: usize, x: i64) {
        if r <= nl || nr <= l || self.node[k].max1 <= x {
            return;
        }
        if l <= nl && nr <= r && self.node[k].max2 < x {
            self.apply_chmin(k, x);
            return;
        }
        self.push_down(k);
        let mid = (nl + nr) / 2;
        self.chmin_rec(2 * k, nl, mid, l, r, x);
        self.chmin_rec(2 * k + 1, mid, nr, l, r, x);
        self.pull(k);
    }

    /// [l, r) の各要素を max(a[i], x) に更新する。
    pub fn range_chmax(&mut self, l: usize, r: usize, x: i64) {
        assert!(l <= r && r <= self.n);
        if l < r {
            self.chmax_rec(1, 0, self.size, l, r, x);
        }
    }
    fn chmax_rec(&mut self, k: usize, nl: usize, nr: usize, l: usize, r: usize, x: i64) {
        if r <= nl || nr <= l || self.node[k].min1 >= x {
            return;
        }
        if l <= nl && nr <= r && self.node[k].min2 > x {
            self.apply_chmax(k, x);
            return;
        }
        self.push_down(k);
        let mid = (nl + nr) / 2;
        self.chmax_rec(2 * k, nl, mid, l, r, x);
        self.chmax_rec(2 * k + 1, mid, nr, l, r, x);
        self.pull(k);
    }

    /// [l, r) の各要素に x を加算する。
    pub fn range_add(&mut self, l: usize, r: usize, x: i64) {
        assert!(l <= r && r <= self.n);
        if l < r {
            self.add_rec(1, 0, self.size, l, r, x);
        }
    }
    fn add_rec(&mut self, k: usize, nl: usize, nr: usize, l: usize, r: usize, x: i64) {
        if r <= nl || nr <= l {
            return;
        }
        if l <= nl && nr <= r {
            self.apply_add(k, x);
            return;
        }
        self.push_down(k);
        let mid = (nl + nr) / 2;
        self.add_rec(2 * k, nl, mid, l, r, x);
        self.add_rec(2 * k + 1, mid, nr, l, r, x);
        self.pull(k);
    }

    /// [l, r) の総和。
    pub fn range_sum(&mut self, l: usize, r: usize) -> i64 {
        assert!(l <= r && r <= self.n);
        if l == r {
            return 0;
        }
        self.query_rec(1, 0, self.size, l, r).sum
    }

    /// [l, r) の最小値。
    pub fn range_min(&mut self, l: usize, r: usize) -> i64 {
        assert!(l < r && r <= self.n);
        self.query_rec(1, 0, self.size, l, r).min1
    }

    /// [l, r) の最大値。
    pub fn range_max(&mut self, l: usize, r: usize) -> i64 {
        assert!(l < r && r <= self.n);
        self.query_rec(1, 0, self.size, l, r).max1
    }

    fn query_rec(&mut self, k: usize, nl: usize, nr: usize, l: usize, r: usize) -> Node {
        if r <= nl || nr <= l {
            return Node::identity();
        }
        if l <= nl && nr <= r {
            return self.node[k];
        }
        self.push_down(k);
        let mid = (nl + nr) / 2;
        let a = self.query_rec(2 * k, nl, mid, l, r);
        let b = self.query_rec(2 * k + 1, mid, nr, l, r);
        merge(a, b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_values() {
        let mut seg = SegTreeBeats::new(&[4i64, 1, 5, 9, 2, 6]);
        assert_eq!(seg.range_sum(0, 6), 27);
        seg.range_chmin(0, 5, 4);
        assert_eq!(seg.range_sum(0, 6), 4 + 1 + 4 + 4 + 2 + 6);
        seg.range_add(0, 6, 1);
        assert_eq!(seg.range_sum(0, 6), 5 + 2 + 5 + 5 + 3 + 7);
        seg.range_chmax(0, 3, 5);
        assert_eq!(seg.range_sum(0, 3), 15);
    }

    #[derive(Clone)]
    struct Naive(Vec<i64>);
    impl Naive {
        fn chmin(&mut self, l: usize, r: usize, x: i64) {
            for v in &mut self.0[l..r] {
                *v = (*v).min(x);
            }
        }
        fn chmax(&mut self, l: usize, r: usize, x: i64) {
            for v in &mut self.0[l..r] {
                *v = (*v).max(x);
            }
        }
        fn add(&mut self, l: usize, r: usize, x: i64) {
            for v in &mut self.0[l..r] {
                *v += x;
            }
        }
        fn sum(&self, l: usize, r: usize) -> i64 {
            self.0[l..r].iter().sum()
        }
    }

    #[test]
    fn random_vs_naive() {
        let mut x: u64 = 998244353;
        let mut rng = || {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            x
        };
        for trial in 0..20 {
            let n = 1 + (rng() % 40) as usize;
            let init: Vec<i64> = (0..n).map(|_| (rng() % 201) as i64 - 100).collect();
            let mut seg = SegTreeBeats::new(&init);
            let mut naive = Naive(init.clone());
            assert_eq!(seg.len(), n);

            for _ in 0..300 {
                let l = (rng() as usize) % n;
                let r = l + 1 + (rng() as usize) % (n - l);
                let op = rng() % 4;
                match op {
                    0 => {
                        let v = (rng() % 201) as i64 - 100;
                        seg.range_chmin(l, r, v);
                        naive.chmin(l, r, v);
                    }
                    1 => {
                        let v = (rng() % 201) as i64 - 100;
                        seg.range_chmax(l, r, v);
                        naive.chmax(l, r, v);
                    }
                    2 => {
                        let v = (rng() % 21) as i64 - 10;
                        seg.range_add(l, r, v);
                        naive.add(l, r, v);
                    }
                    _ => {
                        assert_eq!(seg.range_sum(l, r), naive.sum(l, r), "trial {trial} sum");
                    }
                }
            }
            assert_eq!(seg.range_sum(0, n), naive.sum(0, n), "trial {trial} final");
        }
    }
}
