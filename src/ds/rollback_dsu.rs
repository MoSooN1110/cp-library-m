//! Rollback 可能 Union-Find（サイズ併合・経路圧縮なし・undo O(1)）。
//! オフライン動的連結性や「辺を時系列で足して巻き戻す」問題の部品。
//!
//! ```
//! use cplib::ds::rollback_dsu::*;
//! let mut d = RollbackDsu::new(4);
//! d.merge(0, 1);
//! let snap = d.snapshot();
//! d.merge(2, 3);
//! d.merge(0, 2);
//! assert!(d.same(1, 3));
//! d.rollback(snap);
//! assert!(!d.same(1, 3) && d.same(0, 1));
//! ```

pub struct RollbackDsu {
    // 負ならその絶対値が成分サイズ（根）、非負なら親
    parent_or_size: Vec<i64>,
    // (書き換えた添字, 元の値)。merge 1 回で高々 2 エントリ
    history: Vec<(usize, i64)>,
    components: usize,
}

impl RollbackDsu {
    pub fn new(n: usize) -> Self {
        Self { parent_or_size: vec![-1; n], history: vec![], components: n }
    }

    /// 経路圧縮しない root（O(log n)、サイズ併合のため木高は log で抑えられる）。
    pub fn leader(&self, mut a: usize) -> usize {
        while self.parent_or_size[a] >= 0 {
            a = self.parent_or_size[a] as usize;
        }
        a
    }

    pub fn same(&self, a: usize, b: usize) -> bool {
        self.leader(a) == self.leader(b)
    }

    pub fn size(&self, a: usize) -> usize {
        (-self.parent_or_size[self.leader(a)]) as usize
    }

    pub fn components(&self) -> usize {
        self.components
    }

    /// 併合。すでに同成分でも履歴は 1 段消費する（undo と対にするため）。
    /// 返り値は「新たに併合が起きたか」。
    pub fn merge(&mut self, a: usize, b: usize) -> bool {
        let (mut x, mut y) = (self.leader(a), self.leader(b));
        if x == y {
            self.history.push((usize::MAX, 0)); // no-op マーカー
            return false;
        }
        if self.parent_or_size[x] > self.parent_or_size[y] {
            std::mem::swap(&mut x, &mut y);
        }
        self.history.push((x, self.parent_or_size[x]));
        self.history.push((y, self.parent_or_size[y]));
        self.parent_or_size[x] += self.parent_or_size[y];
        self.parent_or_size[y] = x as i64;
        self.components -= 1;
        true
    }

    /// 直前の merge を 1 回取り消す。
    pub fn undo(&mut self) {
        let (i, v) = self.history.pop().expect("RollbackDsu: nothing to undo");
        if i == usize::MAX {
            return; // no-op merge
        }
        self.parent_or_size[i] = v;
        let (j, w) = self.history.pop().unwrap();
        self.parent_or_size[j] = w;
        self.components += 1;
    }

    /// 現時点の履歴位置。rollback(snapshot()) でこの状態に戻せる。
    pub fn snapshot(&self) -> usize {
        self.history.len()
    }

    /// merge 履歴が snap になるまで undo を繰り返す。
    pub fn rollback(&mut self, snap: usize) {
        while self.history.len() > snap {
            self.undo();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ds::dsu::Dsu;
    use crate::misc::xorshift::XorShift;

    #[test]
    fn test_random_vs_dsu_with_rollback() {
        let mut rng = XorShift::new(123);
        for _ in 0..50 {
            let n = 2 + rng.next_range(20) as usize;
            let mut rb = RollbackDsu::new(n);
            // 操作列を積んでは途中まで巻き戻し、その都度 Dsu を作り直して照合する
            let mut ops: Vec<(usize, usize)> = vec![];
            let mut snaps: Vec<usize> = vec![]; // snaps[i] = i 回の merge 後の snapshot
            for _ in 0..100 {
                if !ops.is_empty() && rng.next_range(3) == 0 {
                    let keep = rng.next_range(ops.len() as u64 + 1) as usize;
                    rb.rollback(if keep == 0 { 0 } else { snaps[keep - 1] });
                    ops.truncate(keep);
                    snaps.truncate(keep);
                } else {
                    let a = rng.next_range(n as u64) as usize;
                    let b = rng.next_range(n as u64) as usize;
                    rb.merge(a, b);
                    ops.push((a, b));
                    snaps.push(rb.snapshot());
                }
                let mut d = Dsu::new(n);
                for &(a, b) in &ops {
                    d.merge(a, b);
                }
                let mut comp = 0;
                for v in 0..n {
                    if d.leader(v) == v {
                        comp += 1;
                    }
                }
                assert_eq!(rb.components(), comp);
                for u in 0..n {
                    assert_eq!(rb.size(u), d.size(u));
                    for v in 0..n {
                        assert_eq!(rb.same(u, v), d.same(u, v));
                    }
                }
            }
        }
    }
}
