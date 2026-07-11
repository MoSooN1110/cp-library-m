// source snippet: key=lib_scc  prefix=lib_scc

pub struct SCC {
    g: Vec<Vec<usize>>,
    r_g: Vec<Vec<usize>>,
    post_order: VecDeque<usize>,
    used: Vec<bool>,
    pub order: Vec<usize>,
}

impl SCC {
    pub fn new(n: usize) -> Self {
        Self {
            g: vec![vec![]; n],
            r_g: vec![vec![]; n],
            post_order: VecDeque::new(),
            used: vec![false; n],
            order: vec![n; n],
        }
    }
    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.g[u].push(v);
        self.r_g[v].push(u);
    }
    fn dfs(&mut self, u: usize) {
        self.used[u] = true;
        for i in 0..self.g[u].len() {
            let v = self.g[u][i];
            if !self.used[v] {
                self.dfs(v);
            }
        }
        self.post_order.push_front(u);
    }
    fn rdfs(&mut self, u: usize, k: usize) {
        self.used[u] = true;
        self.order[u] = k;
        for i in 0..self.r_g[u].len() {
            let v = self.r_g[u][i];
            if !self.used[v] {
                self.rdfs(v, k);
            }
        }
    }
    pub fn build(&mut self) {
        for v in 0..self.g.len() {
            if !self.used[v] {
                self.dfs(v);
            }
        }
        // dbg!(&self.post_order);
        self.used = vec![false; self.g.len()];
        let mut k = 0;
        for i in 0..self.post_order.len() {
            let v = self.post_order[i];
            if !self.used[v] {
                self.rdfs(v, k);
                k += 1;
            }
        }
    }
}

fn scc_if(graph: &Vec<Vec<(usize, usize)>>) -> (Vec<usize>, Vec<Vec<(usize, usize)>>) {
    let mut res = 0;
    let n = graph.len();
    let mut scc = SCC::new(n);
    let mut es = vec![];
    for i in 0..n {
        for j in 0..graph[i].len() {
            let a = i;
            let b = graph[i][j].0;
            scc.add_edge(a, b);
            es.push((a, b));
        }
    }

    scc.build();
    // d!(scc.order);
    let mut nn = 0;
    for i in 0..scc.order.len() {
        nn = max(scc.order[i], nn);
    }
    nn += 1;
    let mut graph2 = vec![vec![(0 as usize, 0 as usize); (0) as usize]; (nn) as usize];
    let mut st = BTreeSet::new();
    for i in es {
        let a = scc.order[i.0];
        let b = scc.order[i.1];
        if st.contains(&(a, b)) {
            continue;
        }
        st.insert((a, b));
        graph2[a].push((b, 1));
    }

    (scc.order, graph2)
}
