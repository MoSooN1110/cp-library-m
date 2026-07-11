// source snippet: key=Tree  prefix=Tree
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
