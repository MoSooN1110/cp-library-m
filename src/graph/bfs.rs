//! BFS（重みなしグラフの単一始点最短路 = 辺数）。
//!
//! ```
//! use cplib::graph::bfs::*;
//! let g = vec![vec![1, 2], vec![0, 3], vec![0], vec![1]];
//! let d = bfs(&g, 0);
//! assert_eq!(d[3], 2);
//! ```
use std::collections::VecDeque;

pub const INF: u32 = u32::MAX;

/// `adj[v]` = 隣接頂点リスト。到達不能は `INF`。
pub fn bfs(adj: &[Vec<usize>], s: usize) -> Vec<u32> {
    let n = adj.len();
    let mut dist = vec![INF; n];
    let mut q = VecDeque::new();
    dist[s] = 0;
    q.push_back(s);
    while let Some(v) = q.pop_front() {
        for &to in &adj[v] {
            if dist[to] == INF {
                dist[to] = dist[v] + 1;
                q.push_back(to);
            }
        }
    }
    dist
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic() {
        let g = vec![vec![1, 2], vec![0, 3], vec![0, 3], vec![1, 2, 4], vec![3]];
        let d = bfs(&g, 0);
        assert_eq!(d, vec![0, 1, 1, 2, 3]);
    }
}
