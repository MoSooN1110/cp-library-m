//! 二部グラフ判定（2 彩色）。可能なら各頂点の色 (0/1) を返す。
//!
//! ```
//! use cplib::graph::two_coloring::*;
//! let adj = vec![vec![1, 2], vec![0, 3], vec![0, 3], vec![1, 2]]; // 4-cycle
//! assert!(two_coloring(&adj).is_some());
//! let odd = vec![vec![1, 2], vec![0, 2], vec![0, 1]];              // 三角形
//! assert!(two_coloring(&odd).is_none());
//! ```
use std::collections::VecDeque;

/// 二部グラフなら色配列 `Some(colors)`、奇閉路を含むなら `None`。
pub fn two_coloring(adj: &[Vec<usize>]) -> Option<Vec<u8>> {
    let n = adj.len();
    let mut color = vec![u8::MAX; n];
    for s in 0..n {
        if color[s] != u8::MAX {
            continue;
        }
        color[s] = 0;
        let mut q = VecDeque::new();
        q.push_back(s);
        while let Some(v) = q.pop_front() {
            for &to in &adj[v] {
                if color[to] == u8::MAX {
                    color[to] = color[v] ^ 1;
                    q.push_back(to);
                } else if color[to] == color[v] {
                    return None;
                }
            }
        }
    }
    Some(color)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic() {
        // パスは二部
        let path = vec![vec![1], vec![0, 2], vec![1, 3], vec![2]];
        let c = two_coloring(&path).unwrap();
        for v in 0..4 {
            for &to in &path[v] {
                assert_ne!(c[v], c[to]);
            }
        }
        // 奇閉路は不可
        let tri = vec![vec![1, 2], vec![0, 2], vec![0, 1]];
        assert!(two_coloring(&tri).is_none());
        // 非連結でもOK
        let dis = vec![vec![1], vec![0], vec![3], vec![2]];
        assert!(two_coloring(&dis).is_some());
    }
}
