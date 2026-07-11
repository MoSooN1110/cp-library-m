//! Implicit Treap（挿入・削除・分割・結合が O(log n) の可変長列）。
//!
//! 添字を明示的なキーとして持たず、部分木サイズから位置を決める Treap。
//! 優先度には内蔵の xorshift 乱数（`misc` カテゴリの `XorShift`）を使う。
//!
//! ```
//! use cplib::ds::implicit_treap::*;
//! let mut t = ImplicitTreap::new();
//! for (i, ch) in "hello".chars().enumerate() {
//!     t.insert(i, ch);
//! }
//! assert_eq!(t.inorder().into_iter().collect::<String>(), "hello");
//! assert_eq!(t.remove(1), 'e');
//! assert_eq!(t.inorder().into_iter().collect::<String>(), "hllo");
//!
//! let (left, right) = t.split(2);
//! assert_eq!(left.inorder().into_iter().collect::<String>(), "hl");
//! assert_eq!(right.inorder().into_iter().collect::<String>(), "lo");
//! let merged = ImplicitTreap::merge(left, right);
//! assert_eq!(merged.inorder().into_iter().collect::<String>(), "hllo");
//! ```

use crate::misc::xorshift::XorShift;

struct Node<T> {
    value: T,
    priority: u64,
    size: usize,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(value: T, priority: u64) -> Box<Self> {
        Box::new(Node {
            value,
            priority,
            size: 1,
            left: None,
            right: None,
        })
    }
    fn pull(&mut self) {
        self.size = 1 + size(&self.left) + size(&self.right);
    }
}

fn size<T>(node: &Option<Box<Node<T>>>) -> usize {
    node.as_ref().map_or(0, |n| n.size)
}

fn split<T>(
    node: Option<Box<Node<T>>>,
    key: usize,
) -> (Option<Box<Node<T>>>, Option<Box<Node<T>>>) {
    match node {
        None => (None, None),
        Some(mut n) => {
            let left_size = size(&n.left);
            if key <= left_size {
                let (l, r) = split(n.left.take(), key);
                n.left = r;
                n.pull();
                (l, Some(n))
            } else {
                let (l, r) = split(n.right.take(), key - left_size - 1);
                n.right = l;
                n.pull();
                (Some(n), r)
            }
        }
    }
}

fn merge_nodes<T>(left: Option<Box<Node<T>>>, right: Option<Box<Node<T>>>) -> Option<Box<Node<T>>> {
    match (left, right) {
        (None, r) => r,
        (l, None) => l,
        (Some(mut l), Some(mut r)) => {
            if l.priority > r.priority {
                l.right = merge_nodes(l.right.take(), Some(r));
                l.pull();
                Some(l)
            } else {
                r.left = merge_nodes(Some(l), r.left.take());
                r.pull();
                Some(r)
            }
        }
    }
}

fn get_node<T>(node: &Option<Box<Node<T>>>, pos: usize) -> Option<&T> {
    let n = node.as_ref()?;
    let left_size = size(&n.left);
    match pos.cmp(&left_size) {
        std::cmp::Ordering::Less => get_node(&n.left, pos),
        std::cmp::Ordering::Equal => Some(&n.value),
        std::cmp::Ordering::Greater => get_node(&n.right, pos - left_size - 1),
    }
}

fn inorder_node<'a, T>(node: &'a Option<Box<Node<T>>>, out: &mut Vec<&'a T>) {
    if let Some(n) = node {
        inorder_node(&n.left, out);
        out.push(&n.value);
        inorder_node(&n.right, out);
    }
}

/// 挿入・削除・分割・結合が O(log n)（期待値）の可変長列。
pub struct ImplicitTreap<T> {
    root: Option<Box<Node<T>>>,
    rng: XorShift,
}

impl<T> Default for ImplicitTreap<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> ImplicitTreap<T> {
    /// 空の Treap を生成する（優先度の乱数は固定シードで決定的）。
    pub fn new() -> Self {
        Self::with_seed(0x9e3779b97f4a7c15)
    }

    /// 優先度の乱数シードを指定して空の Treap を生成する。
    pub fn with_seed(seed: u64) -> Self {
        Self {
            root: None,
            rng: XorShift::new(seed),
        }
    }

    pub fn len(&self) -> usize {
        size(&self.root)
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    /// pos の位置に value を挿入する（0 <= pos <= len）。
    pub fn insert(&mut self, pos: usize, value: T) {
        assert!(pos <= self.len());
        let priority = self.rng.next_u64();
        let node = Node::new(value, priority);
        let (l, r) = split(self.root.take(), pos);
        self.root = merge_nodes(merge_nodes(l, Some(node)), r);
    }

    /// pos の位置の要素を削除して返す（0 <= pos < len）。
    pub fn remove(&mut self, pos: usize) -> T {
        assert!(pos < self.len());
        let (l, mid_r) = split(self.root.take(), pos);
        let (mid, r) = split(mid_r, 1);
        self.root = merge_nodes(l, r);
        mid.unwrap().value
    }

    /// pos の位置の要素への参照（0 <= pos < len）。
    pub fn get(&self, pos: usize) -> &T {
        assert!(pos < self.len());
        get_node(&self.root, pos).unwrap()
    }

    /// 先頭 `at` 個とそれ以降に分割する。乱数状態はそれぞれ独立に引き継がれる。
    pub fn split(mut self, at: usize) -> (Self, Self) {
        assert!(at <= self.len());
        let seed_r = self.rng.next_u64();
        let (l, r) = split(self.root.take(), at);
        (
            Self {
                root: l,
                rng: self.rng,
            },
            Self {
                root: r,
                rng: XorShift::new(seed_r),
            },
        )
    }

    /// 2 つの Treap を結合する（`left` の全要素が `right` の全要素より前に来る）。
    pub fn merge(left: Self, right: Self) -> Self {
        Self {
            root: merge_nodes(left.root, right.root),
            rng: left.rng,
        }
    }

    /// 先頭から順に要素への参照を並べた Vec。
    pub fn inorder(&self) -> Vec<&T> {
        let mut out = Vec::with_capacity(self.len());
        inorder_node(&self.root, &mut out);
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn characters() {
        let mut t = ImplicitTreap::new();
        for (i, ch) in "hello".chars().enumerate() {
            t.insert(i, ch);
        }
        assert_eq!(t.inorder().into_iter().collect::<String>(), "hello");
        assert_eq!(t.remove(1), 'e');
        assert_eq!(t.inorder().into_iter().collect::<String>(), "hllo");
    }

    #[test]
    fn split_merge() {
        let mut t = ImplicitTreap::new();
        for i in 0..10 {
            t.insert(i, i);
        }
        let (l, r) = t.split(4);
        assert_eq!(l.inorder(), vec![&0, &1, &2, &3]);
        let expected_r: Vec<usize> = (4..10).collect();
        assert_eq!(r.inorder(), expected_r.iter().collect::<Vec<_>>());
        let merged = ImplicitTreap::merge(l, r);
        let expected_all: Vec<usize> = (0..10).collect();
        assert_eq!(merged.inorder(), expected_all.iter().collect::<Vec<_>>());
    }

    #[test]
    fn random_vs_vec() {
        let mut t: ImplicitTreap<i64> = ImplicitTreap::with_seed(2024);
        let mut naive: Vec<i64> = Vec::new();
        let mut x: u64 = 55555;
        let mut rng = || {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            x
        };
        for step in 0..500 {
            let n = naive.len();
            if n == 0 || rng() % 2 == 0 {
                let pos = if n == 0 {
                    0
                } else {
                    (rng() as usize) % (n + 1)
                };
                let v = (rng() % 1000) as i64;
                t.insert(pos, v);
                naive.insert(pos, v);
            } else {
                let pos = (rng() as usize) % n;
                let removed = t.remove(pos);
                let expected = naive.remove(pos);
                assert_eq!(removed, expected, "step {step} remove mismatch");
            }
            assert_eq!(t.len(), naive.len(), "step {step} len mismatch");
            let got: Vec<i64> = t.inorder().into_iter().copied().collect();
            assert_eq!(got, naive, "step {step} content mismatch");
            for (i, &v) in naive.iter().enumerate() {
                assert_eq!(*t.get(i), v);
            }
        }
    }
}
