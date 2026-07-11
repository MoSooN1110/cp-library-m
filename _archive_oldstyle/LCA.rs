// source snippet: key=LCA  prefix=LCA
// (旧実装/CamelCase。lib_ 版の旧バージョンの可能性あり)

#[allow(dead_code)]
pub struct LCA {
    pub depth: Vec<usize>,
    pub parent: Vec<Vec<Option<usize>>>,
}
#[allow(dead_code)]
impl LCA {
    pub fn new(g: &[Vec<usize>]) -> LCA {
        LCA::with_root(0, g)
    }
    pub fn with_root(root: usize, g: &[Vec<usize>]) -> LCA {
        fn dfs(
            i: usize,
            p: Option<usize>,
            d: usize,
            g: &[Vec<usize>],
            depth: &mut [usize],
            parent: &mut [Vec<Option<usize>>],
        ) {
            parent[i][0] = p;
            depth[i] = d;
            for &t in &g[i] {
                if Some(t) != p {
                    dfs(t, Some(i), d + 1, g, depth, parent);
                }
            }
        }
        let n = g.len();
        let l2 = (1..).find(|i| 1usize << i > n).unwrap();
        let mut depth = vec![0; n];
        let mut parent = vec![vec![None; l2 + 1]; n];
        dfs(root, None, 0, &g, &mut depth, &mut parent);
        for i in 1..l2 + 1 {
            for j in 0..n {
                if let Some(p) = parent[j][i - 1] {
                    parent[j][i] = parent[p][i - 1];
                }
            }
        }
        LCA { depth, parent }
    }
    pub fn lca(&self, mut a: usize, mut b: usize) -> usize {
        use std::mem::swap;
        if self.depth[b] < self.depth[a] {
            swap(&mut a, &mut b);
        }
        while self.depth[a] != self.depth[b] {
            b = self.parent[b][(self.depth[b] - self.depth[a]).trailing_zeros() as usize].unwrap();
        }
        if a == b {
            return a;
        }
        for i in (0..self.parent[0].len()).rev() {
            if self.parent[a][i] != self.parent[b][i] {
                a = self.parent[a][i].unwrap();
                b = self.parent[b][i].unwrap();
            }
        }
        self.parent[a][0].unwrap()
    }
}
