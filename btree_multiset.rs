// source snippet: key=lib_btree_multiset  prefix=lib_btree_multiset

#[allow(clippy::module_inception)]
/// B-Tree Multiset
/// `BTreeMultiset` provides basic functions that `BTreeSet` has.
pub mod btree_multiset {
    use std::{
        borrow::Borrow,
        collections::{
            btree_map::{self},
            BTreeMap,
        },
        fmt::Debug,
        iter::FromIterator,
        mem::swap,
        ops::RangeBounds,
    };
    #[derive(Debug, Clone, Default)]
    pub struct BTreeMultiSet<T: Ord + Clone> {
        map: BTreeMap<T, usize>,
        len: usize,
    }
    impl<T: Ord + Clone> BTreeMultiSet<T> {
        /// Makes, a new, empty `BTreeMultiSet`
        pub fn new() -> BTreeMultiSet<T> {
            BTreeMultiSet {
                map: BTreeMap::new(),
                len: 0,
            }
        }
        /// Adds a value to the set.
        /// If the set did not have this value present, `true` is returned.
        pub fn insert(&mut self, value: T) -> bool {
            let count = self.map.entry(value).or_insert(0);
            *count += 1;
            self.len += 1;
            *count == 1
        }
        /// Clears the set, removing all values.
        pub fn clear(&mut self) {
            self.map.clear();
            self.len = 0;
        }
        /// Returns the number of elements in the set.
        pub fn len(&self) -> usize {
            self.len
        }
        /// Returns the number of unique elements in the set.
        pub fn unique_len(&self) -> usize {
            self.map.len()
        }
        /// Returns `true` if the set contains no elements
        pub fn is_empty(&self) -> bool {
            self.len() == 0
        }
        /// Returns a reference to the value and a number of the value in the set.
        pub fn get(&self, value: &T) -> Option<(&T, usize)> {
            self.map
                .get_key_value(value)
                .map(|(key, count)| (key, *count))
        }
        /// Returns `true` if the set contains a value
        pub fn contains(&self, value: &T) -> bool {
            self.map.contains_key(value)
        }
        /// Returns the number of a value in the set.
        pub fn count(&self, value: &T) -> usize {
            self.map.get(value).map_or(0, |x| *x)
        }
        /// Removes a value from the set. Returns whether the value was present in the set.
        pub fn remove(&mut self, value: &T) -> bool {
            if let Some(count) = self.map.get_mut(value) {
                *count -= 1;
                self.len -= 1;
                if *count == 0 {
                    self.map.remove(value);
                }
                true
            } else {
                false
            }
        }
        /// Removes all values from the set. Returns whether the value was present in the set.
        pub fn remove_all(&mut self, value: &T) -> bool {
            let count = self.count(value);
            if count == 0 {
                return false;
            }
            self.len -= count;
            self.map.remove(value).is_some()
        }
        /// Returns a reference to the first value in the set.
        pub fn first(&self) -> Option<&T> {
            self.map.iter().next().map(|(key, _)| key)
        }
        /// Removes the first value from the set and returns it.
        pub fn pop_first(&mut self) -> Option<T> {
            let v = self.first().cloned();
            if let Some(ref v) = v {
                self.remove(v);
            }
            v
        }
        /// Removes the last value from the set and returns it.
        pub fn pop_last(&mut self) -> Option<T> {
            let v = self.last().cloned();
            if let Some(ref v) = v {
                self.remove(v);
            }
            v
        }
        /// Returns a reference to the last value in the set.
        pub fn last(&self) -> Option<&T> {
            self.map.iter().next_back().map(|(key, _)| key)
        }
        /// Gets an iterator that visits the values in the `BTreeMultiSet` in ascending order.
        pub fn iter(&self) -> Iter<'_, T> {
            Iter {
                iter: self.map.iter(),
                front: (None, 0),
                back: (None, 0),
            }
        }
        /// Constructs a double-ended iterator over a sub-range of elements in the set.
        pub fn range<K: ?Sized, R>(&self, range: R) -> Range<'_, T>
        where
            K: Ord,
            T: Borrow<K> + Ord,
            R: RangeBounds<K>,
        {
            Range {
                iter: self.map.range(range),
                front: (None, 0),
                back: (None, 0),
            }
        }
    }
    impl<T: Ord + Clone> FromIterator<T> for BTreeMultiSet<T> {
        fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
            let mut mset = BTreeMultiSet::new();
            for x in iter {
                mset.insert(x);
            }
            mset
        }
    }
    #[derive(Debug)]
    pub struct Range<'a, T: 'a> {
        iter: btree_map::Range<'a, T, usize>,
        front: (Option<&'a T>, usize),
        back: (Option<&'a T>, usize),
    }
    impl<'a, T> Iterator for Range<'a, T> {
        type Item = &'a T;
        fn next(&mut self) -> Option<Self::Item> {
            if self.front.1 == 0 {
                if let Some((key, count)) = self.iter.next() {
                    self.front = (Some(key), *count);
                } else {
                    swap(&mut self.front, &mut self.back);
                }
            }
            if self.front.1 == 0 {
                None
            } else {
                self.front.1 -= 1;
                self.front.0
            }
        }
    }
    impl<'a, T> DoubleEndedIterator for Range<'a, T> {
        fn next_back(&mut self) -> Option<Self::Item> {
            if self.back.1 == 0 {
                if let Some((key, count)) = self.iter.next_back() {
                    self.back = (Some(key), *count);
                } else {
                    swap(&mut self.front, &mut self.back);
                }
            }
            if self.back.1 == 0 {
                None
            } else {
                self.back.1 -= 1;
                self.back.0
            }
        }
    }
    #[derive(Debug)]
    pub struct Iter<'a, T> {
        iter: btree_map::Iter<'a, T, usize>,
        front: (Option<&'a T>, usize),
        back: (Option<&'a T>, usize),
    }
    impl<'a, T> Iterator for Iter<'a, T> {
        type Item = &'a T;
        fn next(&mut self) -> Option<Self::Item> {
            if self.front.1 == 0 {
                if let Some((key, count)) = self.iter.next() {
                    self.front = (Some(key), *count);
                } else {
                    swap(&mut self.front, &mut self.back);
                }
            }
            if self.front.1 == 0 {
                None
            } else {
                self.front.1 -= 1;
                self.front.0
            }
        }
    }
    impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
        fn next_back(&mut self) -> Option<Self::Item> {
            if self.back.1 == 0 {
                if let Some((key, count)) = self.iter.next_back() {
                    self.back = (Some(key), *count);
                } else {
                    swap(&mut self.front, &mut self.back);
                }
            }
            if self.back.1 == 0 {
                None
            } else {
                self.back.1 -= 1;
                self.back.0
            }
        }
    }
}
