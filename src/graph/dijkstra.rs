//! ダイクストラ法（非負重み単一始点最短路）。
//!
//! ```
//! use cplib::graph::dijkstra::*;
//! // 0->1(2), 0->2(5), 1->2(1)
//! let mut g = vec![vec![]; 3];
//! g[0].push((1, 2u64)); g[0].push((2, 5)); g[1].push((2, 1));
//! let d = dijkstra(&g, 0);
//! assert_eq!(d[2], 3);   // 0->1->2
//! let top2 = dijkstra_top2(&g, 0);
//! assert_eq!(top2[2], [3, 5]);
//! ```
use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub const INF: u64 = u64::MAX;

/// `adj[v]` = `(to, weight)` の隣接リスト。到達不能は `INF`。
pub fn dijkstra(adj: &[Vec<(usize, u64)>], s: usize) -> Vec<u64> {
    let n = adj.len();
    let mut dist = vec![INF; n];
    dist[s] = 0;
    let mut pq: BinaryHeap<Reverse<(u64, usize)>> = BinaryHeap::new();
    pq.push(Reverse((0, s)));
    while let Some(Reverse((d, v))) = pq.pop() {
        if d > dist[v] {
            continue;
        }
        for &(to, w) in &adj[v] {
            let nd = d + w;
            if nd < dist[to] {
                dist[to] = nd;
                pq.push(Reverse((nd, to)));
            }
        }
    }
    dist
}

/// 各頂点への最短距離と 2 番目に短い距離を返す。
///
/// `res[v][0]` が最短、`res[v][1]` が 2 番目に短い距離。到達不能な順位は `INF`。
/// 同じ距離の複数経路は 1 つの順位として扱う。
pub fn dijkstra_top2(adj: &[Vec<(usize, u64)>], s: usize) -> Vec<[u64; 2]> {
    let n = adj.len();
    let mut dist = vec![[INF; 2]; n];
    let mut pq: BinaryHeap<Reverse<(u64, usize)>> = BinaryHeap::new();
    dist[s][0] = 0;
    pq.push(Reverse((0, s)));

    while let Some(Reverse((d, v))) = pq.pop() {
        if d > dist[v][1] {
            continue;
        }
        for &(to, w) in &adj[v] {
            let nd = d.saturating_add(w);
            if nd < dist[to][0] {
                dist[to][1] = dist[to][0];
                dist[to][0] = nd;
                pq.push(Reverse((nd, to)));
            } else if dist[to][0] < nd && nd < dist[to][1] {
                dist[to][1] = nd;
                pq.push(Reverse((nd, to)));
            }
        }
    }
    dist
}

/// 最短経路の 1 つを復元（到達不能なら `None`）。
pub fn dijkstra_path(adj: &[Vec<(usize, u64)>], s: usize, t: usize) -> Option<(u64, Vec<usize>)> {
    let n = adj.len();
    let mut dist = vec![INF; n];
    let mut prev = vec![usize::MAX; n];
    dist[s] = 0;
    let mut pq: BinaryHeap<Reverse<(u64, usize)>> = BinaryHeap::new();
    pq.push(Reverse((0, s)));
    while let Some(Reverse((d, v))) = pq.pop() {
        if d > dist[v] {
            continue;
        }
        for &(to, w) in &adj[v] {
            let nd = d + w;
            if nd < dist[to] {
                dist[to] = nd;
                prev[to] = v;
                pq.push(Reverse((nd, to)));
            }
        }
    }
    if dist[t] == INF {
        return None;
    }
    let mut path = vec![t];
    let mut cur = t;
    while cur != s {
        cur = prev[cur];
        path.push(cur);
    }
    path.reverse();
    Some((dist[t], path))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic() {
        let mut g = vec![vec![]; 5];
        let add = |a: usize, b: usize, w: u64, g: &mut Vec<Vec<(usize, u64)>>| {
            g[a].push((b, w));
            g[b].push((a, w));
        };
        add(0, 1, 4, &mut g);
        add(0, 2, 1, &mut g);
        add(2, 1, 2, &mut g);
        add(1, 3, 1, &mut g);
        add(2, 3, 5, &mut g);
        let d = dijkstra(&g, 0);
        assert_eq!(d[0], 0);
        assert_eq!(d[1], 3); // 0-2-1
        assert_eq!(d[2], 1);
        assert_eq!(d[3], 4); // 0-2-1-3
        assert_eq!(d[4], INF);
        let (cost, path) = dijkstra_path(&g, 0, 3).unwrap();
        assert_eq!(cost, 4);
        assert_eq!(path, vec![0, 2, 1, 3]);
        assert!(dijkstra_path(&g, 0, 4).is_none());
    }

    #[test]
    fn top2_basic() {
        let mut g = vec![vec![]; 5];
        g[0].push((1, 2));
        g[0].push((2, 5));
        g[1].push((2, 1));
        g[1].push((3, 4));
        g[2].push((3, 1));
        let d = dijkstra_top2(&g, 0);
        assert_eq!(d[0], [0, INF]);
        assert_eq!(d[1], [2, INF]);
        assert_eq!(d[2], [3, 5]);
        assert_eq!(d[3], [4, 6]);
        assert_eq!(d[4], [INF, INF]);
    }

    #[test]
    fn top2_matches_bruteforce_on_small_positive_graphs() {
        let mut seed = 246813579u64;
        let mut rng = || {
            seed ^= seed << 7;
            seed ^= seed >> 9;
            seed
        };
        for _ in 0..200 {
            let n = 2 + rng() as usize % 8;
            let mut g = vec![vec![]; n];
            for v in 0..n {
                for to in v + 1..n {
                    if rng() % 100 < 35 {
                        g[v].push((to, 1 + rng() % 9));
                    }
                }
            }
            let got = dijkstra_top2(&g, 0);
            let expected = brute_top2(&g, 0);
            assert_eq!(got, expected);
        }
    }

    fn brute_top2(adj: &[Vec<(usize, u64)>], s: usize) -> Vec<[u64; 2]> {
        let n = adj.len();
        let mut vals = vec![Vec::<u64>::new(); n];
        let mut stack = vec![(s, 0u64, vec![false; n])];
        while let Some((v, d, mut used)) = stack.pop() {
            vals[v].push(d);
            used[v] = true;
            for &(to, w) in &adj[v] {
                if !used[to] {
                    stack.push((to, d + w, used.clone()));
                }
            }
        }
        vals.into_iter()
            .map(|mut xs| {
                xs.sort_unstable();
                xs.dedup();
                [*xs.first().unwrap_or(&INF), *xs.get(1).unwrap_or(&INF)]
            })
            .collect()
    }
}
