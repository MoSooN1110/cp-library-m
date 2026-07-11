//! Lowlink（橋・関節点の列挙）。反復 DFS でスタック安全。無向単純グラフ想定。
//!
//! ```
//! use cplib::graph::lowlink::*;
//! // 0-1-2-0 の三角形 + 2-3 の橋
//! let adj = vec![vec![1, 2], vec![0, 2], vec![0, 1, 3], vec![2]];
//! let ll = LowLink::new(&adj);
//! assert_eq!(ll.bridges, vec![(2, 3)]);
//! assert_eq!(ll.articulations, vec![2]);
//! ```

pub struct LowLink {
    /// 橋（u < v に正規化）
    pub bridges: Vec<(usize, usize)>,
    /// 関節点（昇順）
    pub articulations: Vec<usize>,
    pub ord: Vec<i32>,
    pub low: Vec<i32>,
}

impl LowLink {
    pub fn new(adj: &[Vec<usize>]) -> Self {
        let n = adj.len();
        let mut ord = vec![-1i32; n];
        let mut low = vec![0i32; n];
        let mut bridges = vec![];
        let mut is_art = vec![false; n];
        let mut timer = 0i32;

        for s in 0..n {
            if ord[s] != -1 {
                continue;
            }
            // (v, parent, iterator index, root_child_count はルートのみ意味を持つ)
            let mut stack: Vec<(usize, i32, usize)> = vec![(s, -1, 0)];
            let mut root_children = 0;
            ord[s] = timer;
            low[s] = timer;
            timer += 1;
            while let Some(&mut (v, par, ref mut i)) = stack.last_mut() {
                if *i < adj[v].len() {
                    let to = adj[v][*i];
                    *i += 1;
                    if to as i32 == par {
                        continue;
                    }
                    if ord[to] != -1 {
                        // back edge
                        if ord[to] < low[v] {
                            low[v] = ord[to];
                        }
                    } else {
                        if par == -1 {
                            root_children += 1;
                        }
                        ord[to] = timer;
                        low[to] = timer;
                        timer += 1;
                        stack.push((to, v as i32, 0));
                    }
                } else {
                    // v の探索終了 → 親へ戻る処理
                    stack.pop();
                    let _ = par;
                    if let Some(&(p, p_par, _)) = stack.last() {
                        if low[v] < low[p] {
                            low[p] = low[v];
                        }
                        // 橋: low[v] > ord[p]
                        if low[v] > ord[p] {
                            let (a, b) = if p < v { (p, v) } else { (v, p) };
                            bridges.push((a, b));
                        }
                        // 関節点（非ルート p）: 子 v が low[v] >= ord[p]
                        if p_par != -1 && low[v] >= ord[p] {
                            is_art[p] = true;
                        }
                    }
                }
            }
            if root_children >= 2 {
                is_art[s] = true;
            }
        }

        bridges.sort_unstable();
        let articulations: Vec<usize> = (0..n).filter(|&v| is_art[v]).collect();
        LowLink {
            bridges,
            articulations,
            ord,
            low,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// active[v]=true の頂点だけを使った連結成分数
    fn comps(n: usize, adj: &[Vec<usize>], active: &[bool]) -> usize {
        let mut seen = vec![false; n];
        let mut c = 0;
        for s in 0..n {
            if !active[s] || seen[s] {
                continue;
            }
            c += 1;
            let mut st = vec![s];
            seen[s] = true;
            while let Some(v) = st.pop() {
                for &to in &adj[v] {
                    if active[to] && !seen[to] {
                        seen[to] = true;
                        st.push(to);
                    }
                }
            }
        }
        c
    }

    #[test]
    fn random_vs_brute() {
        let mut x: u64 = 20260711;
        let mut rng = || {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            x
        };
        for _ in 0..200 {
            let n = 2 + (rng() as usize) % 7;
            let mut edges: Vec<(usize, usize)> = vec![];
            let mut adj = vec![vec![]; n];
            let mut set = std::collections::HashSet::new();
            let m = (rng() as usize) % 12;
            for _ in 0..m {
                let u = (rng() as usize) % n;
                let v = (rng() as usize) % n;
                if u == v {
                    continue;
                }
                let key = (u.min(v), u.max(v));
                if set.insert(key) {
                    edges.push(key);
                    adj[u].push(v);
                    adj[v].push(u);
                }
            }
            let ll = LowLink::new(&adj);
            let all = vec![true; n];
            let c0 = comps(n, &adj, &all);

            // 橋: 取り除くと成分が増える辺
            let mut brute_bridges = vec![];
            for &(u, v) in &edges {
                let mut adj2 = adj.clone();
                adj2[u].retain(|&w| w != v);
                adj2[v].retain(|&w| w != u);
                if comps(n, &adj2, &all) > c0 {
                    brute_bridges.push((u, v));
                }
            }
            brute_bridges.sort_unstable();
            assert_eq!(ll.bridges, brute_bridges);

            // 関節点: a を消すと成分が増える（c1 > c0）
            let mut brute_art = vec![];
            for a in 0..n {
                let mut active = vec![true; n];
                active[a] = false;
                if comps(n, &adj, &active) > c0 {
                    brute_art.push(a);
                }
            }
            assert_eq!(ll.articulations, brute_art);
        }
    }
}
