//! 約数列挙（昇順）。
//!
//! ```
//! use cplib::math::divisors::*;
//! assert_eq!(divisors(12), vec![1, 2, 3, 4, 6, 12]);
//! assert_eq!(divisors(1), vec![1]);
//! ```

pub fn divisors(n: u64) -> Vec<u64> {
    let mut small = vec![];
    let mut large = vec![];
    let mut i = 1u64;
    while i * i <= n {
        if n % i == 0 {
            small.push(i);
            if i != n / i {
                large.push(n / i);
            }
        }
        i += 1;
    }
    large.reverse();
    small.extend(large);
    small
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic() {
        assert_eq!(divisors(1), vec![1]);
        assert_eq!(divisors(28), vec![1, 2, 4, 7, 14, 28]);
        assert_eq!(divisors(36), vec![1, 2, 3, 4, 6, 9, 12, 18, 36]);
        // 素数
        assert_eq!(divisors(97), vec![1, 97]);
        // ナイーブ照合
        for n in 1..=300u64 {
            let brute: Vec<u64> = (1..=n).filter(|d| n % d == 0).collect();
            assert_eq!(divisors(n), brute);
        }
    }
}
