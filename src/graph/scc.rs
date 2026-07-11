//! 強連結成分分解（Kosaraju, 反復版でスタック安全）。
//! 返り値はトポロジカル順（縮約グラフで前の成分から後ろの成分へ辺が向く）。
//!
//! ```
//! use cplib::graph::scc::*;
//! let mut g = SccGraph::new(5);
//! g.add_edge(0, 1); g.add_edge(1, 0);   // {0,1}
//! g.add_edge(1, 2); g.add_edge(2, 3);
//! g.add_edge(3, 2);                       // {2,3}
//! let comp = g.scc();
//! assert_eq!(comp.len(), 3);              // {0,1},{2,3},{4}
//! ```

pub struct SccGraph {
    n: usize,
    edges: Vec<(usize, usize)>,
}

impl SccGraph {
    pub fn new(n: usize) -> Self {
        SccGraph { n, edges: vec![] }
    }
    pub fn add_edge(&mut self, from: usize, to: usize) {
        assert!(from < self.n && to < self.n);
        self.edges.push((from, to));
    }

    /// 強連結成分をトポロジカル順に返す。
    pub fn scc(&self) -> Vec<Vec<usize>> {
        let n = self.n;
        // CSR 隣接（正方向・逆方向）
        let mut adj = vec![vec![]; n];
        let mut radj = vec![vec![]; n];
        for &(u, v) in &self.edges {
            adj[u].push(v);
            radj[v].push(u);
        }

        // 1) 帰りがけ順（反復 DFS）
        let mut visited = vec![false; n];
        let mut order = Vec::with_capacity(n);
        let mut stack: Vec<(usize, usize)> = Vec::new();
        for s in 0..n {
            if visited[s] {
                continue;
            }
            visited[s] = true;
            stack.push((s, 0));
            while let Some(&mut (v, ref mut i)) = stack.last_mut() {
                if *i < adj[v].len() {
                    let to = adj[v][*i];
                    *i += 1;
                    if !visited[to] {
                        visited[to] = true;
                        stack.push((to, 0));
                    }
                } else {
                    order.push(v);
                    stack.pop();
                }
            }
        }

        // 2) 逆順に逆グラフ DFS で成分収集
        let mut comp = vec![usize::MAX; n];
        let mut groups: Vec<Vec<usize>> = Vec::new();
        for &s in order.iter().rev() {
            if comp[s] != usize::MAX {
                continue;
            }
            let gid = groups.len();
            let mut members = Vec::new();
            let mut st = vec![s];
            comp[s] = gid;
            while let Some(v) = st.pop() {
                members.push(v);
                for &to in &radj[v] {
                    if comp[to] == usize::MAX {
                        comp[to] = gid;
                        st.push(to);
                    }
                }
            }
            groups.push(members);
        }
        groups
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ナイヤル: 到達可能性から SCC を求める
    fn brute(n: usize, edges: &[(usize, usize)]) -> Vec<Vec<usize>> {
        let mut reach = vec![vec![false; n]; n];
        for i in 0..n {
            reach[i][i] = true;
        }
        for &(u, v) in edges {
            reach[u][v] = true;
        }
        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    if reach[i][k] && reach[k][j] {
                        reach[i][j] = true;
                    }
                }
            }
        }
        let mut seen = vec![false; n];
        let mut res = vec![];
        for i in 0..n {
            if seen[i] {
                continue;
            }
            let mut g = vec![];
            for j in 0..n {
                if reach[i][j] && reach[j][i] {
                    g.push(j);
                    seen[j] = true;
                }
            }
            res.push(g);
        }
        res
    }

    fn as_sets(mut groups: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
        for g in groups.iter_mut() {
            g.sort();
        }
        groups.sort();
        groups
    }

    #[test]
    fn random_and_topo() {
        let mut x: u64 = 998244353;
        let mut rng = || {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            x
        };
        for _ in 0..200 {
            let n = 1 + (rng() as usize) % 7;
            let m = (rng() as usize) % 12;
            let mut edges = vec![];
            let mut g = SccGraph::new(n);
            for _ in 0..m {
                let u = (rng() as usize) % n;
                let v = (rng() as usize) % n;
                edges.push((u, v));
                g.add_edge(u, v);
            }
            let got = g.scc();
            // 分割が一致
            assert_eq!(as_sets(got.clone()), as_sets(brute(n, &edges)));
            // トポロジカル順の検証: comp id が「前→後ろ」
            let mut comp = vec![0usize; n];
            for (gid, grp) in got.iter().enumerate() {
                for &v in grp {
                    comp[v] = gid;
                }
            }
            for &(u, v) in &edges {
                assert!(comp[u] <= comp[v], "edge {u}->{v} breaks topo order");
            }
        }
    }
}
