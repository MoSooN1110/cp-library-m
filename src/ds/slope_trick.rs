//! Slope Trick（区分線形凸関数の管理）。
//!
//! `f(x) += max(0, x-a)`, `f(x) += max(0, a-x)`, `f(x) += |x-a|`
//! といった操作と最小値取得を `O(log n)` で行う。
//!
//! ```
//! use cplib::ds::slope_trick::*;
//!
//! let mut f = SlopeTrick::new();
//! f.add_abs(3);
//! f.add_abs(7);
//! assert_eq!(f.min(), 4);
//! assert_eq!(f.argmin_interval(), (3, 7));
//! f.add_const(10);
//! assert_eq!(f.min(), 14);
//! ```

use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Clone, Debug)]
pub struct SlopeTrick {
    min_f: i64,
    left: BinaryHeap<i64>,
    right: BinaryHeap<Reverse<i64>>,
    add_l: i64,
    add_r: i64,
}

impl Default for SlopeTrick {
    fn default() -> Self {
        Self::new()
    }
}

impl SlopeTrick {
    pub fn new() -> Self {
        Self {
            min_f: 0,
            left: BinaryHeap::new(),
            right: BinaryHeap::new(),
            add_l: 0,
            add_r: 0,
        }
    }

    pub fn min(&self) -> i64 {
        self.min_f
    }

    /// 最小値を取る x の閉区間。
    ///
    /// 制約が片側だけの場合は `i64::MIN / 4`, `i64::MAX / 4` を番兵として返す。
    pub fn argmin_interval(&self) -> (i64, i64) {
        (self.top_left(), self.top_right())
    }

    /// f(x) += c
    pub fn add_const(&mut self, c: i64) {
        self.min_f += c;
    }

    /// f(x) += max(0, x - a)
    pub fn add_max_zero_x_minus_a(&mut self, a: i64) {
        let l = self.top_left();
        if l > a {
            self.min_f += l - a;
            self.pop_left();
            self.push_left(a);
            self.push_right(l);
        } else {
            self.push_right(a);
        }
    }

    /// f(x) += max(0, a - x)
    pub fn add_max_zero_a_minus_x(&mut self, a: i64) {
        let r = self.top_right();
        if r < a {
            self.min_f += a - r;
            self.pop_right();
            self.push_right(a);
            self.push_left(r);
        } else {
            self.push_left(a);
        }
    }

    /// f(x) += |x - a|
    pub fn add_abs(&mut self, a: i64) {
        self.add_max_zero_x_minus_a(a);
        self.add_max_zero_a_minus_x(a);
    }

    /// f(x) <- min_{y <= x} f(y)
    pub fn clear_right(&mut self) {
        self.right.clear();
    }

    /// f(x) <- min_{y >= x} f(y)
    pub fn clear_left(&mut self) {
        self.left.clear();
    }

    /// f(x) <- f(x - a)
    pub fn shift(&mut self, a: i64) {
        self.add_l += a;
        self.add_r += a;
    }

    /// f(x) <- min_{x-b <= y <= x-a} f(y)
    pub fn sliding_window_min(&mut self, a: i64, b: i64) {
        assert!(a <= b);
        self.add_l += a;
        self.add_r += b;
    }

    fn push_left(&mut self, x: i64) {
        self.left.push(x - self.add_l);
    }

    fn push_right(&mut self, x: i64) {
        self.right.push(Reverse(x - self.add_r));
    }

    fn top_left(&self) -> i64 {
        self.left
            .peek()
            .map(|&x| x + self.add_l)
            .unwrap_or(i64::MIN / 4)
    }

    fn top_right(&self) -> i64 {
        self.right
            .peek()
            .map(|&Reverse(x)| x + self.add_r)
            .unwrap_or(i64::MAX / 4)
    }

    fn pop_left(&mut self) -> Option<i64> {
        self.left.pop().map(|x| x + self.add_l)
    }

    fn pop_right(&mut self) -> Option<i64> {
        self.right.pop().map(|Reverse(x)| x + self.add_r)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INF: i64 = 1_i64 << 50;
    const OFFSET: i64 = 20;
    const WIDTH: usize = 41;

    fn idx(x: i64) -> usize {
        (x + OFFSET) as usize
    }

    fn val(a: &[i64], x: i64) -> i64 {
        a[idx(x)]
    }

    #[test]
    fn basic_abs() {
        let mut f = SlopeTrick::new();
        f.add_abs(3);
        assert_eq!(f.min(), 0);
        assert_eq!(f.argmin_interval(), (3, 3));
        f.add_abs(7);
        assert_eq!(f.min(), 4);
        assert_eq!(f.argmin_interval(), (3, 7));
        f.add_const(5);
        assert_eq!(f.min(), 9);
    }

    #[test]
    fn one_sided() {
        let mut f = SlopeTrick::new();
        f.add_max_zero_x_minus_a(5);
        assert_eq!(f.min(), 0);
        assert!(f.argmin_interval().0 < -1_000_000);
        assert_eq!(f.argmin_interval().1, 5);

        let mut g = SlopeTrick::new();
        g.add_max_zero_a_minus_x(-2);
        assert_eq!(g.min(), 0);
        assert_eq!(g.argmin_interval().0, -2);
        assert!(g.argmin_interval().1 > 1_000_000);
    }

    #[test]
    fn random_vs_brute_on_small_domain() {
        let mut st = SlopeTrick::new();
        let mut brute = vec![0i64; WIDTH];
        let mut seed = 123456789u64;
        let mut rng = || {
            seed ^= seed << 7;
            seed ^= seed >> 9;
            seed
        };

        for _ in 0..300 {
            let a = (rng() % 21) as i64 - 10;
            match rng() % 5 {
                0 => {
                    st.add_max_zero_x_minus_a(a);
                    for x in -OFFSET..=OFFSET {
                        brute[idx(x)] += (x - a).max(0);
                    }
                }
                1 => {
                    st.add_max_zero_a_minus_x(a);
                    for x in -OFFSET..=OFFSET {
                        brute[idx(x)] += (a - x).max(0);
                    }
                }
                2 => {
                    st.add_abs(a);
                    for x in -OFFSET..=OFFSET {
                        brute[idx(x)] += (x - a).abs();
                    }
                }
                3 => {
                    let c = (rng() % 11) as i64 - 5;
                    st.add_const(c);
                    for v in &mut brute {
                        *v += c;
                    }
                }
                _ => {
                    let d = (rng() % 7) as i64 - 3;
                    st.shift(d);
                    let old = brute.clone();
                    for x in -OFFSET..=OFFSET {
                        let y = x - d;
                        brute[idx(x)] = if (-OFFSET..=OFFSET).contains(&y) {
                            val(&old, y)
                        } else {
                            INF
                        };
                    }
                }
            }
            let expected = *brute.iter().min().unwrap();
            assert_eq!(st.min(), expected);
        }
    }

    #[test]
    fn sliding_window_min_matches_brute() {
        let mut st = SlopeTrick::new();
        st.add_abs(0);
        st.add_abs(4);
        st.sliding_window_min(-2, 3);

        let brute = (-10..=10)
            .map(|x| {
                (x - 3..=x + 2)
                    .map(|y: i64| y.abs() + (y - 4).abs())
                    .min()
                    .unwrap()
            })
            .min()
            .unwrap();
        assert_eq!(st.min(), brute);
    }
}
