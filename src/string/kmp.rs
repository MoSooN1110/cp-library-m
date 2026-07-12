//! MP/KMP（失敗関数・パターン検索・最小周期）。Z-algorithm とは用途で使い分ける。
//!
//! ```
//! use cplib::string::kmp::*;
//! let occ = kmp_search(b"ababcababab", b"abab");
//! assert_eq!(occ, vec![0, 5, 7]);
//! assert_eq!(smallest_period(b"abcabcab"), 3);
//! ```

/// MP 失敗関数: fail[i] = s[..i] の最長の真の border（接頭辞かつ接尾辞）の長さ。
/// 長さ n+1（fail[0] = 0）。
pub fn failure<T: Eq>(s: &[T]) -> Vec<usize> {
    let n = s.len();
    let mut fail = vec![0usize; n + 1];
    let mut j = 0;
    for i in 1..n {
        while j > 0 && s[i] != s[j] {
            j = fail[j];
        }
        if s[i] == s[j] {
            j += 1;
        }
        fail[i + 1] = j;
    }
    fail
}

/// text 中の pattern の全出現開始位置（重なり可）。pattern が空なら 0..=text.len()。
pub fn kmp_search<T: Eq>(text: &[T], pattern: &[T]) -> Vec<usize> {
    if pattern.is_empty() {
        return (0..=text.len()).collect();
    }
    let fail = failure(pattern);
    let mut occ = vec![];
    let mut j = 0;
    for (i, c) in text.iter().enumerate() {
        while j > 0 && *c != pattern[j] {
            j = fail[j];
        }
        if *c == pattern[j] {
            j += 1;
        }
        if j == pattern.len() {
            occ.push(i + 1 - j);
            j = fail[j];
        }
    }
    occ
}

/// s の最小周期 p（s[i] == s[i+p] が全域で成立する最小の p >= 1）。
/// p が |s| を割り切るとき s は周期文字列。
pub fn smallest_period<T: Eq>(s: &[T]) -> usize {
    let n = s.len();
    if n == 0 {
        return 0;
    }
    n - failure(s)[n]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::misc::xorshift::XorShift;

    #[test]
    fn test_failure_known() {
        assert_eq!(failure(b"aabaaab"), vec![0, 0, 1, 0, 1, 2, 2, 3]);
    }

    #[test]
    fn test_period() {
        assert_eq!(smallest_period(b"abcabc"), 3);
        assert_eq!(smallest_period(b"aaaa"), 1);
        assert_eq!(smallest_period(b"abcd"), 4);
    }

    #[test]
    fn test_search_random_vs_bruteforce() {
        let mut rng = XorShift::new(555);
        for _ in 0..300 {
            let n = rng.next_range(40) as usize;
            let m = 1 + rng.next_range(5) as usize;
            let text: Vec<u8> = (0..n).map(|_| b'a' + rng.next_range(2) as u8).collect();
            let pat: Vec<u8> = (0..m).map(|_| b'a' + rng.next_range(2) as u8).collect();
            let naive: Vec<usize> = (0..n.saturating_sub(m - 1))
                .filter(|&i| text[i..i + m] == pat[..])
                .collect();
            assert_eq!(kmp_search(&text, &pat), naive);
        }
    }
}
