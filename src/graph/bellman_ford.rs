//! ベルマンフォード法（負辺可、負閉路検出つき単一始点最短路）。
//!
//! ```
//! use cplib::graph::bellman_ford::*;
//! // 0->1(2), 1->2(-5), 0->2(1)
//! let edges = vec![(0usize, 1usize, 2i64), (1, 2, -5), (0, 2, 1)];
//! let (dist, neg) = bellman_ford(3, &edges, 0);
//! assert!(!neg);
//! assert_eq!(dist[2], -3);
//! ```

pub const INF: i64 = 1 << 60;

/// `edges` = `(from, to, cost)`。返り値 `(dist, neg)`:
/// `dist[v]` は s からの最短距離（到達不能は `INF`）、
/// `neg` は「s から到達可能な頂点に影響する負閉路があるか」。
/// 負閉路の影響を受ける頂点は `dist` が `-INF` になる。
pub fn bellman_ford(n: usize, edges: &[(usize, usize, i64)], s: usize) -> (Vec<i64>, bool) {
    let mut dist = vec![INF; n];
    dist[s] = 0;
    for _ in 0..n - 1 {
        for &(u, v, w) in edges {
            if dist[u] != INF && dist[u] + w < dist[v] {
                dist[v] = dist[u] + w;
            }
        }
    }
    // さらに n 回で -INF を伝播させ、負閉路の影響範囲を検出
    let mut neg = false;
    for _ in 0..n {
        for &(u, v, w) in edges {
            if dist[u] != INF && dist[u] + w < dist[v] {
                if dist[v] != -INF {
                    neg = true;
                }
                dist[v] = -INF;
            }
        }
    }
    (dist, neg)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn shortest() {
        let edges = vec![(0usize, 1usize, 4i64), (0, 2, 1), (2, 1, 2), (1, 3, 1), (2, 3, 5)];
        let (dist, neg) = bellman_ford(4, &edges, 0);
        assert!(!neg);
        assert_eq!(dist, vec![0, 3, 1, 4]);
    }
    #[test]
    fn unreachable() {
        let edges = vec![(0usize, 1usize, 1i64)];
        let (dist, _) = bellman_ford(3, &edges, 0);
        assert_eq!(dist[2], INF);
    }
    #[test]
    fn negative_cycle() {
        // 1->2->3->1 で -1 の負閉路、0 から到達可能
        let edges = vec![(0usize, 1usize, 1i64), (1, 2, 1), (2, 3, 1), (3, 1, -3)];
        let (dist, neg) = bellman_ford(4, &edges, 0);
        assert!(neg);
        assert_eq!(dist[1], -INF);
    }
    #[test]
    fn negative_cycle_unreachable() {
        // 負閉路はあるが s=0 から到達不能
        let edges = vec![(1usize, 2usize, 1i64), (2, 1, -3)];
        let (_, neg) = bellman_ford(3, &edges, 0);
        assert!(!neg);
    }
}
