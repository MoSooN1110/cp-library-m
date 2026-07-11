//! 区間集合（互いに素な整数閉区間を管理）。挿入時に隣接・重複を併合。
//!
//! ```
//! use cplib::ds::segment_set::*;
//! let mut s = SegmentSet::new();
//! s.insert(1, 3);
//! s.insert(5, 7);
//! s.insert(3, 5);          // 1..=3 と 5..=7 が繋がり 1..=7 に
//! assert!(s.contains(4));
//! assert_eq!(s.mex(0), 0);
//! assert_eq!(s.mex(2), 8);
//! ```
use std::collections::BTreeMap;

/// 閉区間 `[l, r]` の集合。key=l, val=r。
pub struct SegmentSet {
    map: BTreeMap<i64, i64>,
}

impl Default for SegmentSet {
    fn default() -> Self {
        Self::new()
    }
}

impl SegmentSet {
    pub fn new() -> Self {
        SegmentSet {
            map: BTreeMap::new(),
        }
    }

    /// x を含む区間があれば `(l, r)` を返す。
    pub fn find(&self, x: i64) -> Option<(i64, i64)> {
        if let Some((&l, &r)) = self.map.range(..=x).next_back() {
            if x <= r {
                return Some((l, r));
            }
        }
        None
    }

    pub fn contains(&self, x: i64) -> bool {
        self.find(x).is_some()
    }

    /// 閉区間 [l, r] を追加（隣接する区間も併合）。
    pub fn insert(&mut self, mut l: i64, mut r: i64) {
        // [l-1, r+1] と重なる既存区間をすべて吸収
        let mut to_remove = vec![];
        // 左側: l-1 以下から始まり l-1 以上で終わる区間
        let mut it: Vec<(i64, i64)> = self
            .map
            .range(..=r + 1)
            .rev()
            .take_while(|(_, &er)| er >= l - 1)
            .map(|(&a, &b)| (a, b))
            .collect();
        it.reverse();
        for (a, b) in it {
            l = l.min(a);
            r = r.max(b);
            to_remove.push(a);
        }
        for a in to_remove {
            self.map.remove(&a);
        }
        self.map.insert(l, r);
    }

    /// x 以上で集合に含まれない最小の整数
    pub fn mex(&self, mut x: i64) -> i64 {
        while let Some((_, r)) = self.find(x) {
            x = r + 1;
        }
        x
    }

    pub fn intervals(&self) -> Vec<(i64, i64)> {
        self.map.iter().map(|(&l, &r)| (l, r)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn merge_and_query() {
        let mut s = SegmentSet::new();
        s.insert(1, 2);
        s.insert(4, 5);
        assert_eq!(s.intervals(), vec![(1, 2), (4, 5)]);
        s.insert(3, 3); // 繋げて 1..=5
        assert_eq!(s.intervals(), vec![(1, 5)]);
        assert!(s.contains(3));
        assert!(!s.contains(0));
        assert!(!s.contains(6));
        assert_eq!(s.mex(0), 0);
        assert_eq!(s.mex(1), 6);
        s.insert(-10, -5);
        assert_eq!(s.find(-7), Some((-10, -5)));
    }
    #[test]
    fn overlap_absorb() {
        let mut s = SegmentSet::new();
        s.insert(1, 10);
        s.insert(3, 5); // 内包
        assert_eq!(s.intervals(), vec![(1, 10)]);
        s.insert(8, 20); // 重なり拡張
        assert_eq!(s.intervals(), vec![(1, 20)]);
    }
}
