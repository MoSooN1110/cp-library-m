// source snippet: key=UFT  prefix=UFT
// (旧実装/CamelCase。lib_ 版の旧バージョンの可能性あり)

#[allow(dead_code)]
/// Union Find Tree
pub struct UFT {
    pub par: Vec<usize>,
    pub rank: Vec<usize>,
}
impl UFT {
    #[allow(dead_code)]
    pub fn new(n: usize) -> Self {
        UFT {
            par: (0..n).collect(),
            rank: vec![0; n],
        }
    }
    #[allow(dead_code)]
    pub fn root(&mut self, x: usize) -> usize {
        if self.par[x] == x {
            x
        } else {
            let p = self.par[x];
            let pp = self.root(p);
            self.par[x] = pp;
            pp
        }
    }
    #[allow(dead_code)]
    pub fn merge(&mut self, x: usize, y: usize) {
        let x = self.root(x);
        let y = self.root(y);
        if x == y {
            return;
        }
        if self.rank[x] < self.rank[y] {
            self.par[x] = y;
        } else {
            self.par[y] = x;
            if self.rank[x] == self.rank[y] {
                self.rank[x] += 1;
            }
        }
    }
}
