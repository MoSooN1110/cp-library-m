// source snippet: key=bipartite_matching  prefix=bipartite_matching
// (旧実装/CamelCase。lib_ 版の旧バージョンの可能性あり)

#[allow(dead_code)]
pub fn bipartite_matching(g: &[Vec<usize>]) -> usize {
    fn dfs(
        v: usize,
        g: &[Vec<usize>],
        mat: &mut [Option<usize>],
        used: &mut [usize],
        id: usize,
    ) -> bool {
        used[v] = id;
        for &u in &g[v] {
            if mat[u].is_none()
                || used[mat[u].unwrap()] != id && dfs(mat[u].unwrap(), g, mat, used, id)
            {
                mat[v] = Some(u);
                mat[u] = Some(v);
                return true;
            }
        }
        false
    }
    let mut res = 0;
    let mut mat = vec![None; g.len()];
    let mut used = vec![0; g.len()];
    for v in 0..g.len() {
        if mat[v].is_none() && dfs(v, g, &mut mat, &mut used, v + 1) {
            res += 1;
        }
    }
    res
}
