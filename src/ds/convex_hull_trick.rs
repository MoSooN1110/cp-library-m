//! Convex Hull Trick（最小値クエリ）。直線を傾き降順で追加、任意の x を三分探索で照会。
//!
//! ```
//! use cplib::ds::convex_hull_trick::*;
//! let mut cht = ConvexHullTrick::new();
//! // 傾きは降順で追加する
//! cht.add_line(2, 3);
//! cht.add_line(1, 0);
//! cht.add_line(-1, 5);
//! assert_eq!(cht.query(0), 0);   // min(3, 0, 5)
//! assert_eq!(cht.query(4), 1);   // min(11, 4, 1)
//! ```

/// 最小値用 CHT。`add_line` は傾き a を **非増加** の順で呼ぶこと。
#[derive(Default)]
pub struct ConvexHullTrick {
    a: Vec<i64>,
    b: Vec<i64>,
}

impl ConvexHullTrick {
    pub fn new() -> Self {
        ConvexHullTrick { a: vec![], b: vec![] }
    }

    // l3 があるとき l2 が不要か（すべて i64、min 用の下側凸包）
    fn unnecessary(&self, l1: usize, l2: usize, a3: i64, b3: i64) -> bool {
        // l2 は不要 ⇔ (b2 - b1)*(a1 - a3) >= (b3 - b1)*(a1 - a2)
        let (a1, b1, a2, b2) = (self.a[l1], self.b[l1], self.a[l2], self.b[l2]);
        (b2 - b1) as i128 * (a1 - a3) as i128 >= (b3 - b1) as i128 * (a1 - a2) as i128
    }

    /// 直線 y = a*x + b を追加（a は非増加の順）。
    pub fn add_line(&mut self, a: i64, b: i64) {
        // 同じ傾きなら b が小さい方だけ残す
        if let Some(&la) = self.a.last() {
            if la == a {
                if *self.b.last().unwrap() <= b {
                    return;
                }
                self.a.pop();
                self.b.pop();
            }
        }
        while self.a.len() >= 2 {
            let n = self.a.len();
            if self.unnecessary(n - 2, n - 1, a, b) {
                self.a.pop();
                self.b.pop();
            } else {
                break;
            }
        }
        self.a.push(a);
        self.b.push(b);
    }

    #[inline]
    fn f(&self, i: usize, x: i64) -> i64 {
        self.a[i] * x + self.b[i]
    }

    /// x における最小値。
    pub fn query(&self, x: i64) -> i64 {
        assert!(!self.a.is_empty());
        // f(i)=a_i*x+b_i は i について凸（下側包絡）→ 凸列の最小を二分探索
        let (mut lo, mut hi) = (0usize, self.a.len() - 1);
        while lo < hi {
            let mid = (lo + hi) / 2;
            if self.f(mid, x) <= self.f(mid + 1, x) {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        self.f(lo, x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn matches_brute() {
        // 傾き降順で追加
        let lines = [(5i64, 1i64), (3, -2), (2, 4), (0, -1), (-2, 3), (-4, 10)];
        let mut cht = ConvexHullTrick::new();
        for &(a, b) in &lines {
            cht.add_line(a, b);
        }
        for x in -50..=50i64 {
            let brute = lines.iter().map(|&(a, b)| a * x + b).min().unwrap();
            assert_eq!(cht.query(x), brute, "x={x}");
        }
    }
}
