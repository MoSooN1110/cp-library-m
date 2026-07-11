//! ローリングハッシュ（mod 2^61-1、衝突しにくい単一ハッシュ）。
//!
//! ```
//! use cplib::string::rolling_hash::*;
//! let rh = RollingHash::new(b"abcabc");
//! assert_eq!(rh.hash(0..3), rh.hash(3..6));   // "abc" == "abc"
//! assert_ne!(rh.hash(0..2), rh.hash(1..3));
//! ```

const MOD: u64 = (1 << 61) - 1;
const BASE: u64 = 1_000_000_007; // 固定基数（十分ランダム）

#[inline]
fn mul(a: u64, b: u64) -> u64 {
    let t = (a as u128) * (b as u128);
    let t = ((t >> 61) + (t & MOD as u128)) as u64;
    if t >= MOD {
        t - MOD
    } else {
        t
    }
}

pub struct RollingHash {
    /// h[i] = s[0..i] のハッシュ
    h: Vec<u64>,
    /// pow[i] = BASE^i
    pow: Vec<u64>,
}

impl RollingHash {
    pub fn new(s: &[u8]) -> Self {
        let n = s.len();
        let mut h = vec![0u64; n + 1];
        let mut pow = vec![1u64; n + 1];
        for i in 0..n {
            h[i + 1] = (mul(h[i], BASE) + s[i] as u64) % MOD;
            pow[i + 1] = mul(pow[i], BASE);
        }
        RollingHash { h, pow }
    }

    /// 半開区間 [l, r) のハッシュ
    pub fn hash(&self, range: std::ops::Range<usize>) -> u64 {
        let (l, r) = (range.start, range.end);
        (self.h[r] + MOD - mul(self.h[l], self.pow[r - l])) % MOD
    }

    /// 2 区間が同一内容か
    pub fn eq(&self, a: std::ops::Range<usize>, b: std::ops::Range<usize>) -> bool {
        a.len() == b.len() && self.hash(a) == self.hash(b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic() {
        let s = b"mississippi";
        let rh = RollingHash::new(s);
        // "issi" が 1.. と 4.. に出現
        assert!(rh.eq(1..5, 4..8));
        assert!(!rh.eq(0..3, 3..6));
        // 全部分文字列をナイーブ比較
        let n = s.len();
        for i in 0..n {
            for j in i..=n {
                for k in 0..n {
                    for l in k..=n {
                        let same = s[i..j] == s[k..l];
                        assert_eq!(rh.eq(i..j, k..l), same);
                    }
                }
            }
        }
    }
}
