//! 無向グラフの連結成分分解。
//!
//! ```
//! use cplib::graph::components::*;
//!
//! let adj = vec![vec![1], vec![0], vec![3], vec![2], vec![]];
//! let comp = connected_components(&adj);
//! assert_eq!(comp.len(), 3);
//! assert_eq!(component_ids(&adj), vec![0, 0, 1, 1, 2]);
//! assert_eq!(additional_edges_to_connect(&adj), 2);
//! ```

use std::collections::VecDeque;

/// 各頂点が属する連結成分 ID を返す。
///
/// ID は 0 から始まり、各成分で最初に見つかった頂点の昇順に付く。
pub fn component_ids(adj: &[Vec<usize>]) -> Vec<usize> {
    let n = adj.len();
    let mut ids = vec![usize::MAX; n];
    let mut comp_id = 0usize;
    let mut q = VecDeque::new();
    for s in 0..n {
        if ids[s] != usize::MAX {
            continue;
        }
        ids[s] = comp_id;
        q.push_back(s);
        while let Some(v) = q.pop_front() {
            for &to in &adj[v] {
                if ids[to] == usize::MAX {
                    ids[to] = comp_id;
                    q.push_back(to);
                }
            }
        }
        comp_id += 1;
    }
    ids
}

/// 連結成分ごとの頂点リストを返す。
pub fn connected_components(adj: &[Vec<usize>]) -> Vec<Vec<usize>> {
    let ids = component_ids(adj);
    let k = ids.iter().copied().max().map_or(0, |x| x + 1);
    let mut comps = vec![Vec::new(); k];
    for (v, id) in ids.into_iter().enumerate() {
        comps[id].push(v);
    }
    comps
}

pub fn connected_component_count(adj: &[Vec<usize>]) -> usize {
    component_ids(adj)
        .into_iter()
        .max()
        .map_or(0, |id| id + 1)
}

pub fn is_connected(adj: &[Vec<usize>]) -> bool {
    connected_component_count(adj) <= 1
}

/// グラフ全体を連結にするために必要な辺数。
pub fn additional_edges_to_connect(adj: &[Vec<usize>]) -> usize {
    connected_component_count(adj).saturating_sub(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn add_undirected(adj: &mut [Vec<usize>], a: usize, b: usize) {
        adj[a].push(b);
        adj[b].push(a);
    }

    #[test]
    fn basic() {
        let mut adj = vec![vec![]; 6];
        add_undirected(&mut adj, 0, 1);
        add_undirected(&mut adj, 1, 2);
        add_undirected(&mut adj, 3, 4);
        assert_eq!(component_ids(&adj), vec![0, 0, 0, 1, 1, 2]);
        assert_eq!(connected_components(&adj), vec![vec![0, 1, 2], vec![3, 4], vec![5]]);
        assert_eq!(connected_component_count(&adj), 3);
        assert!(!is_connected(&adj));
        assert_eq!(additional_edges_to_connect(&adj), 2);
    }

    #[test]
    fn empty_and_singleton() {
        let empty: Vec<Vec<usize>> = vec![];
        assert!(connected_components(&empty).is_empty());
        assert_eq!(connected_component_count(&empty), 0);
        assert!(is_connected(&empty));
        assert_eq!(additional_edges_to_connect(&empty), 0);

        let one = vec![vec![]];
        assert_eq!(component_ids(&one), vec![0]);
        assert_eq!(connected_components(&one), vec![vec![0]]);
        assert!(is_connected(&one));
    }

    #[test]
    fn random_matches_dsu() {
        let mut seed = 1357911u64;
        let mut rng = || {
            seed ^= seed << 7;
            seed ^= seed >> 9;
            seed
        };
        for _ in 0..300 {
            let n = rng() as usize % 30;
            let mut adj = vec![vec![]; n];
            let mut dsu = NaiveDsu::new(n);
            for i in 0..n {
                for j in i + 1..n {
                    if rng() % 100 < 12 {
                        add_undirected(&mut adj, i, j);
                        dsu.merge(i, j);
                    }
                }
            }
            let ids = component_ids(&adj);
            for i in 0..n {
                for j in 0..n {
                    assert_eq!(ids[i] == ids[j], dsu.same(i, j));
                }
            }
        }
    }

    struct NaiveDsu {
        parent: Vec<usize>,
    }

    impl NaiveDsu {
        fn new(n: usize) -> Self {
            Self {
                parent: (0..n).collect(),
            }
        }

        fn root(&mut self, x: usize) -> usize {
            if self.parent[x] == x {
                x
            } else {
                let r = self.root(self.parent[x]);
                self.parent[x] = r;
                r
            }
        }

        fn merge(&mut self, a: usize, b: usize) {
            let ra = self.root(a);
            let rb = self.root(b);
            if ra != rb {
                self.parent[rb] = ra;
            }
        }

        fn same(&mut self, a: usize, b: usize) -> bool {
            self.root(a) == self.root(b)
        }
    }
}
