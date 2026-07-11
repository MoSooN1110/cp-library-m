// source snippet: key=lib_graph_connect_num  prefix=lib_graph_connect_num

fn DFS(v: usize, p: usize, graph: &Vec<Vec<(usize, usize)>>, used: &mut Vec<usize>) {
    used[v] = 1;
    for i in graph[v].iter() {
        let nv = (*i).0;
        if used[nv] == 1 {
            continue;
        }
        DFS(nv, v, &graph, &mut *used);
    }
    return;
}
fn solve() {
    let (n, m) = readuu();
    let mut graph = vec![vec![(0 as usize, 0 as usize); (0) as usize]; (n) as usize];
    for i in 0..m {
        let (mut a, mut b) = readuu();
        a -= 1;
        b -= 1;
        graph[a].push((b, 1));
        graph[b].push((a, 1));
    }
    let mut used = vec![0; n];
    let mut cnt: usize = 0;
    for i in 0..n {
        if (used[i] == 0) {
            DFS(i, INF as usize, &graph, &mut used);
            cnt += 1;
        }
    }
    println!("{:?}", cnt - 1);
}
