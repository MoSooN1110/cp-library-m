//! 重み付き Union-Find（ポテンシャル/差分制約）。
//! `weight[b] - weight[a] = w` を管理し、同一成分内の差を問い合わせる。
//!
//! ```
//! use cplib::ds::weighted_dsu::*;
//! let mut d = WeightedDsu::new(4);
//! d.merge(0, 1, 3);   // w[1]-w[0]=3
//! d.merge(1, 2, 2);   // w[2]-w[1]=2
//! assert_eq!(d.diff(0, 2), Some(5));
//! assert_eq!(d.diff(0, 3), None);     // 別成分
//! ```

pub struct WeightedDsu {
    par: Vec<i64>,   // 根なら -(size)、非根なら親
    diff: Vec<i64>,  // 親からのポテンシャル差 weight[self]-weight[par]
}

impl WeightedDsu {
    pub fn new(n: usize) -> Self {
        WeightedDsu {
            par: vec![-1; n],
            diff: vec![0; n],
        }
    }

    pub fn leader(&mut self, x: usize) -> usize {
        if self.par[x] < 0 {
            return x;
        }
        let p = self.par[x] as usize;
        let r = self.leader(p);
        self.diff[x] += self.diff[p];
        self.par[x] = r as i64;
        r
    }

    /// 根からの相対ポテンシャル
    fn potential(&mut self, x: usize) -> i64 {
        self.leader(x);
        self.diff[x]
    }

    pub fn same(&mut self, a: usize, b: usize) -> bool {
        self.leader(a) == self.leader(b)
    }

    /// weight[b] - weight[a] = w となるよう併合。既存関係と整合すれば true、矛盾なら false。
    pub fn merge(&mut self, a: usize, b: usize, w: i64) -> bool {
        let pa = self.potential(a);
        let pb = self.potential(b);
        let (mut ra, mut rb) = (self.leader(a), self.leader(b));
        if ra == rb {
            // 既に同一成分。整合性チェック。
            return pb - pa == w;
        }
        // weight[rb] - weight[ra] = w + pot(a) - pot(b)
        let mut d = w + pa - pb;
        // サイズが大きい方を根に（rb を ra の下にぶら下げる）
        if -self.par[ra] < -self.par[rb] {
            std::mem::swap(&mut ra, &mut rb);
            d = -d;
        }
        self.par[ra] += self.par[rb];
        self.par[rb] = ra as i64;
        self.diff[rb] = d;
        true
    }

    /// weight[b] - weight[a]。別成分なら None。
    pub fn diff(&mut self, a: usize, b: usize) -> Option<i64> {
        if self.same(a, b) {
            Some(self.potential(b) - self.potential(a))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic() {
        let mut d = WeightedDsu::new(6);
        assert!(d.merge(0, 1, 5));
        assert!(d.merge(1, 2, 3));
        assert!(d.merge(3, 4, 2));
        assert_eq!(d.diff(0, 2), Some(8));
        assert_eq!(d.diff(2, 0), Some(-8));
        assert_eq!(d.diff(0, 4), None);
        assert!(d.same(0, 2));
        assert!(!d.same(0, 3));
        // 既存関係と整合する併合は OK
        assert!(d.merge(0, 2, 8));
    }
}
