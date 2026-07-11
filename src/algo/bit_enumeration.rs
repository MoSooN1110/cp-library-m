//! ビット全探索・部分集合列挙の補助。
//!
//! ```
//! use cplib::algo::bit_enumeration::*;
//!
//! let a = [3, 5, 8];
//! let sums: Vec<i32> = subset_masks(a.len())
//!     .map(|mask| selected_indices(mask, a.len()).map(|i| a[i]).sum())
//!     .collect();
//! assert_eq!(sums, vec![0, 3, 5, 8, 8, 11, 13, 16]);
//!
//! assert_eq!(submasks(0b101).collect::<Vec<_>>(), vec![0b101, 0b100, 0b001, 0]);
//! ```

/// `n` 要素の部分集合を表す mask を `0..2^n` で列挙する。
pub fn subset_masks(n: usize) -> std::ops::Range<usize> {
    assert!(n < usize::BITS as usize);
    0..(1usize << n)
}

/// `mask` で立っている bit の index を昇順で返す。
pub fn selected_indices(mask: usize, n: usize) -> SelectedIndices {
    SelectedIndices { mask, next: 0, n }
}

pub struct SelectedIndices {
    mask: usize,
    next: usize,
    n: usize,
}

impl Iterator for SelectedIndices {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        while self.next < self.n {
            let i = self.next;
            self.next += 1;
            if (self.mask >> i) & 1 == 1 {
                return Some(i);
            }
        }
        None
    }
}

/// `mask` の部分集合を降順に列挙する。最後に 0 も返す。
pub fn submasks(mask: usize) -> Submasks {
    Submasks {
        mask,
        cur: Some(mask),
    }
}

pub struct Submasks {
    mask: usize,
    cur: Option<usize>,
}

impl Iterator for Submasks {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let cur = self.cur?;
        self.cur = if cur == 0 {
            None
        } else {
            Some((cur - 1) & self.mask)
        };
        Some(cur)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subset_sum_example() {
        let a = [3, 5, 8];
        let sums: Vec<i32> = subset_masks(a.len())
            .map(|mask| selected_indices(mask, a.len()).map(|i| a[i]).sum())
            .collect();
        assert_eq!(sums, vec![0, 3, 5, 8, 8, 11, 13, 16]);
    }

    #[test]
    fn selected_indices_order() {
        assert_eq!(
            selected_indices(0b10110, 5).collect::<Vec<_>>(),
            vec![1, 2, 4]
        );
        assert!(selected_indices(0, 10).next().is_none());
    }

    #[test]
    fn submasks_order_and_count() {
        assert_eq!(
            submasks(0b101).collect::<Vec<_>>(),
            vec![0b101, 0b100, 0b001, 0]
        );
        for mask in 0usize..64 {
            let got = submasks(mask).collect::<Vec<_>>();
            assert_eq!(got.len(), 1usize << mask.count_ones());
            for &sub in &got {
                assert_eq!(sub & !mask, 0);
            }
        }
    }
}
