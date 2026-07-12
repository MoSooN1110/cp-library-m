//! 商 `n / i` が一定になる区間を O(sqrt n) 個にまとめて列挙する。
//!
//! ```
//! use cplib::math::quotient_ranges::*;
//!
//! let v = quotient_ranges(10);
//! assert_eq!(v, vec![(1, 2, 10), (2, 3, 5), (3, 4, 3), (4, 6, 2), (6, 11, 1)]);
//! ```

/// `i in [l, r)` で `n / i == q` となる区間 `(l, r, q)` を昇順に返す。
pub fn quotient_ranges(n: u64) -> Vec<(u64, u64, u64)> {
    let mut res = vec![];
    let mut l = 1u64;
    while l <= n {
        let q = n / l;
        let r = n / q + 1;
        res.push((l, r, q));
        l = r;
    }
    res
}

/// `i in [1, n]` で現れる `n / i` の値を降順に返す。
pub fn quotient_values(n: u64) -> Vec<u64> {
    quotient_ranges(n).into_iter().map(|(_, _, q)| q).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_values() {
        assert_eq!(quotient_ranges(0), Vec::<(u64, u64, u64)>::new());
        assert_eq!(quotient_values(10), vec![10, 5, 3, 2, 1]);
    }

    #[test]
    fn exhaustive_small() {
        for n in 1..=1000 {
            let ranges = quotient_ranges(n);
            let mut covered = vec![false; n as usize + 1];
            for &(l, r, q) in &ranges {
                assert!(l < r && r <= n + 1);
                for i in l..r {
                    assert!(!covered[i as usize]);
                    covered[i as usize] = true;
                    assert_eq!(n / i, q);
                }
            }
            assert!((1..=n).all(|i| covered[i as usize]));
        }
    }
}

