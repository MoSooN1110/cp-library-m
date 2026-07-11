//! トライ木（バイト列）。挿入・検索・接頭辞カウント。
//!
//! ```
//! use cplib::string::trie::*;
//! let mut t = Trie::new();
//! t.insert(b"apple");
//! t.insert(b"app");
//! assert!(t.contains(b"app"));
//! assert!(!t.contains(b"ap"));
//! assert_eq!(t.count_prefix(b"app"), 2);   // "apple","app"
//! ```
use std::collections::HashMap;

#[derive(Default)]
struct Node {
    next: HashMap<u8, usize>,
    passes: u32, // ここを通る単語数
    ends: u32,   // ここで終わる単語数
}

#[derive(Default)]
pub struct Trie {
    nodes: Vec<Node>,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            nodes: vec![Node::default()],
        }
    }

    pub fn insert(&mut self, word: &[u8]) {
        let mut v = 0usize;
        self.nodes[v].passes += 1;
        for &c in word {
            let nxt = match self.nodes[v].next.get(&c) {
                Some(&x) => x,
                None => {
                    let id = self.nodes.len();
                    self.nodes.push(Node::default());
                    self.nodes[v].next.insert(c, id);
                    id
                }
            };
            v = nxt;
            self.nodes[v].passes += 1;
        }
        self.nodes[v].ends += 1;
    }

    fn walk(&self, word: &[u8]) -> Option<usize> {
        let mut v = 0usize;
        for &c in word {
            v = *self.nodes[v].next.get(&c)?;
        }
        Some(v)
    }

    /// 完全一致する単語が登録されているか
    pub fn contains(&self, word: &[u8]) -> bool {
        self.walk(word).map_or(false, |v| self.nodes[v].ends > 0)
    }

    /// `prefix` を接頭辞に持つ登録単語数
    pub fn count_prefix(&self, prefix: &[u8]) -> u32 {
        self.walk(prefix).map_or(0, |v| self.nodes[v].passes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic() {
        let mut t = Trie::new();
        for w in [&b"cat"[..], b"car", b"card", b"dog", b"cat"] {
            t.insert(w);
        }
        assert!(t.contains(b"cat"));
        assert!(t.contains(b"card"));
        assert!(!t.contains(b"ca"));
        assert!(!t.contains(b"do"));
        assert_eq!(t.count_prefix(b"ca"), 4); // cat,car,card,cat
        assert_eq!(t.count_prefix(b"car"), 2); // car,card
        assert_eq!(t.count_prefix(b"z"), 0);
    }
}
