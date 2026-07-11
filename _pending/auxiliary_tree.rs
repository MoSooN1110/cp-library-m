// source snippet: key=lib_auxiliary_tree  prefix=lib_auxiliary_tree

fn get_euler_tour_dfs(
    graph: &Vec<Vec<(usize, usize)>>,
    v: usize,
    parent: usize,
    tour: &mut Vec<usize>,
) {
    tour.push(v); // 現在の頂点をツアーに追加
    for &(neighbor, _weight) in &graph[v] {
        if neighbor != parent {
            // 親ノードに戻らないようにする
            get_euler_tour_dfs(graph, neighbor, v, tour); // 再帰的にDFS
            tour.push(v); // 戻りがけにも現在の頂点をツアーに追加
        }
    }
}

fn get_euler_tour(graph: &Vec<Vec<(usize, usize)>>) -> Vec<usize> {
    const FIRST_VISIT_IDX_MODE: bool = false;

    let n = graph.len();
    if n == 0 {
        return vec![];
    }
    let mut tour = Vec::new();
    get_euler_tour_dfs(graph, 0, usize::MAX, &mut tour); // 根ノードからDFS開始

    if FIRST_VISIT_IDX_MODE {
        let mut visited = vec![false; n];
        let mut tour_first = vec![];
        for &v in &tour {
            if !visited[v] {
                visited[v] = true;
                tour_first.push(v);
            }
        }
        tour = tour_first;
    }

    tour
}

struct LCA {
    tree: Vec<Vec<usize>>,
    parent: Vec<Vec<Option<usize>>>,
    depth: Vec<usize>,
}

impl LCA {
    pub fn new(n: usize) -> Self {
        let mut log_n = (n as f64).log2().ceil() as usize;
        if log_n == 0 {
            log_n = 1;
        }
        assert!(log_n > 0);
        LCA {
            tree: vec![vec![]; n],
            parent: vec![vec![None; n]; log_n],
            depth: vec![0; n],
        }
    }
    pub fn connect(&mut self, u: usize, v: usize) {
        self.tree[u].push(v);
        // self.tree[v].push(u);
    }
    pub fn graph_input(&mut self, graph: &Vec<Vec<(usize, usize)>>) {
        let n = graph.len();
        for i in 0..n {
            for &x in graph[i].iter() {
                let v = i;
                let nv = x.0;
                self.connect(v, nv);
            }
        }
    }
    // store direct parent and depth
    fn dfs(&mut self, u: usize, parent: Option<usize>, depth: usize) {
        self.parent[0][u] = parent;
        self.depth[u] = depth;
        for i in 0..self.tree[u].len() {
            let v = self.tree[u][i];
            if Some(v) != parent {
                self.dfs(v, Some(u), depth + 1);
            }
        }
    }
    pub fn build(&mut self, root: usize) {
        self.dfs(root, None, 0);

        let mut k = 0;
        while k + 1 < self.parent.len() {
            for u in 0..self.tree.len() {
                if self.parent[k][u].is_some() {
                    self.parent[k + 1][u] = self.parent[k][self.parent[k][u].unwrap()]
                }
            }
            k += 1;
        }
    }
    pub fn lca(&self, u: usize, v: usize) -> usize {
        let (mut v0, mut v1) = if self.depth[u] <= self.depth[v] {
            (u, v)
        } else {
            (v, u)
        };
        assert!(self.depth[v1] >= self.depth[v0]);

        // move v1 up until depth of v0 and v1 gets equal.
        for k in 0..self.parent.len() {
            if (((self.depth[v1] - self.depth[v0]) >> k) & 1) > 0 {
                assert!(self.parent[k][v1].is_some());
                v1 = self.parent[k][v1].unwrap();
            }
        }
        assert!(self.depth[v1] >= self.depth[v0]);
        assert!(self.depth[v1] == self.depth[v0]);
        if (v0 == v1) {
            return v0;
        }
        for k in (0..self.parent.len()).rev() {
            // LCA's parent is LCA
            if self.parent[k][v0] != self.parent[k][v1] {
                assert!(self.parent[k][v0].is_some());
                assert!(self.parent[k][v1].is_some());
                v0 = self.parent[k][v0].unwrap();
                v1 = self.parent[k][v1].unwrap();
            }
        }
        return self.parent[0][v0].unwrap();
    }
    pub fn distance(&self, u: usize, v: usize) -> usize {
        self.depth[u] + self.depth[v] - 2 * self.depth[self.lca(u, v)]
    }
    pub fn find_kth_parent(&self, u: usize, k: usize) -> Option<usize> {
        let mut v = u;
        let mut k = k;
        for i in (0..self.parent.len()).rev() {
            if (k & (1 << i)) > 0 {
                if self.parent[i][v] == None {
                    return None;
                }
                v = self.parent[i][v].unwrap();
            }
        }
        return Some(v);
    }
}

fn build_auxiliary_tree(
    graph: &Vec<Vec<(usize, usize)>>,
    color: &Vec<usize>,
) -> Vec<(usize, HashMap<usize, Vec<(usize, usize)>>)> {
    // c.f. https://smijake3.hatenablog.com/entry/2019/09/15/200200
    let n = graph.len();
    let mut lca = LCA::new(n);
    lca.graph_input(&graph);
    lca.build(0);

    let euler_tour = get_euler_tour(&graph);
    let mut euler_tour_first = vec![UINF; n];
    let mut euler_tour_last = vec![0; n];
    for i in 0..euler_tour.len() {
        let v = euler_tour[i];
        euler_tour_first[v] = cmp::min(euler_tour_first[v], i);
        euler_tour_last[v] = cmp::max(euler_tour_last[v], i);
    }

    let mut node_groups_tmp = vec![vec![]; n];
    for i in 0..n {
        node_groups_tmp[color[i]].push(i);
    }
    let mut node_groups = vec![];
    for i in 0..node_groups_tmp.len() {
        if node_groups_tmp[i].len() > 0 {
            node_groups.push(node_groups_tmp[i].clone());
        }
    }
    let mut auxiliary_tree = vec![];

    for i in 0..node_groups.len() {
        let mut tree = HashMap::new();
        let mut nodes = node_groups[i].clone();
        let cur_color = color[nodes[0]];
        tree.insert(nodes[0], vec![]);
        nodes.sort_by_key(|&x| euler_tour_first[x]);
        for j in 0..nodes.len() - 1 {
            let v = nodes[j];
            let u = nodes[j + 1];
            nodes.push(lca.lca(v, u));
        }
        nodes.sort_by_key(|&x| euler_tour_first[x]);
        nodes.dedup();
        let mut stack = vec![];
        for &v in &nodes {
            while let Some(&top) = stack.last() {
                if euler_tour_last[top] >= euler_tour_first[v] {
                    break;
                }
                stack.pop();
            }
            if let Some(&top) = stack.last() {
                tree.entry(top).or_insert(vec![]).push((v, 1));
                tree.entry(v).or_insert(vec![]).push((top, 1));
            }
            stack.push(v);
        }

        auxiliary_tree.push((cur_color, tree));
    }
    auxiliary_tree
}
