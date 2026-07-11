//! ワーシャルフロイド法（全点対最短路、負辺可・負閉路検出）。
//!
//! ```
//! use cplib::graph::warshall_floyd::*;
//! let mut d = vec![
//!     vec![0, 3, INF],
//!     vec![INF, 0, 1],
//!     vec![1, INF, 0],
//! ];
//! assert!(!warshall_floyd(&mut d));
//! assert_eq!(d[0][2], 4); // 0->1->2
//! ```

pub const INF: i64 = 1 << 60;

/// `dist` を全点対最短距離へ更新する（in-place）。到達不能は `INF`。
/// 負閉路があれば `true` を返す。
pub fn warshall_floyd(dist: &mut [Vec<i64>]) -> bool {
    let n = dist.len();
    for k in 0..n {
        for i in 0..n {
            if dist[i][k] == INF {
                continue;
            }
            for j in 0..n {
                if dist[k][j] == INF {
                    continue;
                }
                let nd = dist[i][k] + dist[k][j];
                if nd < dist[i][j] {
                    dist[i][j] = nd;
                }
            }
        }
    }
    (0..n).any(|i| dist[i][i] < 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn all_pairs() {
        let mut d = vec![
            vec![0, 4, INF, INF],
            vec![INF, 0, 1, INF],
            vec![1, INF, 0, 5],
            vec![INF, INF, INF, 0],
        ];
        assert!(!warshall_floyd(&mut d));
        assert_eq!(d[0][2], 5); // 0->1->2
        assert_eq!(d[2][1], 5); // 2->0->1
        assert_eq!(d[0][3], 10);
    }
    #[test]
    fn detect_negative_cycle() {
        let mut d = vec![vec![0, 1], vec![-3, 0]];
        assert!(warshall_floyd(&mut d));
    }
}
