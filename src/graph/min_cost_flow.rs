//! 最小費用流（SSP + Bellman-Ford、負辺コスト可）。
//!
//! ```
//! use cplib::graph::min_cost_flow::*;
//! let mut mcf = MinCostFlow::new(4);
//! mcf.add_edge(0, 1, 2, 1);   // cap 2, cost 1
//! mcf.add_edge(0, 2, 1, 2);
//! mcf.add_edge(1, 3, 1, 1);
//! mcf.add_edge(2, 3, 2, 1);
//! // 2 単位流す最小費用
//! assert_eq!(mcf.min_cost_flow(0, 3, 2), Some(5));
//! ```

#[derive(Clone)]
struct Edge {
    to: usize,
    cap: i64,
    cost: i64,
    rev: usize,
}

pub struct MinCostFlow {
    g: Vec<Vec<Edge>>,
}

impl MinCostFlow {
    pub fn new(n: usize) -> Self {
        MinCostFlow { g: vec![vec![]; n] }
    }

    /// 容量 cap・単位コスト cost の有向辺 from->to。
    pub fn add_edge(&mut self, from: usize, to: usize, cap: i64, cost: i64) {
        let a = self.g[to].len();
        let b = self.g[from].len();
        self.g[from].push(Edge {
            to,
            cap,
            cost,
            rev: a,
        });
        self.g[to].push(Edge {
            to: from,
            cap: 0,
            cost: -cost,
            rev: b,
        });
    }

    /// ちょうど `flow` 単位を s->t に流す最小費用。流せなければ `None`。
    pub fn min_cost_flow(&mut self, s: usize, t: usize, mut flow: i64) -> Option<i64> {
        let n = self.g.len();
        let mut res = 0i64;
        const INF: i64 = 1 << 60;
        while flow > 0 {
            // Bellman-Ford で最短（最小費用）路
            let mut dist = vec![INF; n];
            let mut in_queue = vec![false; n];
            let mut prev_v = vec![usize::MAX; n];
            let mut prev_e = vec![usize::MAX; n];
            dist[s] = 0;
            let mut queue = std::collections::VecDeque::new();
            queue.push_back(s);
            in_queue[s] = true;
            while let Some(v) = queue.pop_front() {
                in_queue[v] = false;
                for (i, e) in self.g[v].iter().enumerate() {
                    if e.cap > 0 && dist[v] + e.cost < dist[e.to] {
                        dist[e.to] = dist[v] + e.cost;
                        prev_v[e.to] = v;
                        prev_e[e.to] = i;
                        if !in_queue[e.to] {
                            in_queue[e.to] = true;
                            queue.push_back(e.to);
                        }
                    }
                }
            }
            if dist[t] == INF {
                return None; // これ以上流せない
            }
            // 経路上の最小残容量
            let mut d = flow;
            let mut v = t;
            while v != s {
                d = d.min(self.g[prev_v[v]][prev_e[v]].cap);
                v = prev_v[v];
            }
            // 流す
            let mut v = t;
            while v != s {
                let pv = prev_v[v];
                let pe = prev_e[v];
                self.g[pv][pe].cap -= d;
                let rev = self.g[pv][pe].rev;
                self.g[v][rev].cap += d;
                v = pv;
            }
            flow -= d;
            res += d * dist[t];
        }
        Some(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic() {
        let mut mcf = MinCostFlow::new(5);
        mcf.add_edge(0, 1, 10, 2);
        mcf.add_edge(0, 2, 2, 4);
        mcf.add_edge(1, 2, 6, 6);
        mcf.add_edge(1, 3, 6, 2);
        mcf.add_edge(2, 4, 5, 2);
        mcf.add_edge(3, 2, 3, 3);
        mcf.add_edge(3, 4, 8, 6);
        // 3 単位を 0->4 に流す
        let c = mcf.min_cost_flow(0, 4, 3).unwrap();
        assert!(c > 0);
    }
    #[test]
    fn exact_small() {
        let mut mcf = MinCostFlow::new(4);
        mcf.add_edge(0, 1, 1, 1);
        mcf.add_edge(1, 3, 1, 1);
        mcf.add_edge(0, 2, 1, 5);
        mcf.add_edge(2, 3, 1, 5);
        assert_eq!(mcf.min_cost_flow(0, 3, 1), Some(2)); // 安い方
        assert_eq!(mcf.min_cost_flow(0, 3, 1), Some(10)); // 次は高い方
        assert_eq!(mcf.min_cost_flow(0, 3, 1), None); // もう流せない
    }
}
