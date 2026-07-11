//! Functional graph（各頂点の出次数 1）のサイクル検出。
//!
//! ```
//! use cplib::graph::functional_graph::*;
//! // 0->1->2->0（サイクル）, 3->1（尻尾）
//! let next = vec![1, 2, 0, 1];
//! let fg = FunctionalGraph::new(&next);
//! assert_eq!(fg.on_cycle, vec![true, true, true, false]);
//! assert_eq!(fg.cycles.len(), 1);
//! ```

pub struct FunctionalGraph {
    /// on_cycle[v]: v がいずれかのサイクル上にあるか
    pub on_cycle: Vec<bool>,
    /// 検出したサイクル（各サイクルの頂点列）
    pub cycles: Vec<Vec<usize>>,
}

impl FunctionalGraph {
    pub fn new(next: &[usize]) -> Self {
        let n = next.len();
        let mut state = vec![0u8; n]; // 0 white, 1 gray(現在の経路), 2 black(確定)
        let mut pos = vec![usize::MAX; n]; // 現在の経路中での位置
        let mut on_cycle = vec![false; n];
        let mut cycles = vec![];

        for s in 0..n {
            if state[s] != 0 {
                continue;
            }
            let mut path = vec![];
            let mut v = s;
            // white を辿る
            while state[v] == 0 {
                state[v] = 1;
                pos[v] = path.len();
                path.push(v);
                v = next[v];
            }
            if state[v] == 1 {
                // v から現在経路のサイクル
                let start = pos[v];
                let cyc: Vec<usize> = path[start..].to_vec();
                for &u in &cyc {
                    on_cycle[u] = true;
                }
                cycles.push(cyc);
            }
            // 経路を black に
            for &u in &path {
                state[u] = 2;
            }
        }

        FunctionalGraph { on_cycle, cycles }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn random_vs_brute() {
        let mut x: u64 = 555;
        let mut rng = || {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            x
        };
        for _ in 0..300 {
            let n = 1 + (rng() as usize) % 12;
            let next: Vec<usize> = (0..n).map(|_| (rng() as usize) % n).collect();
            let fg = FunctionalGraph::new(&next);
            // brute: v は n 歩以内に v へ戻れば cycle 上
            for v in 0..n {
                let mut cur = v;
                let mut back = false;
                for _ in 0..n {
                    cur = next[cur];
                    if cur == v {
                        back = true;
                        break;
                    }
                }
                assert_eq!(fg.on_cycle[v], back, "v={v} next={next:?}");
            }
            // cycles の頂点集合と on_cycle が一致
            let mut in_cyc = vec![false; n];
            for c in &fg.cycles {
                for &u in c {
                    in_cyc[u] = true;
                }
            }
            assert_eq!(in_cyc, fg.on_cycle);
        }
    }
}
