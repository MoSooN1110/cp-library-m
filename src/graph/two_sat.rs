//! 2-SAT（強連結成分分解による充足判定と割り当て構成）。`graph::scc` に依存。
//!
//! 変数 i (0-indexed) のリテラルは `(i, true)`=x_i, `(i, false)`=¬x_i で表す。
//!
//! ```
//! use cplib::graph::two_sat::*;
//! // (x0 ∨ x1) ∧ (¬x0 ∨ x1) ∧ (¬x1 ∨ x0 の否定はなし)
//! let mut ts = TwoSat::new(2);
//! ts.add_clause(0, true, 1, true);
//! ts.add_clause(0, false, 1, true);
//! let ans = ts.solve().unwrap();
//! assert!(ans[1]); // どの解でも x1 = true
//! ```

use crate::graph::scc::SccGraph;

pub struct TwoSat {
    n: usize,
    g: SccGraph,
}

impl TwoSat {
    pub fn new(n: usize) -> Self {
        Self { n, g: SccGraph::new(2 * n) }
    }

    /// 節 (x_i = f) ∨ (x_j = g) を追加する。
    pub fn add_clause(&mut self, i: usize, f: bool, j: usize, g: bool) {
        assert!(i < self.n && j < self.n);
        // ¬(x_i = f) → (x_j = g), ¬(x_j = g) → (x_i = f)
        self.g.add_edge(2 * i + if f { 0 } else { 1 }, 2 * j + if g { 1 } else { 0 });
        self.g.add_edge(2 * j + if g { 0 } else { 1 }, 2 * i + if f { 1 } else { 0 });
    }

    /// x_i = f を強制する（単項節）。
    pub fn add_unit(&mut self, i: usize, f: bool) {
        self.add_clause(i, f, i, f);
    }

    /// 充足可能なら各変数の割り当てを返す。不能なら None。
    pub fn solve(&self) -> Option<Vec<bool>> {
        let groups = self.g.scc();
        let mut id = vec![0usize; 2 * self.n];
        for (k, group) in groups.iter().enumerate() {
            for &v in group {
                id[v] = k;
            }
        }
        let mut ans = vec![false; self.n];
        for i in 0..self.n {
            if id[2 * i] == id[2 * i + 1] {
                return None;
            }
            // 頂点 2i+1 が x_i=true。トポロジカル順で後ろにある側を採用
            ans[i] = id[2 * i] < id[2 * i + 1];
        }
        Some(ans)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(clauses: &[(usize, bool, usize, bool)], ans: &[bool]) -> bool {
        clauses.iter().all(|&(i, f, j, g)| ans[i] == f || ans[j] == g)
    }

    #[test]
    fn test_unsat() {
        let mut ts = TwoSat::new(1);
        ts.add_unit(0, true);
        ts.add_unit(0, false);
        assert!(ts.solve().is_none());
    }

    #[test]
    fn test_implication_chain() {
        // x0→x1→x2 かつ x0 強制
        let mut ts = TwoSat::new(3);
        ts.add_clause(0, false, 1, true);
        ts.add_clause(1, false, 2, true);
        ts.add_unit(0, true);
        let ans = ts.solve().unwrap();
        assert_eq!(ans, vec![true, true, true]);
    }

    #[test]
    fn test_random_vs_bruteforce() {
        use crate::misc::xorshift::XorShift;
        let mut rng = XorShift::new(998244353);
        for _ in 0..300 {
            let n = 1 + rng.next_range(5) as usize;
            let m = rng.next_range(8) as usize;
            let mut ts = TwoSat::new(n);
            let mut clauses = vec![];
            for _ in 0..m {
                let i = rng.next_range(n as u64) as usize;
                let j = rng.next_range(n as u64) as usize;
                let f = rng.next_range(2) == 0;
                let g = rng.next_range(2) == 0;
                ts.add_clause(i, f, j, g);
                clauses.push((i, f, j, g));
            }
            let brute_sat = (0..1u32 << n).any(|mask| {
                let ans: Vec<bool> = (0..n).map(|k| mask >> k & 1 == 1).collect();
                check(&clauses, &ans)
            });
            match ts.solve() {
                Some(ans) => {
                    assert!(brute_sat);
                    assert!(check(&clauses, &ans));
                }
                None => assert!(!brute_sat),
            }
        }
    }
}
