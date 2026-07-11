// source snippet: key=lib_binarytrie  prefix=lib_binarytrie

// https://atcoder.jp/contests/arc122/submissions/23373340
pub struct BinaryTrie {
    child: Vec<[usize; 2]>,
    accept: Vec<usize>,
    lazy: usize,
}
impl BinaryTrie {
    pub fn new() -> Self {
        Self {
            child: vec![[0; 2]],
            accept: vec![0],
            lazy: 0,
        }
    }
    pub fn with_capacity(capacity: usize) -> Self {
        let mut child = Vec::with_capacity(capacity);
        child.push([0; 2]);
        let mut accept = Vec::with_capacity(capacity);
        accept.push(0);
        Self {
            child,
            accept,
            lazy: 0,
        }
    }
    pub fn insert_at(&mut self, mut node: usize, x: usize) -> Vec<usize> {
        let mut path = Vec::new();
        for i in (0u32..30).rev() {
            let ch = (x >> i) & 1;
            path.push(node);
            if self.child[node][ch] == 0 {
                self.child[node][ch] = self.child.len();
                self.child.push([0; 2]);
                self.accept.push(0);
            }
            node = self.child[node][ch];
        }
        path.push(node);
        self.accept[node] += 1;
        path
    }
    pub fn insert(&mut self, x: usize) -> Vec<usize> {
        self.insert_at(0, x)
    }
    pub fn find_min_at(&self, mut node: usize) -> usize {
        let x = self.lazy;
        let mut res = 0;
        for i in (0u32..30).rev() {
            let mut ch = (x >> i) & 1;
            if self.child[node][ch] == 0 {
                ch ^= 1;
            }
            res = res * 2 + ch;
            node = self.child[node][ch];
        }
        res
    }
    pub fn find_min(&self) -> usize {
        self.find_min_at(0)
    }
    pub fn next_node(&self, node: usize, ch: usize) -> Option<usize> {
        if self.child[node][ch] == 0 {
            None
        } else {
            Some(self.child[node][ch])
        }
    }
    pub fn count(&self, node: usize) -> usize {
        self.accept[node]
    }
    pub fn next_count(&self, node: usize, ch: usize) -> usize {
        if let Some(node) = self.next_node(node, ch) {
            self.count(node)
        } else {
            0
        }
    }
}
//    bt.insert(8);
// bt.lazy = 9;
// println!("{:?}", bt.find_min_at(0));
//lazyに値を入れてからminクエリするとxorminが取れる（出てくる値はxorされていない。）
