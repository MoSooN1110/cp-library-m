// source snippet: key=lib_mst  prefix=lib_mst

pub struct Dsu {
    n: usize,
    // root node: -1 * component size
    // otherwise: parent
    parent_or_size: Vec<i32>,
}

impl Dsu {
    // 0 <= size <= 10^8 is constrained.
    pub fn new(size: usize) -> Self {
        Self {
            n: size,
            parent_or_size: vec![-1; size],
        }
    }
    pub fn merge(&mut self, a: usize, b: usize) -> usize {
        assert!(a < self.n);
        assert!(b < self.n);
        let (mut x, mut y) = (self.leader(a), self.leader(b));
        if x == y {
            return x;
        }
        if -self.parent_or_size[x] < -self.parent_or_size[y] {
            std::mem::swap(&mut x, &mut y);
        }
        self.parent_or_size[x] += self.parent_or_size[y];
        self.parent_or_size[y] = x as i32;
        x
    }

    pub fn same(&mut self, a: usize, b: usize) -> bool {
        assert!(a < self.n);
        assert!(b < self.n);
        self.leader(a) == self.leader(b)
    }
    pub fn leader(&mut self, a: usize) -> usize {
        assert!(a < self.n);
        if self.parent_or_size[a] < 0 {
            return a;
        }
        self.parent_or_size[a] = self.leader(self.parent_or_size[a] as usize) as i32;
        self.parent_or_size[a] as usize
    }
    pub fn size(&mut self, a: usize) -> usize {
        assert!(a < self.n);
        let x = self.leader(a);
        -self.parent_or_size[x] as usize
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

mod kraskal {
    use crate::Dsu;

    #[derive(Clone, Copy, Debug)]
    pub struct Edge {
        pub u: usize,
        pub v: usize,
        pub cost: i64,
    }

    #[doc = "es: undirected edges. O(ElogV)"]
    pub fn kraskal(n: usize, ess: Vec<(usize, usize, usize)>) -> (Vec<Edge>, Vec<Edge>) {
        let mut used = vec![];
        let mut unused = vec![];
        let mut x = 0 as usize;
        let mut es = vec![];
        for i in 0..ess.len() {
            let mut e = Edge {
                u: ess[i].0,
                v: ess[i].1,
                cost: ess[i].2 as i64,
            };

            es.push(e);
        }
        es.sort_by_key(|x| x.cost);

        let mut uf = Dsu::new(n);

        for e in es {
            if !uf.same(e.u, e.v) {
                uf.merge(e.u, e.v);
                used.push(e);
            } else {
                unused.push(e);
            }
        }

        (used, unused)
    }
    pub fn kraskal_es(n: usize, es: Vec<Edge>) -> (Vec<Edge>, Vec<Edge>) {
        let mut used = vec![];
        let mut unused = vec![];

        let mut es = es;
        es.sort_by_key(|x| x.cost);

        let mut uf = Dsu::new(n);

        for e in es {
            if !uf.same(e.u, e.v) {
                uf.merge(e.u, e.v);
                used.push(e);
            } else {
                unused.push(e);
            }
        }

        (used, unused)
    }
}
