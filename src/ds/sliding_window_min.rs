//! 固定長 sliding window の最小値・最大値を単調 deque で O(n) に列挙する。
//!
//! ```
//! use cplib::ds::sliding_window_min::*;
//!
//! let a = [3, 1, 4, 1, 5, 9];
//! assert_eq!(sliding_window_min(&a, 3), vec![1, 1, 1, 1]);
//! assert_eq!(sliding_window_max(&a, 3), vec![4, 4, 5, 9]);
//! ```

use std::collections::VecDeque;

pub fn sliding_window_min<T: Ord + Copy>(a: &[T], k: usize) -> Vec<T> {
    sliding_window_by(a, k, |x, y| x <= y)
}

pub fn sliding_window_max<T: Ord + Copy>(a: &[T], k: usize) -> Vec<T> {
    sliding_window_by(a, k, |x, y| x >= y)
}

fn sliding_window_by<T: Copy>(a: &[T], k: usize, better: fn(T, T) -> bool) -> Vec<T> {
    assert!(k > 0, "window size must be positive");
    if a.len() < k {
        return vec![];
    }
    let mut deq: VecDeque<usize> = VecDeque::new();
    let mut res = Vec::with_capacity(a.len() + 1 - k);
    for i in 0..a.len() {
        while let Some(&j) = deq.back() {
            if better(a[j], a[i]) {
                break;
            }
            deq.pop_back();
        }
        deq.push_back(i);
        if deq.front().copied().unwrap() + k <= i {
            deq.pop_front();
        }
        if i + 1 >= k {
            res.push(a[*deq.front().unwrap()]);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::misc::xorshift::XorShift;

    #[test]
    fn known_values() {
        let a = [3, 1, 4, 1, 5, 9];
        assert_eq!(sliding_window_min(&a, 1), a);
        assert_eq!(sliding_window_min(&a, 3), vec![1, 1, 1, 1]);
        assert_eq!(sliding_window_max(&a, 3), vec![4, 4, 5, 9]);
        assert_eq!(sliding_window_min(&a, 10), Vec::<i32>::new());
    }

    #[test]
    fn random_vs_brute() {
        let mut rng = XorShift::new(20260712);
        for _ in 0..200 {
            let n = 1 + rng.next_range(30) as usize;
            let a: Vec<i64> = (0..n).map(|_| rng.next_range(101) as i64 - 50).collect();
            for k in 1..=n {
                let mins: Vec<i64> = (0..=n - k).map(|i| *a[i..i + k].iter().min().unwrap()).collect();
                let maxs: Vec<i64> = (0..=n - k).map(|i| *a[i..i + k].iter().max().unwrap()).collect();
                assert_eq!(sliding_window_min(&a, k), mins);
                assert_eq!(sliding_window_max(&a, k), maxs);
            }
        }
    }
}

