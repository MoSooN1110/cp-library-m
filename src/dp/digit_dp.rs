//! 桁 DP（上限以下の整数の桁和分布）。
//!
//! ```
//! use cplib::dp::digit_dp::*;
//!
//! let cnt = digit_sum_counts_leq(20, 10);
//! assert_eq!(cnt[0], 1); // 0
//! assert_eq!(cnt[1], 2); // 1, 10
//! assert_eq!(count_by_digit_sum_leq(20, 10, 2), 3); // 2, 11, 20
//! assert_eq!(digit_sum_sum_leq(13, 10), 55);
//! ```

/// `n` を `base` 進数の上位桁からの配列にする。
pub fn digits(mut n: u64, base: u32) -> Vec<u32> {
    assert!(base >= 2);
    if n == 0 {
        return vec![0];
    }
    let b = base as u64;
    let mut res = Vec::new();
    while n > 0 {
        res.push((n % b) as u32);
        n /= b;
    }
    res.reverse();
    res
}

/// `0..=n` の各整数について、桁和ごとの個数を返す。
///
/// `res[s]` は桁和が `s` である整数の個数。
pub fn digit_sum_counts_leq(n: u64, base: u32) -> Vec<u128> {
    assert!(base >= 2);
    let ds = digits(n, base);
    let max_sum = (base as usize - 1) * ds.len();
    let mut tight = vec![0u128; max_sum + 1];
    let mut less = vec![0u128; max_sum + 1];
    tight[0] = 1;

    for (i, &lim) in ds.iter().enumerate() {
        let mut next_tight = vec![0u128; max_sum + 1];
        let mut next_less = vec![0u128; max_sum + 1];
        let upto_sum = (base as usize - 1) * i;

        for sum in 0..=upto_sum {
            let tv = tight[sum];
            if tv > 0 {
                for d in 0..=lim {
                    let ns = sum + d as usize;
                    if d == lim {
                        next_tight[ns] += tv;
                    } else {
                        next_less[ns] += tv;
                    }
                }
            }
            let lv = less[sum];
            if lv > 0 {
                for d in 0..base {
                    next_less[sum + d as usize] += lv;
                }
            }
        }
        tight = next_tight;
        less = next_less;
    }

    for (a, b) in less.iter_mut().zip(tight) {
        *a += b;
    }
    while less.len() > 1 && less.last() == Some(&0) {
        less.pop();
    }
    less
}

pub fn count_by_digit_sum_leq(n: u64, base: u32, digit_sum: usize) -> u128 {
    digit_sum_counts_leq(n, base)
        .get(digit_sum)
        .copied()
        .unwrap_or(0)
}

pub fn count_digit_sum_at_most_leq(n: u64, base: u32, max_digit_sum: usize) -> u128 {
    digit_sum_counts_leq(n, base)
        .into_iter()
        .take(max_digit_sum + 1)
        .sum()
}

/// `sum_{x=0}^{n} digit_sum(x, base)`。
pub fn digit_sum_sum_leq(n: u64, base: u32) -> u128 {
    digit_sum_counts_leq(n, base)
        .into_iter()
        .enumerate()
        .map(|(s, c)| s as u128 * c)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn brute_counts(n: u64, base: u32) -> Vec<u128> {
        let mut cnt = Vec::<u128>::new();
        for x in 0..=n {
            let s: usize = digits(x, base).into_iter().map(|d| d as usize).sum();
            if cnt.len() <= s {
                cnt.resize(s + 1, 0);
            }
            cnt[s] += 1;
        }
        cnt
    }

    #[test]
    fn digits_basic() {
        assert_eq!(digits(0, 10), vec![0]);
        assert_eq!(digits(12345, 10), vec![1, 2, 3, 4, 5]);
        assert_eq!(digits(0b10110, 2), vec![1, 0, 1, 1, 0]);
        assert_eq!(digits(255, 16), vec![15, 15]);
    }

    #[test]
    fn basic_counts() {
        assert_eq!(digit_sum_counts_leq(0, 10), vec![1]);
        assert_eq!(digit_sum_counts_leq(9, 10), vec![1; 10]);
        let cnt = digit_sum_counts_leq(20, 10);
        assert_eq!(cnt[0], 1);
        assert_eq!(cnt[1], 2);
        assert_eq!(cnt[2], 3);
        assert_eq!(count_by_digit_sum_leq(20, 10, 2), 3);
        assert_eq!(count_digit_sum_at_most_leq(20, 10, 1), 3);
    }

    #[test]
    fn sum_examples() {
        assert_eq!(digit_sum_sum_leq(9, 10), 45);
        assert_eq!(digit_sum_sum_leq(13, 10), 55);
        assert_eq!(digit_sum_sum_leq(0, 10), 0);
    }

    #[test]
    fn random_matches_brute() {
        let mut seed = 20240721u64;
        let mut rng = || {
            seed ^= seed << 7;
            seed ^= seed >> 9;
            seed
        };
        for _ in 0..300 {
            let base = 2 + (rng() % 15) as u32;
            let n = rng() % 5000;
            let got = digit_sum_counts_leq(n, base);
            let expected = brute_counts(n, base);
            assert_eq!(got, expected, "n={n} base={base}");
            let sum: u128 = expected
                .iter()
                .enumerate()
                .map(|(s, &c)| s as u128 * c)
                .sum();
            assert_eq!(digit_sum_sum_leq(n, base), sum);
        }
    }
}
