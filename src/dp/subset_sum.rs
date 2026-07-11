//! 部分和問題（到達可能な和の集合をビットで管理、O(n * sum / 64)）。
//!
//! ```
//! use cplib::dp::subset_sum::*;
//! let r = subset_sums(&[2, 3, 5], 10);
//! assert!(r[0] && r[2] && r[5] && r[10]);   // 0,2,3,5,7,8,10
//! assert!(!r[1] && !r[4]);
//! ```

/// `items` の部分集合で作れる和 s (0..=cap) について reachable[s] を返す。
pub fn subset_sums(items: &[u64], cap: usize) -> Vec<bool> {
    // ビットセットで dp。bit s が立つ = 和 s が作れる
    let words = cap / 64 + 1;
    let mut dp = vec![0u64; words];
    dp[0] = 1; // 和 0
    for &x in items {
        let x = x as usize;
        if x > cap {
            continue;
        }
        // dp |= dp << x
        let word = x >> 6;
        let bit = x & 63;
        if bit == 0 {
            for i in (word..words).rev() {
                dp[i] |= dp[i - word];
            }
        } else {
            for i in (word..words).rev() {
                let mut v = dp[i - word] << bit;
                if i - word >= 1 {
                    v |= dp[i - word - 1] >> (64 - bit);
                }
                dp[i] |= v;
            }
        }
    }
    (0..=cap).map(|s| (dp[s >> 6] >> (s & 63)) & 1 == 1).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    fn brute(items: &[u64], cap: usize) -> Vec<bool> {
        let mut r = vec![false; cap + 1];
        r[0] = true;
        for &x in items {
            for s in (0..=cap).rev() {
                if r[s] && s + x as usize <= cap {
                    r[s + x as usize] = true;
                }
            }
        }
        r
    }
    #[test]
    fn matches_brute() {
        let mut x: u64 = 999;
        let mut rng = || {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            x
        };
        for _ in 0..200 {
            let n = (rng() as usize) % 8;
            let cap = 1 + (rng() as usize) % 200;
            let items: Vec<u64> = (0..n).map(|_| 1 + rng() % 50).collect();
            assert_eq!(subset_sums(&items, cap), brute(&items, cap));
        }
    }
}
