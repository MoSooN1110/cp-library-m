//! トポロジカルソート（Kahn 法）。閉路があれば `None`。
//!
//! ```
//! use cplib::graph::topo_sort::*;
//! let adj = vec![vec![1, 2], vec![3], vec![3], vec![]];
//! let order = topo_sort(&adj).unwrap();
//! // 0 が 1,2 より前、1,2 が 3 より前
//! let pos: Vec<usize> = {
//!     let mut p = vec![0; 4];
//!     for (i, &v) in order.iter().enumerate() { p[v] = i; }
//!     p
//! };
//! assert!(pos[0] < pos[1] && pos[1] < pos[3]);
//! ```
use std::collections::VecDeque;

pub fn topo_sort(adj: &[Vec<usize>]) -> Option<Vec<usize>> {
    let n = adj.len();
    let mut indeg = vec![0usize; n];
    for u in 0..n {
        for &v in &adj[u] {
            indeg[v] += 1;
        }
    }
    let mut q: VecDeque<usize> = (0..n).filter(|&v| indeg[v] == 0).collect();
    let mut order = Vec::with_capacity(n);
    while let Some(v) = q.pop_front() {
        order.push(v);
        for &to in &adj[v] {
            indeg[to] -= 1;
            if indeg[to] == 0 {
                q.push_back(to);
            }
        }
    }
    if order.len() == n {
        Some(order)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn valid_order() {
        let adj = vec![vec![1, 2], vec![3], vec![3], vec![4], vec![]];
        let order = topo_sort(&adj).unwrap();
        let mut pos = vec![0usize; 5];
        for (i, &v) in order.iter().enumerate() {
            pos[v] = i;
        }
        for u in 0..5 {
            for &v in &adj[u] {
                assert!(pos[u] < pos[v]);
            }
        }
    }
    #[test]
    fn cycle_none() {
        let adj = vec![vec![1], vec![2], vec![0]];
        assert!(topo_sort(&adj).is_none());
    }
}
