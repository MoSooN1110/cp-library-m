//! 最長共通部分列（LCS, DP）。長さと 1 つの復元列を返す。
//!
//! ```
//! use cplib::string::lcs::*;
//! let (len, seq) = lcs(b"abcde", b"ace");
//! assert_eq!(len, 3);
//! assert_eq!(seq, b"ace".to_vec());
//! ```

/// `(長さ, 復元した共通部分列)` を返す。
pub fn lcs<T: PartialEq + Clone>(a: &[T], b: &[T]) -> (usize, Vec<T>) {
    let (n, m) = (a.len(), b.len());
    let mut dp = vec![vec![0u32; m + 1]; n + 1];
    for i in 0..n {
        for j in 0..m {
            dp[i + 1][j + 1] = if a[i] == b[j] {
                dp[i][j] + 1
            } else {
                dp[i][j + 1].max(dp[i + 1][j])
            };
        }
    }
    // 復元
    let mut seq = Vec::new();
    let (mut i, mut j) = (n, m);
    while i > 0 && j > 0 {
        if a[i - 1] == b[j - 1] {
            seq.push(a[i - 1].clone());
            i -= 1;
            j -= 1;
        } else if dp[i - 1][j] >= dp[i][j - 1] {
            i -= 1;
        } else {
            j -= 1;
        }
    }
    seq.reverse();
    (dp[n][m] as usize, seq)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic() {
        assert_eq!(lcs(b"abcde", b"ace").0, 3);
        assert_eq!(lcs(b"", b"abc").0, 0);
        let (len, seq) = lcs(b"axbxxcx", b"abc");
        assert_eq!(len, 3);
        assert_eq!(seq, b"abc".to_vec());
        // 復元が両方の部分列であることを検証
        fn is_subseq(sub: &[u8], s: &[u8]) -> bool {
            let mut it = s.iter();
            sub.iter().all(|c| it.any(|x| x == c))
        }
        let (_, seq) = lcs(b"AGGTAB", b"GXTXAYB");
        assert!(is_subseq(&seq, b"AGGTAB"));
        assert!(is_subseq(&seq, b"GXTXAYB"));
    }
}
