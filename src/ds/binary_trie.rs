//! バイナリトライ（XOR 最小/最大・k 番目・カウント）。非負整数を格納。
//!
//! ```
//! use cplib::ds::binary_trie::*;
//! let mut t = BinaryTrie::new();
//! t.insert(3); t.insert(5); t.insert(8);
//! assert_eq!(t.xor_min(6), 6 ^ 5);    // 6 と最も近い（XOR 最小）
//! assert_eq!(t.kth(0), 3);            // 最小
//! assert_eq!(t.len(), 3);
//! ```

const BITS: usize = 30; // 0 <= x < 2^30

pub struct BinaryTrie {
    ch: Vec<[i32; 2]>,
    cnt: Vec<u32>,
}

impl Default for BinaryTrie {
    fn default() -> Self {
        Self::new()
    }
}

impl BinaryTrie {
    pub fn new() -> Self {
        BinaryTrie {
            ch: vec![[-1, -1]],
            cnt: vec![0],
        }
    }

    pub fn len(&self) -> usize {
        self.cnt[0] as usize
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn insert(&mut self, x: u64) {
        let mut v = 0usize;
        self.cnt[v] += 1;
        for i in (0..BITS).rev() {
            let b = ((x >> i) & 1) as usize;
            if self.ch[v][b] < 0 {
                let id = self.ch.len();
                self.ch.push([-1, -1]);
                self.cnt.push(0);
                self.ch[v][b] = id as i32;
            }
            v = self.ch[v][b] as usize;
            self.cnt[v] += 1;
        }
    }

    /// x を 1 つ削除（存在する前提）。
    pub fn erase(&mut self, x: u64) {
        let mut v = 0usize;
        self.cnt[v] -= 1;
        for i in (0..BITS).rev() {
            let b = ((x >> i) & 1) as usize;
            v = self.ch[v][b] as usize;
            self.cnt[v] -= 1;
        }
    }

    pub fn contains(&self, x: u64) -> bool {
        let mut v = 0usize;
        for i in (0..BITS).rev() {
            let b = ((x >> i) & 1) as usize;
            if self.ch[v][b] < 0 || self.cnt[self.ch[v][b] as usize] == 0 {
                return false;
            }
            v = self.ch[v][b] as usize;
        }
        true
    }

    fn alive(&self, node: i32) -> bool {
        node >= 0 && self.cnt[node as usize] > 0
    }

    /// min over y in set of (x ^ y)
    pub fn xor_min(&self, x: u64) -> u64 {
        assert!(!self.is_empty());
        let mut v = 0usize;
        let mut res = 0u64;
        for i in (0..BITS).rev() {
            let b = ((x >> i) & 1) as usize;
            if self.alive(self.ch[v][b]) {
                v = self.ch[v][b] as usize;
            } else {
                res |= 1 << i;
                v = self.ch[v][b ^ 1] as usize;
            }
        }
        res
    }

    /// max over y in set of (x ^ y)
    pub fn xor_max(&self, x: u64) -> u64 {
        assert!(!self.is_empty());
        let mut v = 0usize;
        let mut res = 0u64;
        for i in (0..BITS).rev() {
            let b = ((x >> i) & 1) as usize;
            if self.alive(self.ch[v][b ^ 1]) {
                res |= 1 << i;
                v = self.ch[v][b ^ 1] as usize;
            } else {
                v = self.ch[v][b] as usize;
            }
        }
        res
    }

    /// k 番目（0-indexed）に小さい値
    pub fn kth(&self, mut k: usize) -> u64 {
        assert!(k < self.len());
        let mut v = 0usize;
        let mut res = 0u64;
        for i in (0..BITS).rev() {
            let lo = self.ch[v][0];
            let lc = if lo >= 0 { self.cnt[lo as usize] as usize } else { 0 };
            if k < lc {
                v = lo as usize;
            } else {
                k -= lc;
                res |= 1 << i;
                v = self.ch[v][1] as usize;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic_and_brute() {
        let mut t = BinaryTrie::new();
        let mut set: Vec<u64> = vec![];
        let vals = [7u64, 3, 10, 3, 255, 1, 128];
        for &v in &vals {
            t.insert(v);
            set.push(v);
        }
        set.sort();
        assert_eq!(t.len(), set.len());
        for k in 0..set.len() {
            assert_eq!(t.kth(k), set[k]);
        }
        for q in 0..300u64 {
            let mn = set.iter().map(|&y| q ^ y).min().unwrap();
            let mx = set.iter().map(|&y| q ^ y).max().unwrap();
            assert_eq!(t.xor_min(q), mn);
            assert_eq!(t.xor_max(q), mx);
        }
        // erase
        t.erase(3);
        assert_eq!(t.len(), set.len() - 1);
        assert!(t.contains(3)); // まだ 1 つ残る
        t.erase(3);
        assert!(!t.contains(3));
    }
}
