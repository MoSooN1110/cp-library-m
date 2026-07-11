//! 最小共通祖先（ダブリング）。距離・k 個上の祖先も取得可。
//!
//! ```
//! use cplib::graph::lca::*;
//! // 0-1, 0-2, 1-3, 1-4
//! let adj = vec![vec![1, 2], vec![0, 3, 4], vec![0], vec![1], vec![1]];
//! let lca = Lca::new(&adj, 0);
//! assert_eq!(lca.lca(3, 4), 1);
//! assert_eq!(lca.lca(3, 2), 0);
//! assert_eq!(lca.dist(3, 4), 2);
//! ```
use std::collections::VecDeque;

pub struct Lca {
    up: Vec<Vec<u32>>,
    depth: Vec<u32>,
    log: usize,
}

impl Lca {
    pub fn new(adj: &[Vec<usize>], root: usize) -> Self {
        let n = adj.len();
        let mut log = 1;
        while (1 << log) < n.max(1) {
            log += 1;
        }
        let mut up = vec![vec![root as u32; n]; log];
        let mut depth = vec![0u32; n];
        // BFS で親・深さ
        let mut visited = vec![false; n];
        let mut q = VecDeque::new();
        visited[root] = true;
        q.push_back(root);
        while let Some(v) = q.pop_front() {
            for &to in &adj[v] {
                if !visited[to] {
                    visited[to] = true;
                    up[0][to] = v as u32;
                    depth[to] = depth[v] + 1;
                    q.push_back(to);
                }
            }
        }
        for k in 1..log {
            for v in 0..n {
                up[k][v] = up[k - 1][up[k - 1][v] as usize];
            }
        }
        Lca { up, depth, log }
    }

    pub fn depth(&self, v: usize) -> usize {
        self.depth[v] as usize
    }

    /// v から k 個上の祖先（根を越えたら根に張り付く）
    pub fn kth_ancestor(&self, mut v: usize, k: usize) -> usize {
        for i in 0..self.log {
            if (k >> i) & 1 == 1 {
                v = self.up[i][v] as usize;
            }
        }
        v
    }

    pub fn lca(&self, mut u: usize, mut v: usize) -> usize {
        if self.depth[u] < self.depth[v] {
            std::mem::swap(&mut u, &mut v);
        }
        u = self.kth_ancestor(u, (self.depth[u] - self.depth[v]) as usize);
        if u == v {
            return u;
        }
        for i in (0..self.log).rev() {
            if self.up[i][u] != self.up[i][v] {
                u = self.up[i][u] as usize;
                v = self.up[i][v] as usize;
            }
        }
        self.up[0][u] as usize
    }

    /// 木上の 2 点間距離（辺数）
    pub fn dist(&self, u: usize, v: usize) -> usize {
        let w = self.lca(u, v);
        (self.depth[u] + self.depth[v] - 2 * self.depth[w]) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic() {
        // パス 0-1-2-3-4 + 分岐 2-5
        let adj = vec![
            vec![1],
            vec![0, 2],
            vec![1, 3, 5],
            vec![2, 4],
            vec![3],
            vec![2],
        ];
        let lca = Lca::new(&adj, 0);
        assert_eq!(lca.lca(4, 5), 2);
        assert_eq!(lca.lca(0, 4), 0);
        assert_eq!(lca.lca(3, 3), 3);
        assert_eq!(lca.dist(4, 5), 3);
        assert_eq!(lca.dist(0, 4), 4);
        assert_eq!(lca.kth_ancestor(4, 2), 2);
        assert_eq!(lca.kth_ancestor(4, 100), 0);
    }
}
