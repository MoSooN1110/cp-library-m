// source snippet: key=lib_trie2  prefix=lib_trie2

//https://atcoder.jp/contests/abc268/submissions/34834224
struct TrieNode {
    exist: usize,
    accept: Vec<usize>,
    child: [Option<usize>; 32],
}

impl TrieNode {
    fn new() -> Self {
        Self {
            exist: 0,
            accept: Vec::new(),
            child: [None; 32],
        }
    }
}

pub struct Trie {
    arena: Vec<TrieNode>,
    root: usize,
}

impl Trie {
    pub fn new() -> Self {
        let mut arena = Vec::new();
        arena.push(TrieNode::new());
        Self { root: 0, arena }
    }

    fn update_direct(&mut self, node_id: usize, str_id: usize) {
        self.arena[node_id].accept.push(str_id)
    }

    fn update_child(&mut self, node_id: usize) {
        self.arena[node_id].exist += 1;
    }

    /// 文字列 `str` を追加する
    pub fn push(&mut self, str: &str) {
        self._push(
            str.as_bytes().iter(),
            self.root,
            self.arena[self.root].exist,
        )
    }

    fn _push(&mut self, mut iter: core::slice::Iter<u8>, node_id: usize, str_id: usize) {
        if let Some(&c) = iter.next() {
            let c = (c - b'a') as usize;
            let child_id = if let Some(child_id) = self.arena[node_id].child[c] {
                child_id
            } else {
                let child_id = self.arena.len();
                self.arena[node_id].child[c] = Some(child_id);
                self.arena.push(TrieNode::new());
                child_id
            };
            self._push(iter, child_id, str_id);
            self.update_child(node_id);
        } else {
            self.update_direct(node_id, str_id);
        }
    }

    /// 文字列 `str` の prefix に一致する文字列を検索する。
    /// 一致した文字列ごとに関数 f を呼び出す。
    pub fn query<F: FnMut(usize)>(&mut self, str: &str, func: F) {
        self._query(str.as_bytes().iter(), func, self.root);
    }

    fn _query<F: FnMut(usize)>(
        &mut self,
        mut iter: core::slice::Iter<u8>,
        mut func: F,
        node_id: usize,
    ) {
        for &str_id in &self.arena[node_id].accept {
            func(str_id);
        }
        if let Some(&c) = iter.next() {
            let c = (c - b'a') as usize;
            if let Some(child_id) = self.arena[node_id].child[c] {
                self._query(iter, func, child_id);
            }
        }
    }

    /// ノード数を返す
    pub fn size(&self) -> usize {
        self.arena.len()
    }

    /// 存在する文字列の個数を返す
    pub fn count(&self) -> usize {
        self.arena[self.root].exist
    }
}
