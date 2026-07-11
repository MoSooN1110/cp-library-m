//! 木の直径（重みつき、2 回 DFS）。反復 DFS でスタック安全。
//!
//! ```
//! use cplib::graph::tree_diameter::*;
//! // 0-1(3), 1-2(4), 1-3(1)
//! let adj = vec![
//!     vec![(1usize, 3u64)],
//!     vec![(0, 3), (2, 4), (3, 1)],
//!     vec![(1, 4)],
//!     vec![(1, 1)],
//! ];
//! let (d, path) = tree_diameter(&adj);
//! assert_eq!(d, 7);                 // 端点 {0, 2}
//! let mut ends = vec![path[0], *path.last().unwrap()];
//! ends.sort();
//! assert_eq!(ends, vec![0, 2]);
//! ```

fn farthest(adj: &[Vec<(usize, u64)>], s: usize) -> (usize, Vec<u64>, Vec<usize>) {
    let n = adj.len();
    let mut dist = vec![u64::MAX; n];
    let mut par = vec![usize::MAX; n];
    dist[s] = 0;
    let mut stack = vec![s];
    while let Some(v) = stack.pop() {
        for &(to, w) in &adj[v] {
            if dist[to] == u64::MAX {
                dist[to] = dist[v] + w;
                par[to] = v;
                stack.push(to);
            }
        }
    }
    let mut far = s;
    for v in 0..n {
        if dist[v] != u64::MAX && dist[v] > dist[far] {
            far = v;
        }
    }
    (far, dist, par)
}

/// `adj[v]` = `(to, weight)`（無向木）。`(直径長, 直径をなす頂点列)` を返す。
pub fn tree_diameter(adj: &[Vec<(usize, u64)>]) -> (u64, Vec<usize>) {
    if adj.is_empty() {
        return (0, vec![]);
    }
    let (u, _, _) = farthest(adj, 0);
    let (v, dist, par) = farthest(adj, u);
    let mut path = vec![v];
    let mut cur = v;
    while par[cur] != usize::MAX {
        cur = par[cur];
        path.push(cur);
    }
    path.reverse();
    (dist[v], path)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic() {
        // パス 0-1-2-3 各重み 2、分岐 1-4 重み 1
        let adj = vec![
            vec![(1usize, 2u64)],
            vec![(0, 2), (2, 2), (4, 1)],
            vec![(1, 2), (3, 2)],
            vec![(2, 2)],
            vec![(1, 1)],
        ];
        let (d, path) = tree_diameter(&adj);
        assert_eq!(d, 6); // 0-1-2-3
        assert_eq!(path.len(), 4);
        let mut ends = vec![path[0], *path.last().unwrap()];
        ends.sort();
        assert_eq!(ends, vec![0, 3]);
    }
    #[test]
    fn single_node() {
        let adj = vec![vec![]];
        assert_eq!(tree_diameter(&adj), (0, vec![0]));
    }
}
