// source snippet: key=lib_trie  prefix=lib_trie

//https://atcoder.jp/contests/abc268/submissions/34764496
#[derive(Clone)]
struct Trie {
    occ: i64,
    me: i64,
    ch: Vec<Option<Box<Trie>>>,
}

impl Trie {
    fn new() -> Self {
        Trie {
            occ: 0,
            me: 0,
            ch: vec![None; 26],
        }
    }
    fn add(&mut self, s: &[char]) {
        if s.is_empty() {
            self.occ += 1;
            self.me += 1;
            return;
        }
        self.occ += 1;
        let idx = s[0] as usize - 97;
        if self.ch[idx].is_none() {
            self.ch[idx] = Some(Box::new(Trie::new()));
        }
        self.ch[idx].as_mut().unwrap().add(&s[1..]);
    }
    fn sup(&self, s: &[char]) -> i64 {
        if s.is_empty() {
            return self.occ;
        }
        let idx = s[0] as usize - 97;
        if let Some(ref ch) = self.ch[idx] {
            ch.sup(&s[1..])
        } else {
            0
        }
    }
    fn sub(&self, s: &[char]) -> i64 {
        if s.is_empty() {
            return self.me;
        }
        let idx = s[0] as usize - 97;
        (if let Some(ref ch) = self.ch[idx] {
            ch.sub(&s[1..])
        } else {
            0
        }) + self.me
    }
}
