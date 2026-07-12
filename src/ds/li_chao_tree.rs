//! Li Chao Tree（直線/線分の任意順挿入・1 点最小値クエリ）。
//! 傾き単調が保証できる場合は `ds::convex_hull_trick` の方が軽い。
//!
//! クエリ座標の候補 xs を先に与える（座標圧縮）。値は `a*x+b` を i64 で計算するので
//! |a*x+b| が i64 に収まる範囲で使うこと。
//!
//! ```
//! use cplib::ds::li_chao_tree::*;
//! let mut lct = LiChaoTree::new(&[-2, 0, 1, 3]);
//! lct.add_line(1, 0);    // y = x
//! lct.add_line(-1, 2);   // y = -x + 2
//! assert_eq!(lct.query(-2), -2);
//! assert_eq!(lct.query(3), -1);
//! ```

#[derive(Clone, Copy)]
struct Line {
    a: i64,
    b: i64,
}

impl Line {
    fn eval(&self, x: i64) -> i64 {
        self.a * x + self.b
    }
}

pub struct LiChaoTree {
    n: usize,
    xs: Vec<i64>,
    node: Vec<Option<Line>>,
}

impl LiChaoTree {
    /// クエリで使う x 座標の候補（重複可・未ソート可）を与えて構築する。
    pub fn new(xs: &[i64]) -> Self {
        let mut xs = xs.to_vec();
        xs.sort_unstable();
        xs.dedup();
        assert!(!xs.is_empty());
        let n = xs.len().next_power_of_two();
        let last = *xs.last().unwrap();
        xs.resize(n, last); // 末尾を複製してサイズを 2 冪に
        Self { n, xs, node: vec![None; 2 * n] }
    }

    fn add_inner(&mut self, mut new: Line, mut k: usize, mut l: usize, mut r: usize) {
        loop {
            let cur = match self.node[k] {
                None => {
                    self.node[k] = Some(new);
                    return;
                }
                Some(cur) => cur,
            };
            let m = (l + r) / 2;
            let left_better = new.eval(self.xs[l]) < cur.eval(self.xs[l]);
            let mid_better = new.eval(self.xs[m]) < cur.eval(self.xs[m]);
            if mid_better {
                self.node[k] = Some(new);
                new = cur;
            }
            if r - l == 1 {
                return;
            }
            // new が優位な側の半区間にだけ降りる
            if left_better != mid_better {
                k = 2 * k;
                r = m;
            } else {
                k = 2 * k + 1;
                l = m;
            }
        }
    }

    /// 直線 y = a*x + b を追加する。
    pub fn add_line(&mut self, a: i64, b: i64) {
        let n = self.n;
        self.add_inner(Line { a, b }, 1, 0, n);
    }

    /// x ∈ [xl, xr) に制限した線分 y = a*x + b を追加する（x は値、添字ではない）。
    pub fn add_segment(&mut self, xl: i64, xr: i64, a: i64, b: i64) {
        let mut l = self.xs.partition_point(|&x| x < xl);
        let mut r = self.xs.partition_point(|&x| x < xr);
        // セグ木の区間分解で降ろす
        l += self.n;
        r += self.n;
        let (mut sl, mut sr) = (l - self.n, r - self.n);
        let mut len = 1;
        while l < r {
            if l & 1 == 1 {
                self.add_inner(Line { a, b }, l, sl, sl + len);
                l += 1;
                sl += len;
            }
            if r & 1 == 1 {
                r -= 1;
                sr -= len;
                self.add_inner(Line { a, b }, r, sr, sr + len);
            }
            l >>= 1;
            r >>= 1;
            len <<= 1;
        }
    }

    /// x での最小値。x は new に渡した候補のいずれかであること。直線が無ければ None。
    pub fn query(&self, x: i64) -> i64 {
        self.try_query(x).expect("LiChaoTree: no line covers x")
    }

    pub fn try_query(&self, x: i64) -> Option<i64> {
        let i = self.xs.partition_point(|&v| v < x);
        assert!(i < self.n && self.xs[i] == x, "LiChaoTree: x not in candidates");
        let mut k = i + self.n;
        let mut best: Option<i64> = None;
        while k >= 1 {
            if let Some(line) = self.node[k] {
                let v = line.eval(x);
                best = Some(best.map_or(v, |b: i64| b.min(v)));
            }
            k >>= 1;
        }
        best
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::misc::xorshift::XorShift;

    #[test]
    fn test_lines_random() {
        let mut rng = XorShift::new(42);
        for _ in 0..50 {
            let m = 1 + rng.next_range(20) as usize;
            let xs: Vec<i64> = (0..m).map(|_| rng.next_range(2001) as i64 - 1000).collect();
            let mut lct = LiChaoTree::new(&xs);
            let mut lines: Vec<(i64, i64)> = vec![];
            for _ in 0..30 {
                let a = rng.next_range(201) as i64 - 100;
                let b = rng.next_range(20001) as i64 - 10000;
                lct.add_line(a, b);
                lines.push((a, b));
                for &x in &xs {
                    let naive = lines.iter().map(|&(a, b)| a * x + b).min().unwrap();
                    assert_eq!(lct.query(x), naive);
                }
            }
        }
    }

    #[test]
    fn test_segments_random() {
        let mut rng = XorShift::new(777);
        for _ in 0..50 {
            let m = 2 + rng.next_range(15) as usize;
            let xs: Vec<i64> = (0..m).map(|_| rng.next_range(201) as i64 - 100).collect();
            let mut lct = LiChaoTree::new(&xs);
            let mut segs: Vec<(i64, i64, i64, i64)> = vec![];
            for _ in 0..30 {
                let a = rng.next_range(21) as i64 - 10;
                let b = rng.next_range(201) as i64 - 100;
                let mut xl = rng.next_range(241) as i64 - 120;
                let mut xr = rng.next_range(241) as i64 - 120;
                if xl > xr {
                    std::mem::swap(&mut xl, &mut xr);
                }
                lct.add_segment(xl, xr, a, b);
                segs.push((xl, xr, a, b));
                for &x in &xs {
                    let naive = segs
                        .iter()
                        .filter(|&&(l, r, _, _)| l <= x && x < r)
                        .map(|&(_, _, a, b)| a * x + b)
                        .min();
                    assert_eq!(lct.try_query(x), naive);
                }
            }
        }
    }
}
