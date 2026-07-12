//! グラフの閉路検出と復元。有向グラフ・無向グラフに対応する。
//!
//! ```
//! use cplib::graph::cycle_detection::*;
//!
//! let g = vec![vec![1], vec![2], vec![0]];
//! assert_eq!(find_cycle_directed(&g), Some(vec![0, 1, 2, 0]));
//! ```

pub fn find_cycle_directed(adj: &[Vec<usize>]) -> Option<Vec<usize>> {
    let n = adj.len();
    let mut color = vec![0u8; n];
    let mut parent = vec![usize::MAX; n];
    for s in 0..n {
        if color[s] != 0 {
            continue;
        }
        let mut stack = vec![(s, 0usize)];
        color[s] = 1;
        while let Some((v, i)) = stack.pop() {
            if i == adj[v].len() {
                color[v] = 2;
                continue;
            }
            stack.push((v, i + 1));
            let to = adj[v][i];
            assert!(to < n);
            if color[to] == 0 {
                parent[to] = v;
                color[to] = 1;
                stack.push((to, 0));
            } else if color[to] == 1 {
                let mut cyc = vec![to];
                let mut cur = v;
                while cur != to {
                    cyc.push(cur);
                    cur = parent[cur];
                }
                cyc.push(to);
                cyc.reverse();
                return Some(cyc);
            }
        }
    }
    None
}

pub fn find_cycle_undirected(adj: &[Vec<usize>]) -> Option<Vec<usize>> {
    let n = adj.len();
    let mut seen = vec![false; n];
    let mut parent = vec![usize::MAX; n];
    for s in 0..n {
        if seen[s] {
            continue;
        }
        seen[s] = true;
        let mut stack = vec![(s, usize::MAX, 0usize)];
        while let Some((v, p, i)) = stack.pop() {
            if i == adj[v].len() {
                continue;
            }
            stack.push((v, p, i + 1));
            let to = adj[v][i];
            assert!(to < n);
            if to == p {
                continue;
            }
            if !seen[to] {
                seen[to] = true;
                parent[to] = v;
                stack.push((to, v, 0));
            } else {
                let mut path_v = vec![v];
                let mut cur = v;
                while cur != usize::MAX && cur != to {
                    cur = parent[cur];
                    path_v.push(cur);
                }
                if cur == to {
                    path_v.push(v);
                    return Some(path_v);
                }
                let mut mark = std::collections::HashMap::new();
                cur = v;
                let mut d = 0usize;
                while cur != usize::MAX {
                    mark.insert(cur, d);
                    cur = parent[cur];
                    d += 1;
                }
                let mut right = vec![to];
                cur = to;
                while !mark.contains_key(&cur) {
                    cur = parent[cur];
                    right.push(cur);
                }
                let lca = cur;
                let mut left = vec![v];
                cur = v;
                while cur != lca {
                    cur = parent[cur];
                    left.push(cur);
                }
                right.pop();
                right.reverse();
                left.extend(right);
                left.push(v);
                return Some(left);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_valid_cycle(adj: &[Vec<usize>], cyc: &[usize]) -> bool {
        cyc.len() >= 2
            && cyc.first() == cyc.last()
            && cyc.windows(2).all(|w| adj[w[0]].contains(&w[1]))
    }

    #[test]
    fn directed_known() {
        let g = vec![vec![1], vec![2], vec![0]];
        assert!(is_valid_cycle(&g, &find_cycle_directed(&g).unwrap()));
        let dag = vec![vec![1, 2], vec![2], vec![]];
        assert!(find_cycle_directed(&dag).is_none());
    }

    #[test]
    fn undirected_known() {
        let g = vec![vec![1, 2], vec![0, 2], vec![1, 0]];
        assert!(is_valid_cycle(&g, &find_cycle_undirected(&g).unwrap()));
        let tree = vec![vec![1], vec![0, 2], vec![1]];
        assert!(find_cycle_undirected(&tree).is_none());
    }
}

