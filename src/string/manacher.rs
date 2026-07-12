//! Manacher 法（全中心の回文半径を O(n) で求める）。
//!
//! ```
//! use cplib::string::manacher::*;
//! // "abacaba": 奇数長回文の半径（中心 i の回文は s[i-r+1ではなく i-(r-1)..=i+(r-1)]、長さ 2r-1）
//! let odd = odd_radii(b"abacaba");
//! assert_eq!(odd, vec![1, 2, 1, 4, 1, 2, 1]);
//! let (start, len) = longest_palindrome(b"abacaba");
//! assert_eq!((start, len), (0, 7));
//! ```

/// 奇数長: odd[i] = 中心 i の最大回文の「半径」r（回文長は 2r-1、常に r >= 1）。
pub fn odd_radii<T: Eq>(s: &[T]) -> Vec<usize> {
    let sep = separated_radii(s);
    (0..s.len()).map(|i| (sep[2 * i + 1] + 1) / 2).collect()
}

/// 偶数長: even[i] = s[i-1] と s[i] の間を中心とする最大回文の片側長（回文長は 2*even[i]）。
/// 長さは s.len()+1（両端は 0）。
pub fn even_radii<T: Eq>(s: &[T]) -> Vec<usize> {
    let sep = separated_radii(s);
    (0..=s.len()).map(|i| sep[2 * i] / 2).collect()
}

/// 最長回文部分文字列の (開始位置, 長さ)。同長なら最左。
pub fn longest_palindrome<T: Eq>(s: &[T]) -> (usize, usize) {
    let sep = separated_radii(s);
    let mut best = (0, 0);
    for (i, &r) in sep.iter().enumerate() {
        // セパレータ挿入列での回文半径 r → 元の回文長は r-1（i+r の偶奇で自動的に一致）
        let len = r.saturating_sub(1);
        if len > best.1 {
            best = ((i + 1 - r) / 2, len);
        }
    }
    best
}

/// セパレータ挿入列（長さ 2n+1）上の回文半径。sep[i] = その中心での回文の広がり
/// （挿入列上で [i-r+1, i+r-1] が回文となる最大 r）。
pub fn separated_radii<T: Eq>(s: &[T]) -> Vec<usize> {
    let n = s.len();
    let m = 2 * n + 1;
    // 挿入列 t: t[2i+1] = s[i], t[偶数] = セパレータ。比較を関数化して実体は作らない
    let eq = |a: usize, b: usize| -> bool {
        if a % 2 == 0 {
            b % 2 == 0
        } else {
            b % 2 == 1 && s[a / 2] == s[b / 2]
        }
    };
    let mut rad = vec![0usize; m];
    let (mut i, mut j) = (0usize, 0usize);
    while i < m {
        while i >= j && i + j < m && eq(i - j, i + j) {
            j += 1;
        }
        rad[i] = j;
        let mut k = 1;
        while i >= k && k + rad[i - k] < j {
            rad[i + k] = rad[i - k];
            k += 1;
        }
        i += k;
        j -= k;
    }
    rad
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::misc::xorshift::XorShift;

    fn is_pal(s: &[u8]) -> bool {
        s.iter().eq(s.iter().rev())
    }

    #[test]
    fn test_known() {
        assert_eq!(odd_radii(b"aaa"), vec![1, 2, 1]);
        assert_eq!(even_radii(b"aabb"), vec![0, 1, 0, 1, 0]);
        assert_eq!(longest_palindrome(b"abba"), (0, 4));
        assert_eq!(longest_palindrome(b"ab"), (0, 1));
    }

    #[test]
    fn test_random_vs_bruteforce() {
        let mut rng = XorShift::new(2024);
        for _ in 0..200 {
            let n = 1 + rng.next_range(30) as usize;
            let s: Vec<u8> = (0..n).map(|_| b'a' + rng.next_range(3) as u8).collect();
            let odd = odd_radii(&s);
            for i in 0..n {
                let mut r = 0;
                while i >= r && i + r < n && s[i - r] == s[i + r] {
                    r += 1;
                }
                assert_eq!(odd[i], r);
            }
            let even = even_radii(&s);
            for i in 0..=n {
                let mut r = 0;
                while i >= r + 1 && i + r < n && s[i - r - 1] == s[i + r] {
                    r += 1;
                }
                assert_eq!(even[i], r);
            }
            let (st, len) = longest_palindrome(&s);
            assert!(len >= 1 && is_pal(&s[st..st + len]));
            for l in 0..n {
                for r in l + 1..=n {
                    if is_pal(&s[l..r]) {
                        assert!(r - l <= len);
                    }
                }
            }
        }
    }
}
