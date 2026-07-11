//! XOR 基底（GF(2) 上の線形基底）。最大 XOR 部分集合和・表現可能性など。
//!
//! ```
//! use cplib::math::xor_basis::*;
//! let mut b = XorBasis::new();
//! for x in [1u64, 2, 4] { b.insert(x); }
//! assert_eq!(b.rank(), 3);
//! assert_eq!(b.max_xor(), 7);   // 1^2^4
//! assert!(b.can_represent(0));
//! ```

#[derive(Default)]
pub struct XorBasis {
    basis: Vec<u64>, // 各要素は最上位ビットが互いに異なる
}

impl XorBasis {
    pub fn new() -> Self {
        XorBasis { basis: vec![] }
    }

    /// x を基底に追加。ランクが増えたら true。
    pub fn insert(&mut self, mut x: u64) -> bool {
        for &b in &self.basis {
            x = x.min(x ^ b);
        }
        if x != 0 {
            self.basis.push(x);
            // 降順に整列（最上位ビットが大きい順）
            self.basis.sort_unstable_by(|a, b| b.cmp(a));
            true
        } else {
            false
        }
    }

    /// x が基底で表現可能か
    pub fn can_represent(&self, mut x: u64) -> bool {
        for &b in &self.basis {
            x = x.min(x ^ b);
        }
        x == 0
    }

    /// 部分集合 XOR の最大値
    pub fn max_xor(&self) -> u64 {
        let mut res = 0u64;
        for &b in &self.basis {
            res = res.max(res ^ b);
        }
        res
    }

    pub fn rank(&self) -> usize {
        self.basis.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic() {
        let mut b = XorBasis::new();
        assert!(b.insert(4));
        assert!(b.insert(2));
        assert!(!b.insert(6)); // 4^2
        assert_eq!(b.rank(), 2);
        assert!(b.can_represent(6));
        assert!(!b.can_represent(1));
        assert_eq!(b.max_xor(), 6);
    }
    #[test]
    fn brute_max_and_represent() {
        let vals = [3u64, 5, 6, 9, 10];
        let mut b = XorBasis::new();
        for &v in &vals {
            b.insert(v);
        }
        // 全部分集合 XOR を列挙
        let mut reachable = std::collections::HashSet::new();
        for mask in 0..(1u32 << vals.len()) {
            let mut x = 0u64;
            for i in 0..vals.len() {
                if mask >> i & 1 == 1 {
                    x ^= vals[i];
                }
            }
            reachable.insert(x);
        }
        assert_eq!(b.max_xor(), *reachable.iter().max().unwrap());
        for x in 0..64u64 {
            assert_eq!(b.can_represent(x), reachable.contains(&x));
        }
    }
}
