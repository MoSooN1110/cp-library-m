// source snippet: key=lib_topo_sort  prefix=lib_topo_sort

fn topo_sort_dfs(
    v: usize,
    graph: &Vec<Vec<(usize, usize)>>,
    used: &mut Vec<usize>,
    data: &mut Vec<usize>,
) {
    if used[v] == 1 {
        return;
    }

    used[v] = 1;
    for i in graph[v].iter() {
        let nv = (*i).0;
        if used[nv] == 1 {
            continue;
        }
        topo_sort_dfs(nv, &graph, used, data);
    }
    data.push(v);
    return;
}
fn topo_cycle_ditect(graph: &Vec<Vec<(usize, usize)>>, sorted: &Vec<usize>) -> bool {
    let mut flg = false;
    // let mut  = 0 as usize;
    let mut n = graph.len();
    let mut num = vec![0; n];
    for i in 0..n {
        num[sorted[i]] = i;
    }
    for i in 0..n {
        let mut v = i;
        for j in 0..graph[v].len() {
            let mut nv = graph[v][j].0;
            if num[v] > num[nv] {
                flg = true;
            }
        }
    }
    return flg;
}
fn topo_sort(graph: &Vec<Vec<(usize, usize)>>) -> Vec<usize> {
    let mut n = graph.len();
    // let mut = 0 as usize;c
    let mut used = vec![0; n];
    let mut res = vec![0; 0];
    let mut flg = true;
    for i in 0..n {
        topo_sort_dfs(i, &graph, &mut used, &mut res);
    }
    res.reverse();

    let mut flg_cycle = true;
    if flg_cycle {
        if topo_cycle_ditect(&graph, &res) {
            return vec![0; 0];
        }
    }
    return res;
}
