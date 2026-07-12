//! 文字インデックスでの部分文字列取得。
//!
//! Rust の文字列スライスは byte 境界を要求するため、Unicode scalar value の
//! 半開区間 `[l, r)` を byte 範囲へ変換してから取り出す。
//!
//! ```
//! use cplib::string::substring::*;
//!
//! assert_eq!(substring("aβcd", 1, 3), "βc");
//! assert_eq!(char_range("aβcd", 1, 3), Some(1..4));
//! assert_eq!(char_range("abc", 2, 5), None);
//! ```

pub fn substring(s: &str, l: usize, r: usize) -> String {
    char_range(s, l, r).map_or_else(String::new, |range| s[range].to_string())
}

pub fn char_range(s: &str, l: usize, r: usize) -> Option<std::ops::Range<usize>> {
    if l > r {
        return None;
    }
    let start = byte_index(s, l)?;
    let end = byte_index(s, r)?;
    Some(start..end)
}

pub fn byte_index(s: &str, char_index: usize) -> Option<usize> {
    if char_index == s.chars().count() {
        return Some(s.len());
    }
    s.char_indices().nth(char_index).map(|(i, _)| i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ascii_and_unicode() {
        assert_eq!(substring("abcdef", 2, 5), "cde");
        assert_eq!(substring("aβcd", 1, 3), "βc");
        assert_eq!(substring("あいうえお", 1, 4), "いうえ");
        assert_eq!(char_range("aβcd", 1, 3), Some(1..4));
    }

    #[test]
    fn invalid_ranges() {
        assert_eq!(substring("abc", 3, 3), "");
        assert_eq!(substring("abc", 2, 5), "");
        assert_eq!(substring("abc", 3, 2), "");
        assert_eq!(char_range("abc", 2, 5), None);
        assert_eq!(char_range("abc", 3, 2), None);
    }

    #[test]
    fn matches_naive_chars_collect() {
        let s = "aβcdえfg山";
        let chars: Vec<char> = s.chars().collect();
        for l in 0..=chars.len() {
            for r in l..=chars.len() {
                let expected: String = chars[l..r].iter().collect();
                assert_eq!(substring(s, l, r), expected);
            }
        }
    }
}
