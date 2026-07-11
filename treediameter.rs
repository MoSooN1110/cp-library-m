// source snippet: key=lib_treediameter  prefix=lib_treediameter

fn tree_diameter_dfs(v: usize, p: usize, depth: &mut Vec<i64>, graph: &Vec<Vec<(usize, usize)>>) {
    let mut res: (i64) = (0);
    if (p != INF as usize) {
        depth[v] = depth[p] + 1;
    } else {
        depth[v] = 0;
    }
    // println!("{:?}", v);

    for nv in graph[v].iter() {
        if (nv.0 == p || depth[nv.0] != -1) {
            continue;
        }
        tree_diameter_dfs(nv.0, v, depth, graph);
    }
    // return res;
}

fn tree_diameter(graph: &Vec<Vec<(usize, usize)>>) -> (i64, usize, usize) {
    let mut depth: Vec<i64> = vec![-1; graph.len()];
    tree_diameter_dfs(0, INF as usize, &mut depth, &graph);
    let mut d = (0, 0);
    for i in 0..depth.len() {
        if (d.1 < depth[i]) {
            d = (i, depth[i])
        }
    }
    let mut v1 = d.0;

    let mut depth: Vec<i64> = vec![-1; graph.len()];
    tree_diameter_dfs(d.0, INF as usize, &mut depth, &graph);
    let mut d = (0, 0);
    for i in 0..depth.len() {
        if (d.1 < depth[i]) {
            d = (i, depth[i])
        }
    }
    let v2 = d.0;

    return (d.1, v1, v2);
}
