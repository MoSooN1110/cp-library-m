//! 可変長ビットセット（u64 ワード）。AND/OR/XOR/シフト・popcount。
//!
//! ```
//! use cplib::ds::bitset::*;
//! let mut a = BitSet::new(10);
//! a.set(1); a.set(3); a.set(5);
//! assert_eq!(a.count_ones(), 3);
//! assert!(a.get(3));
//! a.shl(2);              // 左シフト
//! assert!(a.get(5) && a.get(7));
//! ```

#[derive(Clone, PartialEq, Eq)]
pub struct BitSet {
    n: usize,
    w: Vec<u64>,
}

impl BitSet {
    pub fn new(n: usize) -> Self {
        BitSet {
            n,
            w: vec![0; n.div_ceil(64)],
        }
    }
    pub fn len(&self) -> usize {
        self.n
    }
    pub fn is_empty(&self) -> bool {
        self.n == 0
    }
    #[inline]
    pub fn get(&self, i: usize) -> bool {
        (self.w[i >> 6] >> (i & 63)) & 1 == 1
    }
    #[inline]
    pub fn set(&mut self, i: usize) {
        self.w[i >> 6] |= 1 << (i & 63);
    }
    #[inline]
    pub fn reset(&mut self, i: usize) {
        self.w[i >> 6] &= !(1 << (i & 63));
    }
    #[inline]
    pub fn flip(&mut self, i: usize) {
        self.w[i >> 6] ^= 1 << (i & 63);
    }
    pub fn count_ones(&self) -> usize {
        self.w.iter().map(|x| x.count_ones() as usize).sum()
    }
    fn trim(&mut self) {
        let r = self.n & 63;
        if r != 0 {
            let last = self.w.len() - 1;
            self.w[last] &= (1u64 << r) - 1;
        }
    }
    /// 左シフト（インデックスが増える方向）。長さは不変。
    pub fn shl(&mut self, s: usize) {
        let word = s >> 6;
        let bit = s & 63;
        let m = self.w.len();
        if word >= m {
            self.w.iter_mut().for_each(|x| *x = 0);
            return;
        }
        if bit == 0 {
            for i in (0..m).rev() {
                self.w[i] = if i >= word { self.w[i - word] } else { 0 };
            }
        } else {
            for i in (0..m).rev() {
                let hi = self.w[i - word] << bit;
                let lo = if i > word {
                    self.w[i - word - 1] >> (64 - bit)
                } else {
                    0
                };
                self.w[i] = if i >= word { hi | lo } else { 0 };
            }
        }
        self.trim();
    }
    /// 右シフト。
    pub fn shr(&mut self, s: usize) {
        let word = s >> 6;
        let bit = s & 63;
        let m = self.w.len();
        if word >= m {
            self.w.iter_mut().for_each(|x| *x = 0);
            return;
        }
        if bit == 0 {
            for i in 0..m {
                self.w[i] = if i + word < m { self.w[i + word] } else { 0 };
            }
        } else {
            for i in 0..m {
                let lo = self.w[i + word] >> bit;
                let hi = if i + word + 1 < m {
                    self.w[i + word + 1] << (64 - bit)
                } else {
                    0
                };
                self.w[i] = if i + word < m { lo | hi } else { 0 };
            }
        }
    }
    pub fn and_with(&mut self, o: &BitSet) {
        for i in 0..self.w.len() {
            self.w[i] &= o.w[i];
        }
    }
    pub fn or_with(&mut self, o: &BitSet) {
        for i in 0..self.w.len() {
            self.w[i] |= o.w[i];
        }
    }
    pub fn xor_with(&mut self, o: &BitSet) {
        for i in 0..self.w.len() {
            self.w[i] ^= o.w[i];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ops_vs_brute() {
        let n = 130;
        let mut b = BitSet::new(n);
        let mut naive = vec![false; n];
        let idxs = [0, 1, 63, 64, 65, 100, 129];
        for &i in &idxs {
            b.set(i);
            naive[i] = true;
        }
        assert_eq!(b.count_ones(), idxs.len());
        for i in 0..n {
            assert_eq!(b.get(i), naive[i]);
        }
        // shl
        let s = 10;
        b.shl(s);
        let mut n2 = vec![false; n];
        for i in 0..n {
            if naive[i] && i + s < n {
                n2[i + s] = true;
            }
        }
        for i in 0..n {
            assert_eq!(b.get(i), n2[i], "shl bit {i}");
        }
        // shr back
        b.shr(s);
        for i in 0..n {
            let expect = if i + s < n { n2[i + s] } else { false };
            assert_eq!(b.get(i), expect, "shr bit {i}");
        }
    }
    #[test]
    fn logic() {
        let mut a = BitSet::new(64);
        let mut b = BitSet::new(64);
        a.set(1);
        a.set(2);
        b.set(2);
        b.set(3);
        let mut x = a.clone();
        x.and_with(&b);
        assert_eq!(x.count_ones(), 1);
        assert!(x.get(2));
        let mut o = a.clone();
        o.or_with(&b);
        assert_eq!(o.count_ones(), 3);
    }
}
