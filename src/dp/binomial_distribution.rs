//! 公平な二項分布の確率表。
//!
//! `fair_binomial_table(n)[i][j]` は、公平なコインを `i` 回投げて
//! 表がちょうど `j` 回出る確率。
//!
//! ```rust
//! use cplib::dp::binomial_distribution::*;
//!
//! let table = fair_binomial_table(4);
//! assert!((table[4][2] - 0.375).abs() < 1e-12);
//! assert!((fair_binomial_prob(10, 3) - 120.0 / 1024.0).abs() < 1e-12);
//! assert!((fair_binomial_cdf(2, 1) - 0.75).abs() < 1e-12);
//! ```

pub fn fair_binomial_table(max_n: usize) -> Vec<Vec<f64>> {
    let mut dp = vec![vec![0.0; max_n + 1]; max_n + 1];
    dp[0][0] = 1.0;
    for i in 0..max_n {
        for j in 0..=i {
            dp[i + 1][j] += dp[i][j] * 0.5;
            dp[i + 1][j + 1] += dp[i][j] * 0.5;
        }
    }
    dp
}

pub fn fair_binomial_prob(n: usize, k: usize) -> f64 {
    if k > n {
        return 0.0;
    }
    fair_binomial_table(n)[n][k]
}

pub fn fair_binomial_cdf(n: usize, k: usize) -> f64 {
    if k >= n {
        return 1.0;
    }
    fair_binomial_table(n)[n][..=k].iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn comb(n: usize, k: usize) -> u128 {
        if k > n {
            return 0;
        }
        let k = k.min(n - k);
        let mut res = 1u128;
        for i in 0..k {
            res = res * (n - i) as u128 / (i + 1) as u128;
        }
        res
    }

    #[test]
    fn known_values() {
        let table = fair_binomial_table(5);
        assert_eq!(table[0][0], 1.0);
        assert!((table[3][0] - 0.125).abs() < 1e-12);
        assert!((table[4][2] - 0.375).abs() < 1e-12);
        assert_eq!(fair_binomial_prob(5, 6), 0.0);
        assert!((fair_binomial_cdf(2, 1) - 0.75).abs() < 1e-12);
        assert_eq!(fair_binomial_cdf(2, 2), 1.0);
    }

    #[test]
    fn rows_sum_to_one() {
        let table = fair_binomial_table(100);
        for (i, row) in table.iter().enumerate() {
            let sum: f64 = row[..=i].iter().sum();
            assert!((sum - 1.0).abs() < 1e-12, "{i} {sum}");
        }
    }

    #[test]
    fn matches_combinations_for_small_n() {
        let table = fair_binomial_table(50);
        for n in 0..=50 {
            let denom = 2f64.powi(n as i32);
            for k in 0..=n {
                let expected = comb(n, k) as f64 / denom;
                assert!((table[n][k] - expected).abs() < 1e-12, "{n} {k}");
            }
        }
    }
}
