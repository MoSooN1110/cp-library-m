//! chmin / chmax（より小さい/大きい値で更新し、更新したら true）。
//!
//! ```
//! use cplib::algo::chmin_chmax::*;
//! let mut best = 100;
//! assert!(chmin(&mut best, 40));
//! assert!(!chmin(&mut best, 60));
//! assert_eq!(best, 40);
//! ```

/// `*a > b` なら `*a = b` にして true。
pub fn chmin<T: PartialOrd>(a: &mut T, b: T) -> bool {
    if *a > b {
        *a = b;
        true
    } else {
        false
    }
}

/// `*a < b` なら `*a = b` にして true。
pub fn chmax<T: PartialOrd>(a: &mut T, b: T) -> bool {
    if *a < b {
        *a = b;
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic() {
        let mut x = 5i64;
        assert!(chmax(&mut x, 8));
        assert_eq!(x, 8);
        assert!(!chmax(&mut x, 3));
        assert!(chmin(&mut x, 2));
        assert_eq!(x, 2);
        // f64 でも動く
        let mut f = 1.0f64;
        assert!(chmax(&mut f, 2.5));
        assert_eq!(f, 2.5);
    }
}
