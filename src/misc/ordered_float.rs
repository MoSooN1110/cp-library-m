//! 全順序を持つ f64 ラッパー（ソートや BinaryHeap で使える）。
//!
//! ```
//! use cplib::misc::ordered_float::*;
//! let mut v = vec![Of(3.0), Of(1.0), Of(2.0)];
//! v.sort();
//! assert_eq!(v[0], Of(1.0));
//! ```

/// total ordering な f64（NaN は最大側に寄せる `total_cmp` 準拠）。
#[derive(Clone, Copy, Debug)]
pub struct Of(pub f64);

impl PartialEq for Of {
    fn eq(&self, other: &Self) -> bool {
        self.0.total_cmp(&other.0) == std::cmp::Ordering::Equal
    }
}
impl Eq for Of {}
impl PartialOrd for Of {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Of {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.total_cmp(&other.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sort_and_nan() {
        let mut v = vec![Of(2.5), Of(-1.0), Of(f64::NAN), Of(0.0)];
        v.sort();
        assert_eq!(v[0], Of(-1.0));
        assert_eq!(v[1], Of(0.0));
        assert_eq!(v[2], Of(2.5));
        assert!(v[3].0.is_nan());
        assert_eq!(*v.iter().min().unwrap(), Of(-1.0));
    }
}
