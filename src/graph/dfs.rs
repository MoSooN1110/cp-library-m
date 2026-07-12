//! 非再帰 DFS。
//!
//! ```rust
//! use cplib::graph::dfs::*;
//!
//! let g = vec![vec![1, 2], vec![3], vec![], vec![]];
//! assert_eq!(dfs_order(&g, 0), vec![0, 1, 3, 2]);
//! assert_eq!(dfs_reachable(&g, 0), vec![true, true, true, true]);
//! ```

pub fn dfs_order(adj: &[Vec<usize>], start: usize) -> Vec<usize> {
    assert!(start < adj.len());
    let mut visited = vec![false; adj.len()];
    let mut order = Vec::new();
    let mut stack = vec![start];
    while let Some(v) = stack.pop() {
        if visited[v] {
            continue;
        }
        visited[v] = true;
        order.push(v);
        for &to in adj[v].iter().rev() {
            if !visited[to] {
                stack.push(to);
            }
        }
    }
    order
}

pub fn dfs_reachable(adj: &[Vec<usize>], start: usize) -> Vec<bool> {
    assert!(start < adj.len());
    let mut visited = vec![false; adj.len()];
    let mut stack = vec![start];
    while let Some(v) = stack.pop() {
        if visited[v] {
            continue;
        }
        visited[v] = true;
        for &to in &adj[v] {
            if !visited[to] {
                stack.push(to);
            }
        }
    }
    visited
}

pub fn dfs_order_weighted<W>(adj: &[Vec<(usize, W)>], start: usize) -> Vec<usize> {
    assert!(start < adj.len());
    let mut visited = vec![false; adj.len()];
    let mut order = Vec::new();
    let mut stack = vec![start];
    while let Some(v) = stack.pop() {
        if visited[v] {
            continue;
        }
        visited[v] = true;
        order.push(v);
        for &(to, _) in adj[v].iter().rev() {
            if !visited[to] {
                stack.push(to);
            }
        }
    }
    order
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ds::dsu::Dsu;

    fn add_undirected(adj: &mut [Vec<usize>], a: usize, b: usize) {
        adj[a].push(b);
        adj[b].push(a);
    }

    #[test]
    fn order_and_reachable() {
        let g = vec![vec![1, 2], vec![3], vec![], vec![]];
        assert_eq!(dfs_order(&g, 0), vec![0, 1, 3, 2]);
        assert_eq!(dfs_reachable(&g, 0), vec![true, true, true, true]);
        let g = vec![vec![1], vec![], vec![3], vec![]];
        assert_eq!(dfs_order(&g, 2), vec![2, 3]);
        assert_eq!(dfs_reachable(&g, 0), vec![true, true, false, false]);
    }

    #[test]
    fn weighted_ignores_weights() {
        let g = vec![
            vec![(1usize, 10i64), (2, 20)],
            vec![(3, 30)],
            vec![],
            vec![],
        ];
        assert_eq!(dfs_order_weighted(&g, 0), vec![0, 1, 3, 2]);
    }

    #[test]
    fn random_reachability_matches_dsu_on_undirected_graph() {
        let mut seed = 192837465u64;
        let mut rng = || {
            seed ^= seed << 7;
            seed ^= seed >> 9;
            seed
        };
        for _ in 0..300 {
            let n = 1 + rng() as usize % 40;
            let mut adj = vec![vec![]; n];
            let mut dsu = Dsu::new(n);
            for i in 0..n {
                for j in i + 1..n {
                    if rng() % 100 < 10 {
                        add_undirected(&mut adj, i, j);
                        dsu.merge(i, j);
                    }
                }
            }
            let s = rng() as usize % n;
            let reachable = dfs_reachable(&adj, s);
            for v in 0..n {
                assert_eq!(reachable[v], dsu.same(s, v), "{n} {s} {v}");
            }
        }
    }
}
