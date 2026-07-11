//! 最大流（Dinic 法）。
//!
//! ```
//! use cplib::graph::max_flow::*;
//! let mut mf = MaxFlow::new(4);
//! mf.add_edge(0, 1, 2);
//! mf.add_edge(0, 2, 1);
//! mf.add_edge(1, 2, 1);
//! mf.add_edge(1, 3, 1);
//! mf.add_edge(2, 3, 2);
//! assert_eq!(mf.max_flow(0, 3), 3);
//! ```
use std::collections::VecDeque;

#[derive(Clone)]
struct Edge {
    to: usize,
    cap: u64,
    rev: usize,
}

pub struct MaxFlow {
    g: Vec<Vec<Edge>>,
    level: Vec<i32>,
    iter: Vec<usize>,
}

impl MaxFlow {
    pub fn new(n: usize) -> Self {
        MaxFlow {
            g: vec![vec![]; n],
            level: vec![0; n],
            iter: vec![0; n],
        }
    }

    /// 容量 cap の有向辺 from->to を追加。
    pub fn add_edge(&mut self, from: usize, to: usize, cap: u64) {
        let a = self.g[to].len();
        let b = self.g[from].len();
        self.g[from].push(Edge { to, cap, rev: a });
        self.g[to].push(Edge {
            to: from,
            cap: 0,
            rev: b,
        });
    }

    fn bfs(&mut self, s: usize, t: usize) -> bool {
        self.level.iter_mut().for_each(|x| *x = -1);
        let mut q = VecDeque::new();
        self.level[s] = 0;
        q.push_back(s);
        while let Some(v) = q.pop_front() {
            for e in &self.g[v] {
                if e.cap > 0 && self.level[e.to] < 0 {
                    self.level[e.to] = self.level[v] + 1;
                    q.push_back(e.to);
                }
            }
        }
        self.level[t] >= 0
    }

    fn dfs(&mut self, v: usize, t: usize, f: u64) -> u64 {
        if v == t {
            return f;
        }
        while self.iter[v] < self.g[v].len() {
            let i = self.iter[v];
            let (to, cap, rev) = {
                let e = &self.g[v][i];
                (e.to, e.cap, e.rev)
            };
            if cap > 0 && self.level[v] < self.level[to] {
                let d = self.dfs(to, t, f.min(cap));
                if d > 0 {
                    self.g[v][i].cap -= d;
                    self.g[to][rev].cap += d;
                    return d;
                }
            }
            self.iter[v] += 1;
        }
        0
    }

    /// s から t への最大流。
    pub fn max_flow(&mut self, s: usize, t: usize) -> u64 {
        let mut flow = 0;
        while self.bfs(s, t) {
            self.iter.iter_mut().for_each(|x| *x = 0);
            loop {
                let f = self.dfs(s, t, u64::MAX);
                if f == 0 {
                    break;
                }
                flow += f;
            }
        }
        flow
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic() {
        let mut mf = MaxFlow::new(6);
        // classic max-flow = 5 example
        for &(u, v, c) in &[
            (0, 1, 3u64),
            (0, 2, 2),
            (1, 2, 1),
            (1, 3, 3),
            (2, 4, 2),
            (3, 4, 2),
            (3, 5, 2),
            (4, 5, 3),
        ] {
            mf.add_edge(u, v, c);
        }
        assert_eq!(mf.max_flow(0, 5), 5);
    }
    #[test]
    fn simple() {
        let mut mf = MaxFlow::new(2);
        mf.add_edge(0, 1, 10);
        assert_eq!(mf.max_flow(0, 1), 10);
    }
}
