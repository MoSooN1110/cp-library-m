// source snippet: key=articulation_points  prefix=articulation_points
// (旧実装/CamelCase。lib_ 版の旧バージョンの可能性あり)

pub fn articulation_points(g: &[Vec<usize>]) -> Vec<usize> {
    #[allow(clippy::too_many_arguments)]
    fn dfs(
        i: usize,
        p: usize,
        g: &[Vec<usize>],
        t: usize,
        visited: &mut [bool],
        prenum: &mut [usize],
        parent: &mut [usize],
        lowest: &mut [usize],
    ) {
        prenum[i] = t;
        lowest[i] = t;
        visited[i] = true;
        for &to in &g[i] {
            if !visited[to] {
                parent[to] = i;
                dfs(to, i, g, t + 1, visited, prenum, parent, lowest);
                lowest[i] = min(lowest[i], lowest[to]);
            } else if to != p {
                lowest[i] = min(lowest[i], prenum[to]);
            }
        }
    }
    let n = g.len();
    let mut visited = vec![false; n];
    let mut prenum = vec![0; n];
    let mut parent = vec![0; n];
    let mut lowest = vec![0; n];
    dfs(
        0,
        0,
        g,
        1,
        &mut visited,
        &mut prenum,
        &mut parent,
        &mut lowest,
    );
    let mut res = if (1..n).filter(|&i| parent[i] == 0).count() >= 2 {
        vec![0]
    } else {
        Vec::new()
    };
    for i in 1..n {
        let p = parent[i];
        if p != 0 && prenum[p] <= lowest[i] {
            res.push(p);
        }
    }
    res.sort();
    res.dedup();
    res
}
