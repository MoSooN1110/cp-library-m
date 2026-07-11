// source snippet: key=lib_euler_tour  prefix=lib_euler_tour

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
