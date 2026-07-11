//! 接尾辞配列（ダブリング, O(n log^2 n)）と LCP 配列（Kasai, O(n)）。
//!
//! ```
//! use cplib::string::suffix_array::*;
//! let sa = suffix_array(b"banana");
//! // 接尾辞を辞書順に並べた開始位置
//! assert_eq!(sa, vec![5, 3, 1, 0, 4, 2]);
//! let lcp = lcp_array(b"banana", &sa);
//! assert_eq!(lcp.len(), 6);
//! ```

/// 接尾辞配列を返す（`sa[k]` = 辞書順 k 番目の接尾辞の開始位置）。
pub fn suffix_array<T: Ord>(s: &[T]) -> Vec<usize> {
    let n = s.len();
    if n == 0 {
        return vec![];
    }
    // 初期ランク = 文字の順位
    let mut idx: Vec<usize> = (0..n).collect();
    let mut ord: Vec<usize> = (0..n).collect();
    ord.sort_by(|&a, &b| s[a].cmp(&s[b]));
    let mut rank = vec![0usize; n];
    let mut r = 0;
    rank[ord[0]] = 0;
    for i in 1..n {
        if s[ord[i]] != s[ord[i - 1]] {
            r += 1;
        }
        rank[ord[i]] = r;
    }

    let mut tmp = vec![0usize; n];
    let mut k = 1;
    while k < n {
        let key = |i: usize| -> (usize, i64) {
            let second = if i + k < n { rank[i + k] as i64 } else { -1 };
            (rank[i], second)
        };
        idx.sort_by(|&a, &b| key(a).cmp(&key(b)));
        tmp[idx[0]] = 0;
        for i in 1..n {
            tmp[idx[i]] = tmp[idx[i - 1]] + if key(idx[i]) != key(idx[i - 1]) { 1 } else { 0 };
        }
        rank.copy_from_slice(&tmp);
        if rank[idx[n - 1]] == n - 1 {
            break;
        }
        k <<= 1;
    }
    idx
}

/// LCP 配列（Kasai）。`lcp[k]` = sa[k] と sa[k-1] の接尾辞の最長共通接頭辞長（lcp[0]=0）。
pub fn lcp_array<T: PartialEq>(s: &[T], sa: &[usize]) -> Vec<usize> {
    let n = s.len();
    let mut rank = vec![0usize; n];
    for i in 0..n {
        rank[sa[i]] = i;
    }
    let mut lcp = vec![0usize; n];
    let mut h = 0usize;
    for i in 0..n {
        if rank[i] == 0 {
            h = 0;
            continue;
        }
        let j = sa[rank[i] - 1];
        while i + h < n && j + h < n && s[i + h] == s[j + h] {
            h += 1;
        }
        lcp[rank[i]] = h;
        if h > 0 {
            h -= 1;
        }
    }
    lcp
}

#[cfg(test)]
mod tests {
    use super::*;
    fn brute(s: &[u8]) -> Vec<usize> {
        let n = s.len();
        let mut idx: Vec<usize> = (0..n).collect();
        idx.sort_by(|&a, &b| s[a..].cmp(&s[b..]));
        idx
    }
    #[test]
    fn matches_brute() {
        let cases: &[&[u8]] = &[b"banana", b"mississippi", b"aaaa", b"abcabcabc", b"z"];
        for &c in cases {
            assert_eq!(suffix_array(c), brute(c));
        }
        // ランダム
        let mut x: u64 = 88172645463325252;
        let mut rng = || {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            x
        };
        for _ in 0..200 {
            let n = (rng() as usize) % 25;
            let s: Vec<u8> = (0..n).map(|_| b'a' + (rng() % 3) as u8).collect();
            assert_eq!(suffix_array(&s), brute(&s));
        }
    }
    #[test]
    fn lcp_property() {
        let s = b"mississippi";
        let sa = suffix_array(s);
        let lcp = lcp_array(s, &sa);
        for k in 1..s.len() {
            let a = &s[sa[k]..];
            let b = &s[sa[k - 1]..];
            let common = a.iter().zip(b.iter()).take_while(|(x, y)| x == y).count();
            assert_eq!(lcp[k], common);
        }
    }
}
