//! 有理数（既約分数）。比較・四則。分母は常に正、既約。
//!
//! ```
//! use cplib::math::ratio::*;
//! let a = Ratio::new(1, 2);
//! let b = Ratio::new(1, 3);
//! assert_eq!(a + b, Ratio::new(5, 6));
//! assert!(a > b);
//! ```
use std::cmp::Ordering;
use std::ops::{Add, Mul, Sub};

#[derive(Clone, Copy, Debug)]
pub struct Ratio {
    pub num: i64,
    pub den: i64, // 常に > 0
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a.abs()
    } else {
        gcd(b, a % b)
    }
}

impl Ratio {
    pub fn new(mut num: i64, mut den: i64) -> Self {
        assert!(den != 0, "denominator must be non-zero");
        if den < 0 {
            num = -num;
            den = -den;
        }
        let g = gcd(num, den).max(1);
        Ratio {
            num: num / g,
            den: den / g,
        }
    }
    pub fn from_int(x: i64) -> Self {
        Ratio { num: x, den: 1 }
    }
}

impl PartialEq for Ratio {
    fn eq(&self, o: &Self) -> bool {
        self.num == o.num && self.den == o.den
    }
}
impl Eq for Ratio {}
impl PartialOrd for Ratio {
    fn partial_cmp(&self, o: &Self) -> Option<Ordering> {
        Some(self.cmp(o))
    }
}
impl Ord for Ratio {
    fn cmp(&self, o: &Self) -> Ordering {
        // num/den vs o.num/o.den, den>0
        (self.num as i128 * o.den as i128).cmp(&(o.num as i128 * self.den as i128))
    }
}
impl Add for Ratio {
    type Output = Ratio;
    fn add(self, o: Ratio) -> Ratio {
        Ratio::new(self.num * o.den + o.num * self.den, self.den * o.den)
    }
}
impl Sub for Ratio {
    type Output = Ratio;
    fn sub(self, o: Ratio) -> Ratio {
        Ratio::new(self.num * o.den - o.num * self.den, self.den * o.den)
    }
}
impl Mul for Ratio {
    type Output = Ratio;
    fn mul(self, o: Ratio) -> Ratio {
        Ratio::new(self.num * o.num, self.den * o.den)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic() {
        assert_eq!(Ratio::new(2, 4), Ratio::new(1, 2));
        assert_eq!(Ratio::new(1, -2), Ratio::new(-1, 2));
        assert_eq!(Ratio::new(1, 2) + Ratio::new(1, 6), Ratio::new(2, 3));
        assert_eq!(Ratio::new(2, 3) - Ratio::new(1, 6), Ratio::new(1, 2));
        assert_eq!(Ratio::new(2, 3) * Ratio::new(3, 4), Ratio::new(1, 2));
        let mut v = vec![Ratio::new(3, 4), Ratio::new(1, 2), Ratio::new(2, 3)];
        v.sort();
        assert_eq!(v, vec![Ratio::new(1, 2), Ratio::new(2, 3), Ratio::new(3, 4)]);
    }
}
