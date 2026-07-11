// source snippet: key=lib_bipartite_matching  prefix=lib_bipartite_matching

pub fn bipartite_matching(g: &Vec<Vec<(usize, usize)>>) -> usize {
    fn dfs(
        v: usize,
        g: &Vec<Vec<(usize, usize)>>,
        mat: &mut [Option<usize>],
        used: &mut [usize],
        id: usize,
    ) -> bool {
        used[v] = id;
        for &u in &g[v] {
            if mat[u.0].is_none()
                || used[mat[u.0].unwrap()] != id && dfs(mat[u.0].unwrap(), g, mat, used, id)
            {
                mat[v] = Some(u.0);
                mat[u.0] = Some(v);
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
