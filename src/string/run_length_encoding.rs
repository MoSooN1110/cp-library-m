//! ランレングス圧縮。
//!
//! ```
//! use cplib::string::run_length_encoding::*;
//! assert_eq!(rle(b"aaabbc"), vec![(b'a', 3), (b'b', 2), (b'c', 1)]);
//! ```

pub fn rle<T: PartialEq + Clone>(s: &[T]) -> Vec<(T, usize)> {
    let mut res: Vec<(T, usize)> = Vec::new();
    for x in s {
        if let Some(last) = res.last_mut() {
            if &last.0 == x {
                last.1 += 1;
                continue;
            }
        }
        res.push((x.clone(), 1));
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic() {
        assert_eq!(rle::<u8>(b""), vec![]);
        assert_eq!(rle(b"aaa"), vec![(b'a', 3)]);
        assert_eq!(rle(b"abcabc"), vec![
            (b'a', 1), (b'b', 1), (b'c', 1), (b'a', 1), (b'b', 1), (b'c', 1)
        ]);
        assert_eq!(rle(&[1, 1, 2, 3, 3, 3]), vec![(1, 2), (2, 1), (3, 3)]);
    }
}
