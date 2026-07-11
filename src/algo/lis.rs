//! 最長増加部分列（LIS）の長さ。狭義/広義を選べる。
//!
//! ```
//! use cplib::algo::lis::*;
//! assert_eq!(lis(&[3, 1, 4, 1, 5, 9, 2, 6], true), 4);   // 1,4,5,9 など
//! assert_eq!(lis(&[2, 2, 2], false), 3);                  // 広義
//! ```

/// `strict=true` で狭義増加、`false` で広義増加（非減少）の LIS 長。
pub fn lis<T: Ord + Copy>(a: &[T], strict: bool) -> usize {
    let mut tail: Vec<T> = Vec::new();
    for &x in a {
        // strict: x が入る最左（lower_bound）／ non-strict: upper_bound
        let pos = if strict {
            tail.partition_point(|&t| t < x)
        } else {
            tail.partition_point(|&t| t <= x)
        };
        if pos == tail.len() {
            tail.push(x);
        } else {
            tail[pos] = x;
        }
    }
    tail.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    fn brute_strict(a: &[i64]) -> usize {
        let n = a.len();
        let mut dp = vec![1usize; n];
        let mut best = 0;
        for i in 0..n {
            for j in 0..i {
                if a[j] < a[i] {
                    dp[i] = dp[i].max(dp[j] + 1);
                }
            }
            best = best.max(dp[i]);
        }
        best
    }
    #[test]
    fn matches_brute() {
        let mut x: u64 = 42;
        let mut rng = || {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            x
        };
        for _ in 0..300 {
            let n = (rng() as usize) % 20;
            let a: Vec<i64> = (0..n).map(|_| (rng() % 7) as i64).collect();
            assert_eq!(lis(&a, true), brute_strict(&a));
        }
        assert_eq!(lis::<i64>(&[], true), 0);
    }
}
