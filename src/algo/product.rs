//! 直積列挙 iterator。
//!
//! 辞書順で `Vec<usize>` を返す。`product_k_n(k, n)` は
//! `{0, 1, ..., k - 1}^n`、`cartesian_product(&radices)` は
//! `0..radices[0]`, `0..radices[1]`, ... の直積を列挙する。
//!
//! ```
//! use cplib::algo::product::*;
//!
//! assert_eq!(
//!     product_k_n(2, 3).collect::<Vec<_>>(),
//!     vec![
//!         vec![0, 0, 0],
//!         vec![0, 0, 1],
//!         vec![0, 1, 0],
//!         vec![0, 1, 1],
//!         vec![1, 0, 0],
//!         vec![1, 0, 1],
//!         vec![1, 1, 0],
//!         vec![1, 1, 1],
//!     ],
//! );
//! ```

pub fn product_k_n(k: usize, n: usize) -> CartesianProduct {
    CartesianProduct::new(vec![k; n])
}

pub fn cartesian_product(radices: &[usize]) -> CartesianProduct {
    CartesianProduct::new(radices.to_vec())
}

#[derive(Clone, Debug)]
pub struct CartesianProduct {
    radices: Vec<usize>,
    cur: Vec<usize>,
    first: bool,
    finished: bool,
}

impl CartesianProduct {
    pub fn new(radices: Vec<usize>) -> Self {
        let finished = !radices.is_empty() && radices.iter().any(|&r| r == 0);
        let cur = vec![0; radices.len()];
        Self {
            radices,
            cur,
            first: true,
            finished,
        }
    }
}

impl Iterator for CartesianProduct {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }
        if self.first {
            self.first = false;
            return Some(self.cur.clone());
        }

        for i in (0..self.radices.len()).rev() {
            self.cur[i] += 1;
            if self.cur[i] < self.radices[i] {
                return Some(self.cur.clone());
            }
            self.cur[i] = 0;
        }
        self.finished = true;
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn product_k_n_order() {
        assert_eq!(
            product_k_n(3, 2).collect::<Vec<_>>(),
            vec![
                vec![0, 0],
                vec![0, 1],
                vec![0, 2],
                vec![1, 0],
                vec![1, 1],
                vec![1, 2],
                vec![2, 0],
                vec![2, 1],
                vec![2, 2],
            ]
        );
    }

    #[test]
    fn mixed_radices() {
        assert_eq!(
            cartesian_product(&[2, 1, 3]).collect::<Vec<_>>(),
            vec![
                vec![0, 0, 0],
                vec![0, 0, 1],
                vec![0, 0, 2],
                vec![1, 0, 0],
                vec![1, 0, 1],
                vec![1, 0, 2],
            ]
        );
    }

    #[test]
    fn empty_and_zero_radices() {
        assert_eq!(product_k_n(10, 0).collect::<Vec<_>>(), vec![vec![]]);
        assert_eq!(cartesian_product(&[]).collect::<Vec<_>>(), vec![vec![]]);
        assert!(product_k_n(0, 3).next().is_none());
        assert!(cartesian_product(&[2, 0, 3]).next().is_none());
    }

    #[test]
    fn count_matches_product() {
        for k in 1usize..=5 {
            for n in 0usize..=6 {
                let expected = (0..n).fold(1usize, |acc, _| acc * k);
                assert_eq!(product_k_n(k, n).count(), expected);
            }
        }
        let cases = [vec![1], vec![2, 3], vec![4, 1, 2], vec![2, 2, 2, 2]];
        for radices in cases {
            let expected = radices.iter().product::<usize>();
            assert_eq!(cartesian_product(&radices).count(), expected);
        }
    }
}
