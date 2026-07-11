//! 配列全体の gcd/lcm。
//!
//! ```
//! use cplib::math::vector_lcm::*;
//!
//! assert_eq!(gcd_slice(&[12, 18, 30]), 6);
//! assert_eq!(lcm_slice(&[4, 6, 10]), 60);
//! assert_eq!(lcm_slice(&[0, 5, 10]), 0);
//! assert_eq!(checked_lcm_slice(&[u64::MAX, 2]), None);
//! ```

use crate::math::number::{gcd, lcm};

/// 空 slice の gcd は 0 とする。
pub fn gcd_slice(a: &[u64]) -> u64 {
    a.iter().copied().fold(0, gcd)
}

/// 空 slice の lcm は 1 とする。0 を含むと lcm は 0。
///
/// オーバーフローは通常の整数演算と同じ扱い。
pub fn lcm_slice(a: &[u64]) -> u64 {
    a.iter().copied().fold(1, lcm)
}

pub fn checked_lcm(a: u64, b: u64) -> Option<u64> {
    if a == 0 || b == 0 {
        return Some(0);
    }
    (a / gcd(a, b)).checked_mul(b)
}

pub fn checked_lcm_slice(a: &[u64]) -> Option<u64> {
    let mut res = 1u64;
    for &x in a {
        res = checked_lcm(res, x)?;
    }
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(gcd_slice(&[]), 0);
        assert_eq!(gcd_slice(&[12, 18, 30]), 6);
        assert_eq!(gcd_slice(&[0, 0, 6]), 6);
        assert_eq!(lcm_slice(&[]), 1);
        assert_eq!(lcm_slice(&[4, 6, 10]), 60);
        assert_eq!(lcm_slice(&[0, 5, 10]), 0);
        assert_eq!(checked_lcm_slice(&[4, 6, 10]), Some(60));
        assert_eq!(checked_lcm_slice(&[0, u64::MAX]), Some(0));
    }

    #[test]
    fn checked_overflow() {
        assert_eq!(checked_lcm(u64::MAX, 2), None);
        assert_eq!(checked_lcm_slice(&[u64::MAX, 2]), None);
    }

    #[test]
    fn random_matches_pairwise() {
        let mut seed = 314159265u64;
        let mut rng = || {
            seed ^= seed << 7;
            seed ^= seed >> 9;
            seed
        };
        for _ in 0..500 {
            let n = rng() as usize % 10;
            let a: Vec<u64> = (0..n).map(|_| rng() % 30).collect();
            let mut g = 0u64;
            let mut l = 1u64;
            for &x in &a {
                g = gcd(g, x);
                l = lcm(l, x);
            }
            assert_eq!(gcd_slice(&a), g);
            assert_eq!(lcm_slice(&a), l);
        }
    }
}
