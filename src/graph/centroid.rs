//! 木の重心と重心分解（centroid decomposition）。すべて反復実装でスタック安全。
//!
//! - `tree_centroids`: 木全体の重心（1 個または 2 個）
//! - `centroid_decomposition`: 再帰的な重心分解（重心木の親配列・分解順）
//!
//! ```
//! use cplib::graph::centroid::*;
//! // パス 0-1-2-3: 重心は 1 と 2
//! let adj = vec![vec![1], vec![0, 2], vec![1, 3], vec![2]];
//! assert_eq!(tree_centroids(&adj), vec![1, 2]);
//! let cd = centroid_decomposition(&adj);
//! assert_eq!(cd.parent[cd.root], None);
//! assert_eq!(cd.order.len(), 4);
//! ```

/// 木全体の重心を昇順で返す（1 個または 2 個）。
/// 重心 = 取り除いたとき残る各成分のサイズがすべて n/2 以下になる頂点。
pub fn tree_centroids(adj: &[Vec<usize>]) -> Vec<usize> {
    let n = adj.len();
    if n == 0 {
        return vec![];
    }
    // 0 を根とした preorder と親
    let mut pre: Vec<(usize, usize)> = vec![(0, usize::MAX)];
    let mut i = 0;
    while i < pre.len() {
        let (v, p) = pre[i];
        i += 1;
        for &to in &adj[v] {
            if to != p {
                pre.push((to, v));
            }
        }
    }
    let mut size = vec![1usize; n];
    let mut max_child = vec![0usize; n];
    for &(v, p) in pre.iter().rev() {
        if p != usize::MAX {
            size[p] += size[v];
            max_child[p] = max_child[p].max(size[v]);
        }
    }
    (0..n)
        .filter(|&v| max_child[v].max(n - size[v]) * 2 <= n)
        .collect()
}

/// 重心分解の結果。
pub struct CentroidDecomp {
    /// 重心木の根（木全体の重心）
    pub root: usize,
    /// 重心木での親。根は None
    pub parent: Vec<Option<usize>>,
    /// 分解した順（親は必ず子より前に現れる）
    pub order: Vec<usize>,
}

/// 連結な木の重心分解を行う。O(n log n)。
pub fn centroid_decomposition(adj: &[Vec<usize>]) -> CentroidDecomp {
    let n = adj.len();
    let mut removed = vec![false; n];
    let mut parent = vec![None; n];
    let mut order = Vec::with_capacity(n);
    let mut size = vec![0usize; n];
    let mut root = 0;
    if n == 0 {
        return CentroidDecomp {
            root,
            parent,
            order,
        };
    }
    // (成分内の任意の頂点, 重心木での親)
    let mut work: Vec<(usize, Option<usize>)> = vec![(0, None)];
    while let Some((start, cpar)) = work.pop() {
        let c = component_centroid(adj, &removed, &mut size, start);
        parent[c] = cpar;
        if cpar.is_none() {
            root = c;
        }
        order.push(c);
        removed[c] = true;
        for &to in &adj[c] {
            if !removed[to] {
                work.push((to, Some(c)));
            }
        }
    }
    CentroidDecomp {
        root,
        parent,
        order,
    }
}

/// removed を除いた start の連結成分の重心を 1 つ返す。
fn component_centroid(
    adj: &[Vec<usize>],
    removed: &[bool],
    size: &mut [usize],
    start: usize,
) -> usize {
    // preorder（親つき）で部分木サイズを計算
    let mut pre: Vec<(usize, usize)> = vec![(start, usize::MAX)];
    let mut i = 0;
    while i < pre.len() {
        let (v, p) = pre[i];
        i += 1;
        for &to in &adj[v] {
            if to != p && !removed[to] {
                pre.push((to, v));
            }
        }
    }
    for &(v, _) in &pre {
        size[v] = 1;
    }
    for &(v, p) in pre.iter().rev() {
        if p != usize::MAX {
            size[p] += size[v];
        }
    }
    let total = size[start];
    // サイズ total/2 超の子側へ降りていくと重心に到達する
    let mut v = start;
    let mut p = usize::MAX;
    'walk: loop {
        for &to in &adj[v] {
            if to != p && !removed[to] && size[to] * 2 > total {
                p = v;
                v = to;
                continue 'walk;
            }
        }
        break;
    }
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    fn random_tree(n: usize, rng: &mut impl FnMut() -> u64) -> Vec<Vec<usize>> {
        let mut adj = vec![vec![]; n];
        for v in 1..n {
            let p = (rng() as usize) % v;
            adj[p].push(v);
            adj[v].push(p);
        }
        adj
    }

    fn component(adj: &[Vec<usize>], removed: &[bool], start: usize) -> Vec<usize> {
        let mut seen = vec![start];
        let mut visited: std::collections::HashSet<usize> = seen.iter().copied().collect();
        let mut i = 0;
        while i < seen.len() {
            let v = seen[i];
            i += 1;
            for &to in &adj[v] {
                if !removed[to] && visited.insert(to) {
                    seen.push(to);
                }
            }
        }
        seen
    }

    #[test]
    fn centroids_known() {
        // パス 0-1-2-3 → {1,2}、スター → 中心のみ
        let path = vec![vec![1], vec![0, 2], vec![1, 3], vec![2]];
        assert_eq!(tree_centroids(&path), vec![1, 2]);
        let star = vec![vec![1, 2, 3, 4], vec![0], vec![0], vec![0], vec![0]];
        assert_eq!(tree_centroids(&star), vec![0]);
        let single = vec![vec![]];
        assert_eq!(tree_centroids(&single), vec![0]);
    }

    #[test]
    fn centroids_random_vs_brute() {
        let mut x: u64 = 271828182845;
        let mut rng = move || {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            x
        };
        for _ in 0..50 {
            let n = 1 + (rng() as usize) % 30;
            let adj = random_tree(n, &mut rng);
            // brute force: 各頂点を取り除いて最大成分サイズを計算
            let mut brute = vec![];
            for v in 0..n {
                let mut removed = vec![false; n];
                removed[v] = true;
                let mut visited = vec![false; n];
                visited[v] = true;
                let mut max_part = 0;
                for s in 0..n {
                    if !visited[s] {
                        let comp = component(&adj, &removed, s);
                        for &u in &comp {
                            visited[u] = true;
                        }
                        max_part = max_part.max(comp.len());
                    }
                }
                if max_part * 2 <= n {
                    brute.push(v);
                }
            }
            assert_eq!(tree_centroids(&adj), brute);
        }
    }

    #[test]
    fn decomposition_properties() {
        let mut x: u64 = 161803398875;
        let mut rng = move || {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            x
        };
        for _ in 0..30 {
            let n = 1 + (rng() as usize) % 60;
            let adj = random_tree(n, &mut rng);
            let cd = centroid_decomposition(&adj);

            // order は全頂点をちょうど 1 回ずつ
            let mut sorted = cd.order.clone();
            sorted.sort();
            assert_eq!(sorted, (0..n).collect::<Vec<_>>());

            // 分解を再現しつつ重心性を検証
            let mut removed = vec![false; n];
            for &c in &cd.order {
                let comp = component(&adj, &removed, c);
                let cs = comp.len();
                match cd.parent[c] {
                    None => {
                        assert_eq!(c, cd.root);
                        assert_eq!(cs, n);
                    }
                    Some(p) => assert!(removed[p], "parent removed before child"),
                }
                removed[c] = true;
                for &to in &adj[c] {
                    if !removed[to] {
                        let sub = component(&adj, &removed, to);
                        assert!(
                            sub.len() * 2 <= cs,
                            "component after removal must be <= half"
                        );
                    }
                }
            }

            // 重心木の深さは O(log n)
            let limit = 2 * (usize::BITS - n.leading_zeros()) as usize + 2;
            for v in 0..n {
                let mut d = 0;
                let mut cur = v;
                while let Some(p) = cd.parent[cur] {
                    cur = p;
                    d += 1;
                    assert!(d <= n);
                }
                assert_eq!(cur, cd.root);
                assert!(d <= limit, "depth {d} exceeds {limit} for n={n}");
            }
        }
    }
}
