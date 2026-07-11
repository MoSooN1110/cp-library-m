// source snippet: key=bridges  prefix=bridges
// (旧実装/CamelCase。lib_ 版の旧バージョンの可能性あり)

pub fn bridges(g: &[Vec<usize>]) -> Vec<(usize, usize)> {
    #[allow(clippy::too_many_arguments)]
    fn dfs(
        i: usize,
        p: usize,
        g: &[Vec<usize>],
        t: usize,
        visited: &mut [bool],
        pre: &mut [usize],
        low: &mut [usize],
        res: &mut Vec<(usize, usize)>,
    ) -> usize {
        visited[i] = true;
        pre[i] = t;
        low[i] = t;
        for &to in &g[i] {
            if p != to {
                if visited[to] {
                    low[i] = min(low[i], low[to]);
                } else {
                    low[i] = min(low[i], dfs(to, i, g, t + 1, visited, pre, low, res));
                    if low[to] == pre[to] {
                        res.push((i, to));
                    }
                }
            }
        }
        low[i]
    }
    let n = g.len();
    let mut visited = vec![false; n];
    let mut pre = vec![0; n];
    let mut low = vec![0; n];
    let mut res = Vec::new();
    dfs(0, 0, g, 0, &mut visited, &mut pre, &mut low, &mut res);
    res
}
