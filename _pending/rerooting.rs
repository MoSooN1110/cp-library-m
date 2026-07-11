// source snippet: key=lib_rerooting  prefix=lib_rerooting

// ---------- begin Rerooting ----------
pub trait RerootingOperator {
    type Value: Clone;
    type Edge: Clone;
    fn init(&mut self, v: usize) -> Self::Value;
    fn merge(&mut self, p: &Self::Value, c: &Self::Value, e: &Self::Edge) -> Self::Value;
}

pub struct Rerooting<R: RerootingOperator> {
    manager: R,
    size: usize,
    edge: Vec<(usize, usize, R::Edge, R::Edge)>,
}

impl<R: RerootingOperator> Rerooting<R> {
    pub fn new(size: usize, manager: R) -> Self {
        assert!(size > 0 && size < 10usize.pow(8));
        Rerooting {
            manager: manager,
            size: size,
            edge: vec![],
        }
    }
    pub fn add_edge(&mut self, a: usize, b: usize, cost: R::Edge) {
        assert!(a < self.size && b < self.size && a != b);
        self.add_edge_bi(a, b, cost.clone(), cost);
    }
    pub fn add_edge_bi(&mut self, a: usize, b: usize, ab: R::Edge, ba: R::Edge) {
        assert!(a < self.size && b < self.size && a != b);
        self.edge.push((a, b, ab, ba));
    }
    pub fn solve(&mut self) -> Vec<R::Value> {
        let size = self.size;
        let mut graph = vec![vec![]; size];
        for e in self.edge.iter() {
            graph[e.0].push((e.1, e.2.clone()));
            graph[e.1].push((e.0, e.3.clone()));
        }
        let root = 0;
        let mut topo = vec![root];
        let mut parent = vec![root; size];
        let mut parent_edge: Vec<Option<R::Edge>> = (0..size).map(|_| None).collect();
        for i in 0..size {
            let v = topo[i];
            let child = std::mem::take(&mut graph[v]);
            for e in child.iter() {
                let k = graph[e.0].iter().position(|e| e.0 == v).unwrap();
                let c = graph[e.0].remove(k).1;
                parent_edge[e.0] = Some(c);
                parent[e.0] = v;
                topo.push(e.0);
            }
            graph[v] = child;
        }
        let manager = &mut self.manager;
        let mut down: Vec<_> = (0..size).map(|v| manager.init(v)).collect();
        for &v in topo.iter().rev() {
            for e in graph[v].iter() {
                down[v] = manager.merge(&down[v], &down[e.0], &e.1);
            }
        }
        let mut up: Vec<_> = (0..size).map(|v| manager.init(v)).collect();
        let mut stack = vec![];
        for &v in topo.iter() {
            if let Some(e) = parent_edge[v].take() {
                let ini = manager.init(v);
                up[v] = manager.merge(&ini, &up[v], &e);
            }
            if !graph[v].is_empty() {
                stack.push((graph[v].as_slice(), up[v].clone()));
                while let Some((g, val)) = stack.pop() {
                    if g.len() == 1 {
                        up[g[0].0] = val;
                    } else {
                        let m = g.len() / 2;
                        let (a, b) = g.split_at(m);
                        for a in [(a, b), (b, a)].iter() {
                            let mut p = val.clone();
                            for a in a.0.iter() {
                                p = manager.merge(&p, &down[a.0], &a.1);
                            }
                            stack.push((a.1, p));
                        }
                    }
                }
            }
            for e in graph[v].iter() {
                up[v] = manager.merge(&up[v], &down[e.0], &e.1);
            }
        }
        up
    }
}
// ---------- end Rerooting ----------

// use proconio::marker::*;
// use proconio::*;
//拝借します https://atcoder.jp/contests/abc222/submissions/26448929
struct Info {
    d: Vec<u64>,
}

impl RerootingOperator for Info {
    type Value = (u64, usize);
    type Edge = u64;
    fn init(&mut self, v: usize) -> Self::Value {
        (0, v)
    }
    fn merge(&mut self, p: &Self::Value, c: &Self::Value, e: &Self::Edge) -> Self::Value {
        (std::cmp::max(p.0, c.0 + *e).max(self.d[c.1] + *e), p.1)
    }
}

// #[fastout]
// fn main() {
//     input! {
//         n: usize,
//         e: [(Usize1, Usize1, u64); n - 1],
//         d: [u64; n],
//     }
//     let mut solver = Rerooting::new(n, Info { d: d });
//     for (a, b, c) in e {
//         solver.add_edge(a, b, c);
//     }
//     let ans = solver.solve();
//     for a in ans {
//         println!("{}", a.0);
//     }
// }
