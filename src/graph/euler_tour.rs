//! オイラーツアー（部分木を連続区間に対応づける）。反復 DFS でスタック安全。
//!
//! ```
//! use cplib::graph::euler_tour::*;
//! let adj = vec![vec![1, 2], vec![0, 3], vec![0], vec![1]];
//! let et = EulerTour::new(&adj, 0);
//! // 部分木 1 は {1,3}
//! let (l, r) = et.subtree(1);
//! assert_eq!(r - l, 2);
//! assert!(et.is_ancestor(0, 3));
//! assert!(!et.is_ancestor(2, 3));
//! ```

pub struct EulerTour {
    /// tin[v]: 行きがけ順（部分木区間の左端）
    pub tin: Vec<usize>,
    /// tout[v]: 部分木区間の右端（半開）
    pub tout: Vec<usize>,
    pub depth: Vec<u32>,
    pub order: Vec<usize>, // tin 順に並べた頂点
}

impl EulerTour {
    pub fn new(adj: &[Vec<usize>], root: usize) -> Self {
        let n = adj.len();
        let mut tin = vec![0usize; n];
        let mut tout = vec![0usize; n];
        let mut depth = vec![0u32; n];
        let mut order = Vec::with_capacity(n);
        let mut timer = 0usize;
        // (v, parent, child index)
        let mut stack: Vec<(usize, usize, usize)> = vec![(root, usize::MAX, 0)];
        tin[root] = 0;
        // 反復 DFS
        while let Some(&mut (v, p, ref mut i)) = stack.last_mut() {
            if *i == 0 {
                tin[v] = timer;
                order.push(v);
                timer += 1;
            }
            if *i < adj[v].len() {
                let to = adj[v][*i];
                *i += 1;
                if to != p {
                    depth[to] = depth[v] + 1;
                    stack.push((to, v, 0));
                }
            } else {
                tout[v] = timer;
                stack.pop();
            }
        }
        EulerTour {
            tin,
            tout,
            depth,
            order,
        }
    }

    /// v の部分木に対応する半開区間 [l, r)
    pub fn subtree(&self, v: usize) -> (usize, usize) {
        (self.tin[v], self.tout[v])
    }

    /// u は v の祖先か（u == v も真）
    pub fn is_ancestor(&self, u: usize, v: usize) -> bool {
        self.tin[u] <= self.tin[v] && self.tout[v] <= self.tout[u]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn subtree_ranges() {
        // 0-1-3, 0-2, 1-4
        let adj = vec![vec![1, 2], vec![0, 3, 4], vec![0], vec![1], vec![1]];
        let et = EulerTour::new(&adj, 0);
        assert_eq!(et.subtree(0), (0, 5));
        let (l1, r1) = et.subtree(1);
        assert_eq!(r1 - l1, 3); // {1,3,4}
        assert_eq!(et.subtree(3).1 - et.subtree(3).0, 1);
        assert!(et.is_ancestor(0, 4));
        assert!(et.is_ancestor(1, 3));
        assert!(!et.is_ancestor(2, 4));
        assert!(et.is_ancestor(3, 3));
    }
}
