// source snippet: key=lib_weighted_dsu  prefix=lib_weighted_dsu

//http://judge.u-aizu.ac.jp/onlinejudge/review.jsp?rid=4941093#1
#[derive(Clone, Debug)]
enum DsuNode {
    Root(usize),
    Node((usize, i64)),
}

pub struct WeightedDsu {
    n: usize,
    parent_or_size: Vec<DsuNode>,
}

impl WeightedDsu {
    // 0 <= size <= 10^8 is constrained.
    pub fn new(size: usize) -> Self {
        Self {
            n: size,
            parent_or_size: vec![DsuNode::Root(1); size],
        }
    }

    pub fn merge(&mut self, a: usize, b: usize, mut d: i64) -> usize {
        assert!(a < self.n);
        assert!(b < self.n);
        let (mut x, mut y) = (self.leader(a), self.leader(b));
        if x == y {
            return x;
        }
        d += self.weight(a);
        d -= self.weight(b);
        if let (DsuNode::Root(sx), DsuNode::Root(sy)) =
            (&self.parent_or_size[x], &self.parent_or_size[y])
        {
            if sx < sy {
                std::mem::swap(&mut x, &mut y);
                d = -d
            }
            self.parent_or_size[x] = DsuNode::Root(sx + sy);
            self.parent_or_size[y] = DsuNode::Node((x, d));
        }
        x
    }

    pub fn same(&mut self, a: usize, b: usize) -> bool {
        assert!(a < self.n);
        assert!(b < self.n);
        self.leader(a) == self.leader(b)
    }

    pub fn leader(&mut self, a: usize) -> usize {
        assert!(a < self.n);
        if let DsuNode::Node((b, d)) = self.parent_or_size[a] {
            let x = self.leader(b);
            self.parent_or_size[a] = if let DsuNode::Node((_, e)) = self.parent_or_size[b] {
                DsuNode::Node((x, d + e))
            } else {
                DsuNode::Node((x, d))
            };
            x
        } else {
            a
        }
    }

    pub fn weight(&mut self, a: usize) -> i64 {
        self.leader(a);
        if let DsuNode::Node((_, d)) = self.parent_or_size[a] {
            d
        } else {
            0
        }
    }

    pub fn diff(&mut self, a: usize, b: usize) -> i64 {
        self.weight(b) - self.weight(a)
    }

    pub fn size(&mut self, a: usize) -> usize {
        assert!(a < self.n);
        let x = self.leader(a);
        if let DsuNode::Root(size) = self.parent_or_size[x] {
            size
        } else {
            panic!();
        }
    }

    pub fn groups(&mut self) -> Vec<Vec<usize>> {
        let mut leader_buf = vec![0; self.n];
        let mut group_size = vec![0; self.n];
        for i in 0..self.n {
            leader_buf[i] = self.leader(i);
            group_size[leader_buf[i]] += 1;
        }
        let mut result = vec![Vec::new(); self.n];
        for i in 0..self.n {
            result[i].reserve(group_size[i]);
        }
        for i in 0..self.n {
            result[leader_buf[i]].push(i);
        }
        result
            .into_iter()
            .filter(|x| !x.is_empty())
            .collect::<Vec<Vec<usize>>>()
    }
}
