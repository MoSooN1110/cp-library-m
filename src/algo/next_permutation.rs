//! 辞書順の次の順列を in-place で生成（C++ の next_permutation 相当）。
//!
//! ```
//! use cplib::algo::next_permutation::*;
//! let mut a = vec![1, 2, 3];
//! assert!(next_permutation(&mut a));
//! assert_eq!(a, vec![1, 3, 2]);
//! ```

/// 次の順列があれば `true` を返し `a` を更新。最大順列なら `false`（最小に戻す）。
pub fn next_permutation<T: Ord>(a: &mut [T]) -> bool {
    let n = a.len();
    if n <= 1 {
        return false;
    }
    let mut i = n - 1;
    while i > 0 && a[i - 1] >= a[i] {
        i -= 1;
    }
    if i == 0 {
        a.reverse();
        return false;
    }
    let mut j = n - 1;
    while a[j] <= a[i - 1] {
        j -= 1;
    }
    a.swap(i - 1, j);
    a[i..].reverse();
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn enumerates_all() {
        let mut a = vec![1, 2, 3, 4];
        let mut count = 1;
        while next_permutation(&mut a) {
            count += 1;
        }
        assert_eq!(count, 24); // 4!
        assert_eq!(a, vec![1, 2, 3, 4]); // 一巡して戻る
    }
    #[test]
    fn with_duplicates() {
        let mut a = vec![1, 1, 2];
        let mut seen = std::collections::HashSet::new();
        seen.insert(a.clone());
        while next_permutation(&mut a) {
            seen.insert(a.clone());
        }
        assert_eq!(seen.len(), 3); // 112,121,211
    }
}
