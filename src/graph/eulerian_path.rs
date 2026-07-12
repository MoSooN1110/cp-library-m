//! オイラー路/閉路（全辺をちょうど 1 回通る walk）を Hierholzer 法で構成する。
//!
//! ```
//! use cplib::graph::eulerian_path::*;
//!
//! let edges = [(0, 1), (1, 2), (2, 0)];
//! let path = eulerian_trail_directed(3, &edges).unwrap();
//! assert_eq!(path.len(), edges.len() + 1);
//! ```

pub fn eulerian_trail_directed(n: usize, edges: &[(usize, usize)]) -> Option<Vec<usize>> {
    if edges.is_empty() {
        return Some(if n == 0 { vec![] } else { vec![0] });
    }
    let mut indeg = vec![0i32; n];
    let mut outdeg = vec![0i32; n];
    let mut adj = vec![vec![]; n];
    for (id, &(u, v)) in edges.iter().enumerate() {
        assert!(u < n && v < n);
        outdeg[u] += 1;
        indeg[v] += 1;
        adj[u].push((v, id));
    }
    let mut start = edges[0].0;
    let mut plus = 0;
    let mut minus = 0;
    for v in 0..n {
        match outdeg[v] - indeg[v] {
            1 => {
                plus += 1;
                start = v;
            }
            -1 => minus += 1,
            0 => {}
            _ => return None,
        }
    }
    if !((plus == 1 && minus == 1) || (plus == 0 && minus == 0)) {
        return None;
    }
    let mut used = vec![false; edges.len()];
    let mut stack = vec![start];
    let mut path = vec![];
    while let Some(&v) = stack.last() {
        while let Some((to, id)) = adj[v].pop() {
            if !used[id] {
                used[id] = true;
                stack.push(to);
                break;
            }
        }
        if stack.last() == Some(&v) {
            path.push(v);
            stack.pop();
        }
    }
    path.reverse();
    if path.len() == edges.len() + 1 {
        Some(path)
    } else {
        None
    }
}

pub fn eulerian_circuit_directed(n: usize, edges: &[(usize, usize)]) -> Option<Vec<usize>> {
    let path = eulerian_trail_directed(n, edges)?;
    if path.first() == path.last() {
        Some(path)
    } else {
        None
    }
}

pub fn eulerian_trail_undirected(n: usize, edges: &[(usize, usize)]) -> Option<Vec<usize>> {
    if edges.is_empty() {
        return Some(if n == 0 { vec![] } else { vec![0] });
    }
    let mut deg = vec![0usize; n];
    let mut adj = vec![vec![]; n];
    for (id, &(u, v)) in edges.iter().enumerate() {
        assert!(u < n && v < n);
        deg[u] += 1;
        deg[v] += 1;
        adj[u].push((v, id));
        adj[v].push((u, id));
    }
    let odd: Vec<usize> = (0..n).filter(|&v| deg[v] % 2 == 1).collect();
    if !(odd.is_empty() || odd.len() == 2) {
        return None;
    }
    let start = odd.first().copied().unwrap_or_else(|| (0..n).find(|&v| deg[v] > 0).unwrap());
    let mut used = vec![false; edges.len()];
    let mut stack = vec![start];
    let mut path = vec![];
    while let Some(&v) = stack.last() {
        while let Some((to, id)) = adj[v].pop() {
            if !used[id] {
                used[id] = true;
                stack.push(to);
                break;
            }
        }
        if stack.last() == Some(&v) {
            path.push(v);
            stack.pop();
        }
    }
    path.reverse();
    if path.len() == edges.len() + 1 {
        Some(path)
    } else {
        None
    }
}

pub fn eulerian_circuit_undirected(n: usize, edges: &[(usize, usize)]) -> Option<Vec<usize>> {
    let path = eulerian_trail_undirected(n, edges)?;
    if path.first() == path.last() {
        Some(path)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_directed(path: &[usize], edges: &[(usize, usize)]) -> bool {
        let mut rem = edges.to_vec();
        for w in path.windows(2) {
            if let Some(i) = rem.iter().position(|&e| e == (w[0], w[1])) {
                rem.swap_remove(i);
            } else {
                return false;
            }
        }
        rem.is_empty()
    }

    fn check_undirected(path: &[usize], edges: &[(usize, usize)]) -> bool {
        let mut rem: Vec<(usize, usize)> = edges.iter().map(|&(u, v)| (u.min(v), u.max(v))).collect();
        for w in path.windows(2) {
            let e = (w[0].min(w[1]), w[0].max(w[1]));
            if let Some(i) = rem.iter().position(|&x| x == e) {
                rem.swap_remove(i);
            } else {
                return false;
            }
        }
        rem.is_empty()
    }

    #[test]
    fn directed_known() {
        let edges = [(0, 1), (1, 2), (2, 0)];
        let path = eulerian_circuit_directed(3, &edges).unwrap();
        assert!(check_directed(&path, &edges));
        assert!(eulerian_trail_directed(3, &[(0, 1), (0, 2)]).is_none());
    }

    #[test]
    fn undirected_known() {
        let edges = [(0, 1), (1, 2), (2, 0), (0, 3)];
        let path = eulerian_trail_undirected(4, &edges).unwrap();
        assert!(check_undirected(&path, &edges));
        assert!(eulerian_circuit_undirected(4, &edges).is_none());
        assert!(eulerian_trail_undirected(4, &[(0, 1), (0, 2), (0, 3)]).is_none());
    }

    #[test]
    fn disconnected_rejected() {
        assert!(eulerian_trail_directed(4, &[(0, 1), (2, 3)]).is_none());
        assert!(eulerian_trail_undirected(4, &[(0, 1), (2, 3)]).is_none());
    }
}

