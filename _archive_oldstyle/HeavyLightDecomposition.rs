// source snippet: key=HeavyLightDecomposition  prefix=HeavyLightDecomposition
// (旧実装/CamelCase。lib_ 版の旧バージョンの可能性あり)

pub struct Tree {
    pub root: usize,
    pub parent: Vec<Option<usize>>,
    pub childs: Vec<Vec<usize>>,
}
impl Tree {
    pub fn from_neighbor_list(n: usize, root: usize, g: &[Vec<usize>]) -> Tree {
        let mut parent = vec![None; n];
        let mut childs = vec![Vec::new(); n];
        let mut stack = vec![(root, None)];
        while let Some((i, p)) = stack.pop() {
            parent[i] = p;
            for &to in &g[i] {
                if Some(to) != p {
                    stack.push((to, Some(i)));
                    childs[i].push(to);
                }
            }
        }
        Tree {
            root,
            parent,
            childs,
        }
    }
}
pub struct HeavyLightDecomposition {
    pub ids: Vec<(usize, usize)>,
    pub parents: Vec<Option<(usize, usize)>>,
    pub parts: Vec<Vec<usize>>,
}
impl HeavyLightDecomposition {
    pub fn new(tree: &Tree) -> HeavyLightDecomposition {
        fn size(i: usize, tree: &Tree, memo: &mut [Option<usize>]) -> usize {
            if let Some(res) = memo[i] {
                return res;
            }
            let res = tree.childs[i]
                .iter()
                .map(|&to| size(to, tree, memo))
                .sum::<usize>()
                + 1;
            memo[i] = Some(res);
            res
        }
        let n = tree.parent.len();
        let root = tree.root;
        let mut memo = vec![None; n];
        let mut ids = vec![(0, 0); n];
        let mut parts: Vec<Vec<usize>> = Vec::new();
        let mut stack = vec![(root, false, None)];
        let mut parents = Vec::new();
        while let Some((i, h, pid)) = stack.pop() {
            if h {
                let (k, _) = pid.unwrap();
                ids[i] = (k, parts[k].len());
                parts[k].push(i);
            } else {
                ids[i] = (parts.len(), 0);
                parts.push(vec![i]);
                parents.push(pid);
            }
            let id = ids[i];
            let heavy = tree.childs[i]
                .iter()
                .max_by_key(|&&to| size(to, &tree, &mut memo));
            for &to in &tree.childs[i] {
                if Some(&to) != heavy {
                    stack.push((to, false, Some(id)));
                }
            }
            if let Some(&h) = heavy {
                stack.push((h, true, Some(id)));
            }
        }
        HeavyLightDecomposition {
            ids,
            parents,
            parts,
        }
    }
}
