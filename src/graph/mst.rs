//! 最小全域木（Kruskal 法）。DSU を再利用。
//!
//! ```
//! use cplib::graph::mst::*;
//! // (weight, u, v)
//! let edges = vec![(1u64, 0usize, 1usize), (2, 1, 2), (3, 0, 2)];
//! let (cost, used) = mst(3, &edges).unwrap();
//! assert_eq!(cost, 3);          // 辺(1)+辺(2)
//! assert_eq!(used.len(), 2);
//! ```
use crate::ds::dsu::Dsu;

/// `edges` = `(weight, u, v)`。最小全域木の `(総コスト, 採用辺インデックス列)` を返す。
/// 連結でなければ `None`。
pub fn mst(n: usize, edges: &[(u64, usize, usize)]) -> Option<(u64, Vec<usize>)> {
    let mut idx: Vec<usize> = (0..edges.len()).collect();
    idx.sort_by_key(|&i| edges[i].0);
    let mut dsu = Dsu::new(n);
    let mut cost = 0u64;
    let mut used = Vec::new();
    for &i in &idx {
        let (w, u, v) = edges[i];
        if !dsu.same(u, v) {
            dsu.merge(u, v);
            cost += w;
            used.push(i);
        }
    }
    if n == 0 {
        return Some((0, vec![]));
    }
    if used.len() == n - 1 {
        Some((cost, used))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic() {
        let edges = vec![
            (4u64, 0usize, 1usize),
            (1, 1, 2),
            (2, 0, 2),
            (3, 2, 3),
            (5, 1, 3),
        ];
        let (cost, used) = mst(4, &edges).unwrap();
        assert_eq!(cost, 1 + 2 + 3); // 辺1(1)+辺2(2)+辺3(3)
        assert_eq!(used.len(), 3);
    }
    #[test]
    fn disconnected() {
        let edges = vec![(1u64, 0usize, 1usize)];
        assert!(mst(3, &edges).is_none());
    }
    #[test]
    fn single() {
        assert_eq!(mst(1, &[]).unwrap().0, 0);
    }
}
