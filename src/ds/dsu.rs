//! Union-Find（DSU, ACL 互換の API）。
//!
//! ```
//! use cplib::ds::dsu::*;
//! let mut d = Dsu::new(5);
//! d.merge(0, 1);
//! assert!(d.same(0, 1));
//! assert_eq!(d.size(0), 2);
//! ```

pub struct Dsu {
    n: usize,
    /// 根なら -(サイズ)、非根なら親
    parent_or_size: Vec<i32>,
}

impl Dsu {
    pub fn new(n: usize) -> Self {
        Dsu {
            n,
            parent_or_size: vec![-1; n],
        }
    }
    /// a と b を併合し、併合後の代表を返す
    pub fn merge(&mut self, a: usize, b: usize) -> usize {
        assert!(a < self.n && b < self.n);
        let (mut x, mut y) = (self.leader(a), self.leader(b));
        if x == y {
            return x;
        }
        if -self.parent_or_size[x] < -self.parent_or_size[y] {
            std::mem::swap(&mut x, &mut y);
        }
        self.parent_or_size[x] += self.parent_or_size[y];
        self.parent_or_size[y] = x as i32;
        x
    }
    pub fn same(&mut self, a: usize, b: usize) -> bool {
        self.leader(a) == self.leader(b)
    }
    pub fn leader(&mut self, a: usize) -> usize {
        assert!(a < self.n);
        if self.parent_or_size[a] < 0 {
            return a;
        }
        let p = self.parent_or_size[a] as usize;
        let root = self.leader(p);
        self.parent_or_size[a] = root as i32;
        root
    }
    pub fn size(&mut self, a: usize) -> usize {
        let l = self.leader(a);
        -self.parent_or_size[l] as usize
    }
    /// 連結成分ごとの頂点リスト
    pub fn groups(&mut self) -> Vec<Vec<usize>> {
        let mut leader = vec![0usize; self.n];
        for i in 0..self.n {
            leader[i] = self.leader(i);
        }
        let mut res: Vec<Vec<usize>> = vec![vec![]; self.n];
        for i in 0..self.n {
            res[leader[i]].push(i);
        }
        res.retain(|v| !v.is_empty());
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic() {
        let mut d = Dsu::new(6);
        d.merge(0, 1);
        d.merge(1, 2);
        d.merge(3, 4);
        assert!(d.same(0, 2));
        assert!(!d.same(0, 3));
        assert_eq!(d.size(0), 3);
        assert_eq!(d.size(3), 2);
        assert_eq!(d.size(5), 1);
        assert_eq!(d.groups().len(), 3);
    }
}
