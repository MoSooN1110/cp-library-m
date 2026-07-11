//! 後退解析（retrograde analysis）。ゲームグラフの各局面を勝ち/負け/引き分けに分類する。
//!
//! 辺 (u, v) は「局面 u の手番プレイヤーが v へ着手できる」ことを表す。
//! 出次数 0 の局面は手番側の負け。負け局面から逆辺 BFS で伝播し、
//! どちらにも確定しない局面は引き分け（無限ループ）。O(n + m)。
//!
//! ```
//! use cplib::graph::retrograde_analysis::*;
//! // 0 -> 1 -> 2（2 は手詰まり = 負け）
//! let res = retrograde_analysis(3, &[(0, 1), (1, 2)]);
//! assert_eq!(res, vec![GameResult::Lose, GameResult::Win, GameResult::Lose]);
//! // 2 頂点サイクルは引き分け
//! let res = retrograde_analysis(2, &[(0, 1), (1, 0)]);
//! assert_eq!(res, vec![GameResult::Draw, GameResult::Draw]);
//! ```

use std::collections::VecDeque;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum GameResult {
    /// 手番側が勝てる局面
    Win,
    /// 手番側が負ける局面
    Lose,
    /// 双方最善で決着しない局面
    Draw,
}

/// 各局面 0..n の勝敗を返す。edges の (u, v) は u から v への着手。
pub fn retrograde_analysis(n: usize, edges: &[(usize, usize)]) -> Vec<GameResult> {
    let mut remaining = vec![0usize; n]; // 未確定の後続手数
    let mut radj = vec![vec![]; n];
    for &(u, v) in edges {
        assert!(u < n && v < n);
        remaining[u] += 1;
        radj[v].push(u);
    }
    let mut res: Vec<Option<GameResult>> = vec![None; n];
    let mut q = VecDeque::new();
    for v in 0..n {
        if remaining[v] == 0 {
            res[v] = Some(GameResult::Lose);
            q.push_back(v);
        }
    }
    while let Some(v) = q.pop_front() {
        let v_result = res[v].unwrap();
        for &u in &radj[v] {
            if res[u].is_some() {
                continue;
            }
            if v_result == GameResult::Lose {
                // 負け局面へ着手できるなら勝ち
                res[u] = Some(GameResult::Win);
                q.push_back(u);
            } else {
                // 勝ち局面への着手を消費。すべて勝ちなら負け
                remaining[u] -= 1;
                if remaining[u] == 0 {
                    res[u] = Some(GameResult::Lose);
                    q.push_back(u);
                }
            }
        }
    }
    res.into_iter()
        .map(|r| r.unwrap_or(GameResult::Draw))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use GameResult::*;

    // ナイーブ: 確定するまでラベル付けを繰り返す固定点反復
    fn naive(n: usize, edges: &[(usize, usize)]) -> Vec<GameResult> {
        let mut adj = vec![vec![]; n];
        for &(u, v) in edges {
            adj[u].push(v);
        }
        let mut res: Vec<Option<GameResult>> = vec![None; n];
        loop {
            let mut changed = false;
            for v in 0..n {
                if res[v].is_some() {
                    continue;
                }
                let new = if adj[v].iter().any(|&to| res[to] == Some(Lose)) {
                    Some(Win)
                } else if adj[v].iter().all(|&to| res[to] == Some(Win)) {
                    Some(Lose) // 出次数 0 もここ（all は空で真）
                } else {
                    None
                };
                if new.is_some() {
                    res[v] = new;
                    changed = true;
                }
            }
            if !changed {
                break;
            }
        }
        res.into_iter().map(|r| r.unwrap_or(Draw)).collect()
    }

    #[test]
    fn hand_verified() {
        // パス 0->1->2->3->4: 終端 4 から交互
        let edges: Vec<(usize, usize)> = (0..4).map(|i| (i, i + 1)).collect();
        assert_eq!(
            retrograde_analysis(5, &edges),
            vec![Lose, Win, Lose, Win, Lose]
        );
        // 3 サイクル: すべて引き分け
        assert_eq!(
            retrograde_analysis(3, &[(0, 1), (1, 2), (2, 0)]),
            vec![Draw, Draw, Draw]
        );
        // サイクル + 逃げ道: 1 は負け局面 2 へ行けるので勝ち、0 は勝ち局面しかないので負け
        assert_eq!(
            retrograde_analysis(3, &[(0, 1), (1, 0), (1, 2)]),
            vec![Lose, Win, Lose]
        );
        // 引き分けへ逃げられる負けかけ局面は引き分け
        // 0 -> 1(勝ちのみ) だが 0 -> 2 <-> 3 サイクルあり
        assert_eq!(
            retrograde_analysis(5, &[(0, 1), (1, 4), (0, 2), (2, 3), (3, 2)]),
            vec![Draw, Win, Draw, Draw, Lose]
        );
        // 孤立頂点（出次数 0）は負け
        assert_eq!(retrograde_analysis(1, &[]), vec![Lose]);
    }

    #[test]
    fn random_vs_naive() {
        let mut x: u64 = 141421356237;
        let mut rng = move || {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            x
        };
        for _ in 0..200 {
            let n = 1 + (rng() as usize) % 12;
            let m = (rng() as usize) % (2 * n + 1);
            let mut edges = vec![];
            for _ in 0..m {
                let u = (rng() as usize) % n;
                let v = (rng() as usize) % n;
                if u != v {
                    edges.push((u, v));
                }
            }
            assert_eq!(
                retrograde_analysis(n, &edges),
                naive(n, &edges),
                "n={n} edges={edges:?}"
            );
        }
    }
}
