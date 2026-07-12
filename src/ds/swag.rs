//! SWAG（Sliding Window Aggregation）。半群の queue 集約を償却 O(1) で扱う。
//!
//! ```
//! use cplib::ds::swag::*;
//!
//! let mut q = Swag::new(0i64, |a, b| a + b);
//! q.push(1);
//! q.push(2);
//! q.push(3);
//! assert_eq!(q.fold(), 6);
//! assert_eq!(q.pop(), Some(1));
//! assert_eq!(q.fold(), 5);
//! ```

pub struct Swag<T: Copy> {
    front: Vec<(T, T)>,
    back: Vec<(T, T)>,
    e: T,
    op: fn(T, T) -> T,
}

impl<T: Copy> Swag<T> {
    pub fn new(e: T, op: fn(T, T) -> T) -> Self {
        Self { front: vec![], back: vec![], e, op }
    }

    pub fn len(&self) -> usize {
        self.front.len() + self.back.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn push(&mut self, x: T) {
        let agg = match self.back.last() {
            Some(&(_, prev)) => (self.op)(prev, x),
            None => x,
        };
        self.back.push((x, agg));
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.front.is_empty() {
            while let Some((x, _)) = self.back.pop() {
                let agg = match self.front.last() {
                    Some(&(_, prev)) => (self.op)(x, prev),
                    None => x,
                };
                self.front.push((x, agg));
            }
        }
        self.front.pop().map(|(x, _)| x)
    }

    pub fn fold(&self) -> T {
        match (self.front.last(), self.back.last()) {
            (None, None) => self.e,
            (Some(&(_, a)), None) => a,
            (None, Some(&(_, b))) => b,
            (Some(&(_, a)), Some(&(_, b))) => (self.op)(a, b),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::misc::xorshift::XorShift;
    use std::collections::VecDeque;

    fn gcd(mut a: i64, mut b: i64) -> i64 {
        while b != 0 {
            let r = a % b;
            a = b;
            b = r;
        }
        a.abs()
    }

    #[test]
    fn sum_known() {
        let mut q = Swag::new(0i64, |a, b| a + b);
        assert_eq!(q.fold(), 0);
        q.push(1);
        q.push(2);
        q.push(3);
        assert_eq!(q.fold(), 6);
        assert_eq!(q.pop(), Some(1));
        assert_eq!(q.fold(), 5);
        assert_eq!(q.pop(), Some(2));
        assert_eq!(q.pop(), Some(3));
        assert_eq!(q.pop(), None);
    }

    #[test]
    fn random_gcd_vs_deque() {
        let mut rng = XorShift::new(12345);
        let mut q = Swag::new(0i64, gcd);
        let mut d = VecDeque::new();
        for _ in 0..1000 {
            if d.is_empty() || rng.next_range(3) != 0 {
                let x = 1 + rng.next_range(1000) as i64;
                q.push(x);
                d.push_back(x);
            } else {
                assert_eq!(q.pop(), d.pop_front());
            }
            let want = d.iter().copied().fold(0, gcd);
            assert_eq!(q.fold(), want);
            assert_eq!(q.len(), d.len());
        }
    }
}

