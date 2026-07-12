//! `iwi` 型の消去ルールに対する区間 DP。
//!
//! 文字列の連続部分を、`i` + 消去可能列 + `w` + 消去可能列 + `i`
//! の形なら丸ごと消せるとする。任意の分割で独立に消してよいとき、消せる文字数の最大値を求める。
//!
//! ```rust
//! use cplib::dp::iwi::*;
//!
//! assert_eq!(max_iwi_removable_len("iwi"), 3);
//! assert_eq!(max_iwi_removable_len("iiwwii"), 0);
//! assert_eq!(max_iwi_removable_len("iwiiwi"), 6);
//! ```

pub fn max_iwi_removable_len(s: &str) -> usize {
    let chars: Vec<char> = s.chars().collect();
    iwi_dp_table(&chars)[0][chars.len()]
}

pub fn max_iwi_removable_len_chars(s: &[char]) -> usize {
    iwi_dp_table(s)[0][s.len()]
}

pub fn iwi_dp_table(s: &[char]) -> Vec<Vec<usize>> {
    let n = s.len();
    let mut dp = vec![vec![0usize; n + 1]; n + 1];
    for len in 1..=n {
        for l in 0..=n - len {
            let r = l + len;
            let mut best = 0usize;
            for m in l + 1..r {
                best = best.max(dp[l][m] + dp[m][r]);
            }
            if len >= 3 && s[l] == 'i' && s[r - 1] == 'i' {
                for w in l + 1..r - 1 {
                    if s[w] == 'w' && dp[l + 1][w] == w - l - 1 && dp[w + 1][r - 1] == r - w - 2 {
                        best = best.max(len);
                    }
                }
            }
            dp[l][r] = best;
        }
    }
    dp
}

pub fn is_iwi_fully_removable(s: &str) -> bool {
    max_iwi_removable_len(s) == s.chars().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn brute_max(s: &[char]) -> usize {
        let n = s.len();
        let mut memo = vec![vec![None; n + 1]; n + 1];
        brute_rec(0, n, s, &mut memo)
    }

    fn brute_rec(l: usize, r: usize, s: &[char], memo: &mut [Vec<Option<usize>>]) -> usize {
        if let Some(value) = memo[l][r] {
            return value;
        }
        let mut best = 0;
        for m in l + 1..r {
            best = best.max(brute_rec(l, m, s, memo) + brute_rec(m, r, s, memo));
        }
        if r - l >= 3 && s[l] == 'i' && s[r - 1] == 'i' {
            for w in l + 1..r - 1 {
                if s[w] == 'w'
                    && brute_rec(l + 1, w, s, memo) == w - l - 1
                    && brute_rec(w + 1, r - 1, s, memo) == r - w - 2
                {
                    best = best.max(r - l);
                }
            }
        }
        memo[l][r] = Some(best);
        best
    }

    #[test]
    fn known_cases() {
        assert_eq!(max_iwi_removable_len(""), 0);
        assert_eq!(max_iwi_removable_len("iwi"), 3);
        assert_eq!(max_iwi_removable_len("iiwwii"), 0);
        assert_eq!(max_iwi_removable_len("iwiiwi"), 6);
        assert_eq!(max_iwi_removable_len("iwxiwiiwi"), 6);
        assert!(!is_iwi_fully_removable("iiwwii"));
        assert!(is_iwi_fully_removable("iwiiwi"));
        assert!(!is_iwi_fully_removable("iwx"));
    }

    #[test]
    fn table_values() {
        let s: Vec<char> = "iwiiwi".chars().collect();
        let dp = iwi_dp_table(&s);
        assert_eq!(dp[0][3], 3);
        assert_eq!(dp[3][6], 3);
        assert_eq!(dp[0][6], 6);
    }

    #[test]
    fn exhaustive_small_matches_brute() {
        let alphabet = ['i', 'w', 'x'];
        for n in 0..=9usize {
            let mut digits = vec![0usize; n];
            loop {
                let s: Vec<char> = digits.iter().map(|&d| alphabet[d]).collect();
                assert_eq!(max_iwi_removable_len_chars(&s), brute_max(&s), "{s:?}");

                let mut pos = n;
                while pos > 0 {
                    pos -= 1;
                    digits[pos] += 1;
                    if digits[pos] < alphabet.len() {
                        break;
                    }
                    digits[pos] = 0;
                }
                if pos == 0 && (n == 0 || digits[0] == 0) {
                    break;
                }
            }
        }
    }
}
