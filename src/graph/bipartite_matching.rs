//! 二部グラフ最大マッチング（Kuhn の増加路法）。
//!
//! ```
//! use cplib::graph::bipartite_matching::*;
//! let mut bm = BipartiteMatching::new(3, 3);
//! bm.add_edge(0, 0);
//! bm.add_edge(0, 1);
//! bm.add_edge(1, 0);
//! bm.add_edge(2, 2);
//! assert_eq!(bm.max_matching(), 3);
//! ```

pub struct BipartiteMatching {
    nl: usize,
    nr: usize,
    g: Vec<Vec<usize>>,
    match_l: Vec<i64>,
    match_r: Vec<i64>,
}

impl BipartiteMatching {
    pub fn new(nl: usize, nr: usize) -> Self {
        BipartiteMatching {
            nl,
            nr,
            g: vec![vec![]; nl],
            match_l: vec![-1; nl],
            match_r: vec![-1; nr],
        }
    }

    /// 左頂点 l と右頂点 r を結ぶ辺。
    pub fn add_edge(&mut self, l: usize, r: usize) {
        assert!(l < self.nl && r < self.nr);
        self.g[l].push(r);
    }

    fn augment(&mut self, l: usize, used: &mut [bool]) -> bool {
        for i in 0..self.g[l].len() {
            let r = self.g[l][i];
            if used[r] {
                continue;
            }
            used[r] = true;
            if self.match_r[r] == -1 || self.augment(self.match_r[r] as usize, used) {
                self.match_l[l] = r as i64;
                self.match_r[r] = l as i64;
                return true;
            }
        }
        false
    }

    /// 最大マッチングのサイズ。呼び出し後 `matches()` で対応を取得可。
    pub fn max_matching(&mut self) -> usize {
        self.match_l.iter_mut().for_each(|x| *x = -1);
        self.match_r.iter_mut().for_each(|x| *x = -1);
        let mut res = 0;
        for l in 0..self.nl {
            let mut used = vec![false; self.nr];
            if self.augment(l, &mut used) {
                res += 1;
            }
        }
        res
    }

    /// `(l, r)` のマッチング対の一覧。
    pub fn matches(&self) -> Vec<(usize, usize)> {
        (0..self.nl)
            .filter(|&l| self.match_l[l] >= 0)
            .map(|l| (l, self.match_l[l] as usize))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn perfect() {
        let mut bm = BipartiteMatching::new(3, 3);
        for &(l, r) in &[(0, 0), (0, 1), (1, 0), (1, 2), (2, 1)] {
            bm.add_edge(l, r);
        }
        assert_eq!(bm.max_matching(), 3);
        assert_eq!(bm.matches().len(), 3);
    }
    #[test]
    fn limited() {
        let mut bm = BipartiteMatching::new(3, 2);
        bm.add_edge(0, 0);
        bm.add_edge(1, 0);
        bm.add_edge(2, 0);
        assert_eq!(bm.max_matching(), 1);
    }
}
