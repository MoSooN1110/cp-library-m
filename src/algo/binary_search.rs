//! 汎用二分探索（判定関数の境界を求める）。
//!
//! ```
//! use cplib::algo::binary_search::*;
//! // x*x >= 100 となる最小の x（ok=大きい側）
//! let ans = bisect_int(1_000_000, 0, |x| x * x >= 100);
//! assert_eq!(ans, 10);
//! ```

/// `f(ok)=true, f(ng)=false` を前提に、true になる境界（ok に最も近い値）を返す。
/// ok<ng でも ok>ng でもよい。
pub fn bisect_int<F: Fn(i64) -> bool>(mut ok: i64, mut ng: i64, f: F) -> i64 {
    while (ok - ng).abs() > 1 {
        let mid = ok + (ng - ok) / 2;
        if f(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    ok
}

/// 実数版（`iter` 回反復）。
pub fn bisect_float<F: Fn(f64) -> bool>(mut ok: f64, mut ng: f64, iter: usize, f: F) -> f64 {
    for _ in 0..iter {
        let mid = (ok + ng) / 2.0;
        if f(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    ok
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn int_boundary() {
        // a[i] >= 5 の最小 index
        let a = [1, 3, 5, 7, 9];
        let idx = bisect_int(a.len() as i64, -1, |i| a[i as usize] >= 5);
        assert_eq!(idx, 2);
        // 単調増加のしきい値
        assert_eq!(bisect_int(0, 1_000_000, |x| x <= 42), 42);
    }
    #[test]
    fn float_sqrt() {
        let r = bisect_float(0.0, 100.0, 100, |x| x * x <= 2.0);
        assert!((r - std::f64::consts::SQRT_2).abs() < 1e-9);
    }
}
