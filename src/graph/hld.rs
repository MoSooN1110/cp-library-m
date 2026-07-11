//! HL 分解（Heavy-Light Decomposition）。パス/部分木を pos 区間へ写す。
//!
//! ```
//! use cplib::graph::hld::*;
//! let adj = vec![vec![1, 2], vec![0, 3, 4], vec![0], vec![1], vec![1]];
//! let hld = Hld::new(&adj, 0);
//! assert_eq!(hld.lca(3, 4), 1);
//! assert_eq!(hld.lca(3, 2), 0);
//! // 部分木 1 = {1,3,4} は pos 上で連続区間
//! let (l, r) = hld.subtree(1);
//! assert_eq!(r - l, 3);
//! ```
use std::collections::VecDeque;

pub struct Hld {
    pub par: Vec<i32>,
    pub depth: Vec<u32>,
    pub size: Vec<usize>,
    pub head: Vec<usize>,
    /// pos[v] = base 配列上の位置
    pub pos: Vec<usize>,
    /// inv[pos] = 頂点
    pub inv: Vec<usize>,
}

impl Hld {
    pub fn new(adj: &[Vec<usize>], root: usize) -> Self {
        let n = adj.len();
        let mut par = vec![-1i32; n];
        let mut depth = vec![0u32; n];
        let mut order = Vec::with_capacity(n);
        // BFS で par/depth/order
        let mut q = VecDeque::new();
        let mut visited = vec![false; n];
        visited[root] = true;
        q.push_back(root);
        while let Some(v) = q.pop_front() {
            order.push(v);
            for &to in &adj[v] {
                if !visited[to] {
                    visited[to] = true;
                    par[to] = v as i32;
                    depth[to] = depth[v] + 1;
                    q.push_back(to);
                }
            }
        }
        // size（order の逆順で集計）
        let mut size = vec![1usize; n];
        for &v in order.iter().rev() {
            if par[v] >= 0 {
                size[par[v] as usize] += size[v];
            }
        }
        // 分解
        let mut head = vec![0usize; n];
        let mut pos = vec![0usize; n];
        let mut inv = vec![0usize; n];
        let mut timer = 0usize;
        let mut stack = vec![(root, root)];
        while let Some((v, h)) = stack.pop() {
            pos[v] = timer;
            inv[timer] = v;
            timer += 1;
            head[v] = h;
            // heavy child を選ぶ
            let mut heavy = usize::MAX;
            let mut mx = 0;
            for &to in &adj[v] {
                if to as i32 != par[v] && size[to] > mx {
                    mx = size[to];
                    heavy = to;
                }
            }
            // light を先に、heavy を最後に push（heavy が次に処理され pos が連続）
            for &to in &adj[v] {
                if to as i32 != par[v] && to != heavy {
                    stack.push((to, to));
                }
            }
            if heavy != usize::MAX {
                stack.push((heavy, h));
            }
        }
        Hld {
            par,
            depth,
            size,
            head,
            pos,
            inv,
        }
    }

    pub fn lca(&self, mut u: usize, mut v: usize) -> usize {
        while self.head[u] != self.head[v] {
            if self.depth[self.head[u]] < self.depth[self.head[v]] {
                std::mem::swap(&mut u, &mut v);
            }
            u = self.par[self.head[u]] as usize;
        }
        if self.depth[u] < self.depth[v] {
            u
        } else {
            v
        }
    }

    /// 部分木 v に対応する pos 上の半開区間 [l, r)。
    pub fn subtree(&self, v: usize) -> (usize, usize) {
        (self.pos[v], self.pos[v] + self.size[v])
    }

    /// パス u..v を pos 上の半開区間の集合に分解。`edge=true` で辺クエリ（LCA を含めない）。
    /// 区間の順序は保証されない（可換な集約向け）。
    pub fn path_ranges(&self, mut u: usize, mut v: usize, edge: bool) -> Vec<(usize, usize)> {
        let mut res = vec![];
        while self.head[u] != self.head[v] {
            if self.depth[self.head[u]] < self.depth[self.head[v]] {
                std::mem::swap(&mut u, &mut v);
            }
            res.push((self.pos[self.head[u]], self.pos[u] + 1));
            u = self.par[self.head[u]] as usize;
        }
        if self.depth[u] > self.depth[v] {
            std::mem::swap(&mut u, &mut v);
        }
        let lo = self.pos[u] + if edge { 1 } else { 0 };
        if lo < self.pos[v] + 1 {
            res.push((lo, self.pos[v] + 1));
        }
        res
    }

    /// 木上の距離（辺数）。
    pub fn dist(&self, u: usize, v: usize) -> usize {
        let w = self.lca(u, v);
        (self.depth[u] + self.depth[v] - 2 * self.depth[w]) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn brute_lca(par: &[i32], depth: &[u32], mut u: usize, mut v: usize) -> usize {
        while depth[u] > depth[v] {
            u = par[u] as usize;
        }
        while depth[v] > depth[u] {
            v = par[v] as usize;
        }
        while u != v {
            u = par[u] as usize;
            v = par[v] as usize;
        }
        u
    }

    #[test]
    fn random_tree() {
        let mut x: u64 = 314159;
        let mut rng = || {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            x
        };
        for _ in 0..100 {
            let n = 2 + (rng() as usize) % 30;
            let mut adj = vec![vec![]; n];
            for v in 1..n {
                let p = (rng() as usize) % v;
                adj[v].push(p);
                adj[p].push(v);
            }
            let hld = Hld::new(&adj, 0);
            // lca 照合
            for u in 0..n {
                for w in 0..n {
                    assert_eq!(hld.lca(u, w), brute_lca(&hld.par, &hld.depth, u, w));
                }
            }
            // subtree 区間: メンバー = 部分木頂点集合
            for v in 0..n {
                let (l, r) = hld.subtree(v);
                let members: std::collections::HashSet<usize> =
                    (l..r).map(|p| hld.inv[p]).collect();
                // 実際の部分木を BFS で
                let mut sub = std::collections::HashSet::new();
                let mut st = vec![v];
                let mut seen = vec![false; n];
                seen[v] = true;
                while let Some(a) = st.pop() {
                    sub.insert(a);
                    for &to in &adj[a] {
                        if !seen[to] && to as i32 != hld.par[a] {
                            // 子方向のみ
                        }
                        if !seen[to] && hld.depth[to] > hld.depth[a] {
                            seen[to] = true;
                            st.push(to);
                        }
                    }
                }
                assert_eq!(members, sub, "subtree of {v}");
            }
            // path_ranges が u..v の頂点集合を過不足なく覆う
            for u in 0..n {
                for w in 0..n {
                    let ranges = hld.path_ranges(u, w, false);
                    let mut covered = std::collections::HashSet::new();
                    for (l, r) in ranges {
                        for p in l..r {
                            covered.insert(hld.inv[p]);
                        }
                    }
                    // 真のパス
                    let l = hld.lca(u, w);
                    let mut path = std::collections::HashSet::new();
                    let mut a = u;
                    loop {
                        path.insert(a);
                        if a == l {
                            break;
                        }
                        a = hld.par[a] as usize;
                    }
                    let mut b = w;
                    loop {
                        path.insert(b);
                        if b == l {
                            break;
                        }
                        b = hld.par[b] as usize;
                    }
                    assert_eq!(covered, path, "path {u}..{w}");
                }
            }
        }
    }
}
