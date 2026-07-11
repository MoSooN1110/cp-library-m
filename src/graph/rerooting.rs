//! 全方位木 DP（Rerooting）。各頂点を根としたときの値を一括計算。
//!
//! - `e`: merge の単位元
//! - `merge(a, b)`: 子メッセージの結合（可換モノイド）
//! - `put_edge(val, from, to)`: 部分木の値 `val` を辺 (from→to) 越しのメッセージへ変換
//! - `add_vertex(acc, v)`: 集約済みメッセージ `acc` に頂点 v を付加し、v 側の値にする
//!
//! ```
//! use cplib::graph::rerooting::*;
//! // 各頂点からの「距離の総和」を全頂点分求める（(件数, 距離和)）
//! let adj = vec![vec![1, 2], vec![0, 3], vec![0], vec![1]];
//! let ans = rerooting(
//!     &adj,
//!     (0i64, 0i64),
//!     |a: &(i64, i64), b: &(i64, i64)| (a.0 + b.0, a.1 + b.1),
//!     |v: &(i64, i64), _from: usize, _to: usize| (v.0, v.1 + v.0), // 辺を渡ると距離+1×件数
//!     |acc: &(i64, i64), _v: usize| (acc.0 + 1, acc.1),            // 頂点を1つ足す
//! );
//! // 頂点0からの距離和 = 1(→1)+1(→2)+2(→3) = 4
//! assert_eq!(ans[0].1, 4);
//! ```
use std::collections::VecDeque;

pub fn rerooting<S, FM, FE, FV>(
    adj: &[Vec<usize>],
    e: S,
    merge: FM,
    put_edge: FE,
    add_vertex: FV,
) -> Vec<S>
where
    S: Clone,
    FM: Fn(&S, &S) -> S,
    FE: Fn(&S, usize, usize) -> S,
    FV: Fn(&S, usize) -> S,
{
    let n = adj.len();
    if n == 0 {
        return vec![];
    }
    // BFS で par と処理順（親→子）
    let mut par = vec![-1i32; n];
    let mut order = Vec::with_capacity(n);
    let mut visited = vec![false; n];
    let mut q = VecDeque::new();
    visited[0] = true;
    q.push_back(0);
    while let Some(v) = q.pop_front() {
        order.push(v);
        for &to in &adj[v] {
            if !visited[to] {
                visited[to] = true;
                par[to] = v as i32;
                q.push_back(to);
            }
        }
    }

    // 部分木の値 g[v]（0 を根）
    let mut g = vec![e.clone(); n];
    for &v in order.iter().rev() {
        let mut acc = e.clone();
        for &c in &adj[v] {
            if c as i32 != par[v] {
                acc = merge(&acc, &put_edge(&g[c], c, v));
            }
        }
        g[v] = add_vertex(&acc, v);
    }

    // 全方位
    let mut from_par = vec![e.clone(); n]; // 親 → v のメッセージ
    let mut ans = vec![e.clone(); n];
    for &v in &order {
        let nb = &adj[v];
        let m = nb.len();
        let msgs: Vec<S> = nb
            .iter()
            .map(|&w| {
                if w as i32 == par[v] {
                    from_par[v].clone()
                } else {
                    put_edge(&g[w], w, v)
                }
            })
            .collect();
        let mut pre = vec![e.clone(); m + 1];
        for i in 0..m {
            pre[i + 1] = merge(&pre[i], &msgs[i]);
        }
        let mut suf = vec![e.clone(); m + 1];
        for i in (0..m).rev() {
            suf[i] = merge(&msgs[i], &suf[i + 1]);
        }
        ans[v] = add_vertex(&pre[m], v);
        for i in 0..m {
            let w = nb[i];
            if w as i32 != par[v] {
                let without = merge(&pre[i], &suf[i + 1]);
                let vval = add_vertex(&without, v);
                from_par[w] = put_edge(&vval, v, w);
            }
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sum_of_distances(adj: &[Vec<usize>]) -> Vec<i64> {
        let ans = rerooting(
            adj,
            (0i64, 0i64),
            |a: &(i64, i64), b: &(i64, i64)| (a.0 + b.0, a.1 + b.1),
            |v: &(i64, i64), _f: usize, _t: usize| (v.0, v.1 + v.0),
            |acc: &(i64, i64), _v: usize| (acc.0 + 1, acc.1),
        );
        ans.iter().map(|&(_c, d)| d).collect()
    }

    fn brute(adj: &[Vec<usize>], s: usize) -> i64 {
        // BFS 距離和
        let n = adj.len();
        let mut dist = vec![-1i64; n];
        let mut q = VecDeque::new();
        dist[s] = 0;
        q.push_back(s);
        let mut sum = 0;
        while let Some(v) = q.pop_front() {
            sum += dist[v];
            for &to in &adj[v] {
                if dist[to] < 0 {
                    dist[to] = dist[v] + 1;
                    q.push_back(to);
                }
            }
        }
        sum
    }

    #[test]
    fn distances_match_brute() {
        let mut x: u64 = 271828;
        let mut rng = || {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            x
        };
        for _ in 0..150 {
            let n = 1 + (rng() as usize) % 30;
            let mut adj = vec![vec![]; n];
            for v in 1..n {
                let p = (rng() as usize) % v;
                adj[v].push(p);
                adj[p].push(v);
            }
            let got = sum_of_distances(&adj);
            for v in 0..n {
                assert_eq!(got[v], brute(&adj, v), "n={n} v={v}");
            }
        }
    }
}
