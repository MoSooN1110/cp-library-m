//! Auxiliary Tree（virtual tree / 圧縮木）。指定頂点集合とその pairwise LCA だけからなる木。
//!
//! 木を 1 回前処理（O(n log n)）した後、頂点集合 vs ごとに O(|vs| log |vs|) で
//! 祖先関係を保った圧縮木を構築する。LCA とオイラーツアーは既存モジュールを利用。
//!
//! ```
//! use cplib::graph::auxiliary_tree::*;
//! // 0-1, 0-2, 1-3, 1-4
//! let adj = vec![vec![1, 2], vec![0, 3, 4], vec![0], vec![1], vec![1]];
//! let aux = AuxiliaryTree::new(&adj, 0);
//! let (vs, edges) = aux.query(&[3, 4]);
//! assert_eq!(vs, vec![1, 3, 4]); // LCA(3,4)=1 が自動で追加される
//! assert_eq!(edges, vec![(1, 3), (1, 4)]);
//! ```

use crate::graph::euler_tour::EulerTour;
use crate::graph::lca::Lca;

pub struct AuxiliaryTree {
    lca: Lca,
    et: EulerTour,
}

impl AuxiliaryTree {
    /// 隣接リストと根から前処理する。O(n log n)。
    pub fn new(adj: &[Vec<usize>], root: usize) -> Self {
        Self {
            lca: Lca::new(adj, root),
            et: EulerTour::new(adj, root),
        }
    }

    /// u と v の LCA（元の木上）。
    pub fn lca(&self, u: usize, v: usize) -> usize {
        self.lca.lca(u, v)
    }

    /// 頂点集合 vs の auxiliary tree を構築する。
    ///
    /// 戻り値は `(nodes, edges)`:
    /// - `nodes`: vs とその pairwise LCA を行きがけ順（tin 昇順）に並べたもの。
    ///   先頭 `nodes[0]` が auxiliary tree の根。
    /// - `edges`: `(parent, child)` の組。parent は child の元の木での最近祖先
    ///   （nodes 内で child の真の祖先のうち最も深いもの）。
    pub fn query(&self, vs: &[usize]) -> (Vec<usize>, Vec<(usize, usize)>) {
        if vs.is_empty() {
            return (vec![], vec![]);
        }
        let mut nodes: Vec<usize> = vs.to_vec();
        nodes.sort_by_key(|&v| self.et.tin[v]);
        nodes.dedup();
        // 隣接頂点対の LCA を追加
        for i in 0..nodes.len().saturating_sub(1) {
            let w = self.lca.lca(nodes[i], nodes[i + 1]);
            nodes.push(w);
        }
        nodes.sort_by_key(|&v| self.et.tin[v]);
        nodes.dedup();
        // スタックで親子関係を復元
        let mut edges = Vec::with_capacity(nodes.len().saturating_sub(1));
        let mut stack: Vec<usize> = vec![];
        for &v in &nodes {
            while let Some(&top) = stack.last() {
                if self.et.is_ancestor(top, v) {
                    break;
                }
                stack.pop();
            }
            if let Some(&top) = stack.last() {
                edges.push((top, v));
            }
            stack.push(v);
        }
        (nodes, edges)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn random_tree(n: usize, rng: &mut impl FnMut() -> u64) -> Vec<Vec<usize>> {
        let mut adj = vec![vec![]; n];
        for v in 1..n {
            let p = (rng() as usize) % v;
            adj[p].push(v);
            adj[v].push(p);
        }
        adj
    }

    #[test]
    fn known_small() {
        // 0-1, 0-2, 1-3, 1-4, 2-5
        let adj = vec![
            vec![1, 2],
            vec![0, 3, 4],
            vec![0, 5],
            vec![1],
            vec![1],
            vec![2],
        ];
        let aux = AuxiliaryTree::new(&adj, 0);
        let (vs, edges) = aux.query(&[3, 4, 5]);
        // LCA(3,4)=1, LCA(4,5)=0 → {0,1,3,4,5}
        let mut sorted = vs.clone();
        sorted.sort();
        assert_eq!(sorted, vec![0, 1, 3, 4, 5]);
        assert_eq!(vs[0], 0); // 根
        assert_eq!(edges.len(), vs.len() - 1);
        // 単一頂点
        let (vs1, edges1) = aux.query(&[4]);
        assert_eq!(vs1, vec![4]);
        assert!(edges1.is_empty());
        // 空
        assert_eq!(aux.query(&[]), (vec![], vec![]));
    }

    #[test]
    fn random_matches_lca_closure() {
        let mut x: u64 = 314159265;
        let mut rng = move || {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            x
        };
        for _ in 0..40 {
            let n = 2 + (rng() as usize) % 39;
            let adj = random_tree(n, &mut rng);
            let aux = AuxiliaryTree::new(&adj, 0);
            let k = 1 + (rng() as usize) % n;
            let mut vs: Vec<usize> = (0..k).map(|_| (rng() as usize) % n).collect();
            vs.sort();
            vs.dedup();

            let (nodes, edges) = aux.query(&vs);

            // (a) 頂点集合 = vs の pairwise LCA 閉包
            let mut expected: Vec<usize> = vec![];
            for &u in &vs {
                for &v in &vs {
                    expected.push(aux.lca(u, v));
                }
            }
            expected.sort();
            expected.dedup();
            let mut got = nodes.clone();
            got.sort();
            assert_eq!(got, expected, "vertex closure mismatch");

            // nodes は tin 昇順
            for w in nodes.windows(2) {
                assert!(aux.et.tin[w[0]] < aux.et.tin[w[1]]);
            }

            // (b) 辺: parent は child の nodes 内での最近真祖先
            assert_eq!(edges.len(), nodes.len() - 1);
            let mut child_seen = vec![false; n];
            for &(p, c) in &edges {
                assert_ne!(p, c);
                assert!(aux.et.is_ancestor(p, c), "parent must be ancestor");
                assert!(!child_seen[c], "child appears once");
                child_seen[c] = true;
                for &w in &nodes {
                    if w != p && w != c {
                        assert!(
                            !(aux.et.is_ancestor(p, w) && aux.et.is_ancestor(w, c)),
                            "no node strictly between parent and child"
                        );
                    }
                }
            }
            // 根 (nodes[0]) 以外はすべて child として現れる
            assert!(!child_seen[nodes[0]]);
            for &v in &nodes[1..] {
                assert!(child_seen[v]);
            }
        }
    }
}
