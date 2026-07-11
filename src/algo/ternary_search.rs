//! 三分探索（凸/凹関数の極値）。整数・実数版。
//!
//! ```
//! use cplib::algo::ternary_search::*;
//! // f(x)=(x-3)^2 の最小（整数）
//! let x = ternary_search_min_int(-10, 10, |x| (x - 3) * (x - 3));
//! assert_eq!(x, 3);
//! ```

/// 下に凸な整数関数 f の最小点（[lo, hi] の argmin）を返す。
pub fn ternary_search_min_int<F: Fn(i64) -> i64>(mut lo: i64, mut hi: i64, f: F) -> i64 {
    while hi - lo > 2 {
        let m1 = lo + (hi - lo) / 3;
        let m2 = hi - (hi - lo) / 3;
        if f(m1) < f(m2) {
            hi = m2;
        } else {
            lo = m1;
        }
    }
    let mut best = lo;
    for x in lo..=hi {
        if f(x) < f(best) {
            best = x;
        }
    }
    best
}

/// 下に凸な実数関数 f の最小点（`iter` 回反復）。
pub fn ternary_search_min_float<F: Fn(f64) -> f64>(mut lo: f64, mut hi: f64, iter: usize, f: F) -> f64 {
    for _ in 0..iter {
        let m1 = lo + (hi - lo) / 3.0;
        let m2 = hi - (hi - lo) / 3.0;
        if f(m1) < f(m2) {
            hi = m2;
        } else {
            lo = m1;
        }
    }
    (lo + hi) / 2.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn int_min() {
        for c in -20..=20i64 {
            let x = ternary_search_min_int(-50, 50, |x| (x - c) * (x - c) + 7);
            assert_eq!(x, c);
        }
    }
    #[test]
    fn float_min() {
        let x = ternary_search_min_float(-10.0, 10.0, 200, |x| (x - 2.5) * (x - 2.5));
        assert!((x - 2.5).abs() < 1e-6);
    }
}
