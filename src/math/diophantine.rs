//! 一次不定方程式 a*x + b*y = c の整数解（`number::ext_gcd` を再利用）。
//!
//! ```
//! use cplib::math::diophantine::*;
//! let (x, y) = solve(3, 5, 1).unwrap();
//! assert_eq!(3 * x + 5 * y, 1);
//! assert!(solve(2, 4, 3).is_none());   // gcd(2,4)=2 は 3 を割らない
//! ```
use crate::math::number::ext_gcd;

/// a*x + b*y = c の解 (x, y) を 1 つ返す。解なしなら None。
/// (a, b) がともに 0 の場合は c==0 のとき (0,0)。
pub fn solve(a: i64, b: i64, c: i64) -> Option<(i64, i64)> {
    if a == 0 && b == 0 {
        return if c == 0 { Some((0, 0)) } else { None };
    }
    let (g, x0, y0) = ext_gcd(a, b); // a*x0 + b*y0 = g (>0)
    if c % g != 0 {
        return None;
    }
    let k = c / g;
    Some((x0 * k, y0 * k))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solves() {
        for a in -10..=10i64 {
            for b in -10..=10i64 {
                for c in -10..=10i64 {
                    match solve(a, b, c) {
                        Some((x, y)) => assert_eq!(a * x + b * y, c, "a{a} b{b} c{c}"),
                        None => {
                            // 解なしの確認: a=b=0 かつ c!=0、または gcd が c を割らない
                            if a == 0 && b == 0 {
                                assert_ne!(c, 0);
                            } else {
                                let g = {
                                    fn gcd(a: i64, b: i64) -> i64 {
                                        if b == 0 {
                                            a.abs()
                                        } else {
                                            gcd(b, a % b)
                                        }
                                    }
                                    gcd(a, b)
                                };
                                assert_ne!(c % g, 0);
                            }
                        }
                    }
                }
            }
        }
    }
}
