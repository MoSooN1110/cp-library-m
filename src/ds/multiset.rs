//! 多重集合（BTreeMap ベース）。挿入・削除・個数・最小最大・順序境界。
//!
//! ```
//! use cplib::ds::multiset::*;
//! let mut s = MultiSet::new();
//! s.insert(3); s.insert(1); s.insert(3);
//! assert_eq!(s.len(), 3);
//! assert_eq!(s.count(&3), 2);
//! assert_eq!(s.min().copied(), Some(1));
//! assert!(s.remove(&3));
//! assert_eq!(s.count(&3), 1);
//! ```
use std::collections::BTreeMap;
use std::ops::Bound::{Excluded, Unbounded};

pub struct MultiSet<T> {
    map: BTreeMap<T, usize>,
    len: usize,
}

impl<T: Ord> Default for MultiSet<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord> MultiSet<T> {
    pub fn new() -> Self {
        MultiSet {
            map: BTreeMap::new(),
            len: 0,
        }
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    pub fn insert(&mut self, x: T) {
        *self.map.entry(x).or_insert(0) += 1;
        self.len += 1;
    }
    /// 要素を 1 つ削除。存在すれば true。
    pub fn remove(&mut self, x: &T) -> bool {
        if let Some(c) = self.map.get_mut(x) {
            *c -= 1;
            if *c == 0 {
                self.map.remove(x);
            }
            self.len -= 1;
            true
        } else {
            false
        }
    }
    pub fn count(&self, x: &T) -> usize {
        self.map.get(x).copied().unwrap_or(0)
    }
    pub fn contains(&self, x: &T) -> bool {
        self.map.contains_key(x)
    }
    pub fn min(&self) -> Option<&T> {
        self.map.keys().next()
    }
    pub fn max(&self) -> Option<&T> {
        self.map.keys().next_back()
    }
    /// x 以上の最小の要素
    pub fn lower_bound(&self, x: &T) -> Option<&T> {
        self.map.range(x..).next().map(|(k, _)| k)
    }
    /// x より大きい最小の要素
    pub fn upper_bound(&self, x: &T) -> Option<&T> {
        self.map.range((Excluded(x), Unbounded)).next().map(|(k, _)| k)
    }
    /// 種類数（distinct）
    pub fn distinct(&self) -> usize {
        self.map.len()
    }
    /// (値, 個数) を昇順に走査
    pub fn iter(&self) -> impl Iterator<Item = (&T, usize)> {
        self.map.iter().map(|(k, &c)| (k, c))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic() {
        let mut s = MultiSet::new();
        for x in [5, 1, 3, 3, 1, 1] {
            s.insert(x);
        }
        assert_eq!(s.len(), 6);
        assert_eq!(s.distinct(), 3);
        assert_eq!(s.count(&1), 3);
        assert_eq!(s.min().copied(), Some(1));
        assert_eq!(s.max().copied(), Some(5));
        assert_eq!(s.lower_bound(&2).copied(), Some(3));
        assert_eq!(s.upper_bound(&3).copied(), Some(5));
        assert_eq!(s.lower_bound(&6), None);
        assert!(s.remove(&3));
        assert_eq!(s.count(&3), 1);
        assert!(!s.remove(&99));
        assert_eq!(s.len(), 5);
    }
}
