//! Formal power series over `Mint` modulo 998244353.
//!
//! Coefficients are stored in ascending degree order: `a[i]` is the
//! coefficient of `x^i`.
//!
//! ```
//! use cplib::math::fps::*;
//! use cplib::math::modint::Mint;
//!
//! let f = vec![Mint::new(1), Mint::new(-1)]; // 1 - x
//! let inv = inv_series(&f, 5);
//! assert_eq!(inv.iter().map(|x| x.val()).collect::<Vec<_>>(), vec![1, 1, 1, 1, 1]);
//!
//! let exp_x = exp_series(&[Mint::new(0), Mint::new(1)], 4);
//! assert_eq!(exp_x[0], Mint::new(1));
//! assert_eq!(exp_x[1], Mint::new(1));
//! assert_eq!(exp_x[2], Mint::new(2).inv());
//! ```

use crate::math::convolution::convolution;
use crate::math::modint::Mint;

pub type Fps = Vec<Mint>;

#[inline]
fn zero() -> Mint {
    Mint::raw(0)
}

#[inline]
fn one() -> Mint {
    Mint::raw(1)
}

/// Removes trailing zero coefficients.
pub fn shrink(a: &mut Fps) {
    while a.last() == Some(&zero()) {
        a.pop();
    }
}

/// Returns the first `n` coefficients, padding with zeros if necessary.
pub fn prefix(a: &[Mint], n: usize) -> Fps {
    let mut res = a[..a.len().min(n)].to_vec();
    res.resize(n, zero());
    res
}

/// Returns the polynomial with coefficients reversed.
pub fn reverse(a: &[Mint]) -> Fps {
    let mut res = a.to_vec();
    res.reverse();
    res
}

/// Multiplies by `x^k`.
pub fn shift_left(a: &[Mint], k: usize) -> Fps {
    if a.is_empty() {
        return vec![];
    }
    let mut res = vec![zero(); k];
    res.extend_from_slice(a);
    res
}

/// Divides by `x^k`, discarding the lower coefficients.
pub fn shift_right(a: &[Mint], k: usize) -> Fps {
    if a.len() <= k {
        vec![]
    } else {
        a[k..].to_vec()
    }
}

/// Coefficient-wise addition.
pub fn add(a: &[Mint], b: &[Mint]) -> Fps {
    let mut res = a.to_vec();
    if b.len() > res.len() {
        res.resize(b.len(), zero());
    }
    for (i, &x) in b.iter().enumerate() {
        res[i] += x;
    }
    res
}

/// Coefficient-wise subtraction.
pub fn sub(a: &[Mint], b: &[Mint]) -> Fps {
    let mut res = a.to_vec();
    if b.len() > res.len() {
        res.resize(b.len(), zero());
    }
    for (i, &x) in b.iter().enumerate() {
        res[i] -= x;
    }
    res
}

/// Scalar multiplication.
pub fn mul_scalar(a: &[Mint], c: Mint) -> Fps {
    let mut res = a.to_vec();
    for x in &mut res {
        *x *= c;
    }
    res
}

/// Polynomial multiplication.
pub fn mul(a: &[Mint], b: &[Mint]) -> Fps {
    convolution(a, b)
}

/// Formal derivative.
pub fn derivative(a: &[Mint]) -> Fps {
    if a.len() <= 1 {
        return vec![];
    }
    let mut res = vec![zero(); a.len() - 1];
    for i in 1..a.len() {
        res[i - 1] = a[i] * Mint::from(i);
    }
    res
}

/// Formal integral with constant term 0.
pub fn integral(a: &[Mint]) -> Fps {
    let mut res = vec![zero(); a.len() + 1];
    for (i, &x) in a.iter().enumerate() {
        res[i + 1] = x / Mint::from(i + 1);
    }
    res
}

/// Evaluates the polynomial at `x`.
pub fn eval(a: &[Mint], x: Mint) -> Mint {
    let mut res = zero();
    for &c in a.iter().rev() {
        res *= x;
        res += c;
    }
    res
}

/// Multiplicative inverse modulo `x^deg`.
///
/// Requires `deg >= 1` and a non-zero constant term.
pub fn inv_series(a: &[Mint], deg: usize) -> Fps {
    assert!(deg >= 1, "degree must be positive");
    assert!(
        !a.is_empty() && a[0] != zero(),
        "constant term must be non-zero"
    );

    let mut g = vec![a[0].inv()];
    while g.len() < deg {
        let m = (g.len() * 2).min(deg);
        let f = prefix(a, m);
        let fg = prefix(&convolution(&f, &g), m);
        let mut correction = vec![zero(); m];
        correction[0] = one() + one() - fg[0];
        for i in 1..m {
            correction[i] = -fg[i];
        }
        g = prefix(&convolution(&g, &correction), m);
    }
    g
}

/// Formal logarithm modulo `x^deg`.
///
/// Requires `deg >= 1` and constant term 1.
pub fn log_series(a: &[Mint], deg: usize) -> Fps {
    assert!(deg >= 1, "degree must be positive");
    assert!(!a.is_empty() && a[0] == one(), "constant term must be one");
    let mut res = convolution(&derivative(a), &inv_series(a, deg));
    res.truncate(deg.saturating_sub(1));
    let mut res = integral(&res);
    res.truncate(deg);
    res
}

/// Formal exponential modulo `x^deg`.
///
/// Requires `deg >= 1` and constant term 0.
pub fn exp_series(a: &[Mint], deg: usize) -> Fps {
    assert!(deg >= 1, "degree must be positive");
    assert!(
        a.first().copied().unwrap_or_else(zero) == zero(),
        "constant term must be zero"
    );

    let mut g = vec![one()];
    while g.len() < deg {
        let m = (g.len() * 2).min(deg);
        let log_g = log_series(&prefix(&g, m), m);
        let mut h = prefix(a, m);
        for i in 0..m {
            h[i] -= log_g.get(i).copied().unwrap_or_else(zero);
        }
        h[0] += one();
        g = prefix(&convolution(&g, &h), m);
    }
    g
}

/// Formal power modulo `x^deg`.
///
/// Negative exponents are supported only for series with a non-zero constant
/// term.
pub fn pow_series(a: &[Mint], exponent: i64, deg: usize) -> Fps {
    if deg == 0 {
        return vec![];
    }
    if exponent == 0 {
        let mut res = vec![zero(); deg];
        res[0] = one();
        return res;
    }

    let lead = a.iter().position(|&x| x != zero());
    let Some(i0) = lead else {
        return vec![zero(); deg];
    };
    if exponent < 0 && i0 != 0 {
        panic!("negative exponent requires a non-zero constant term");
    }

    let shift = i0.saturating_mul(exponent.unsigned_abs() as usize);
    if exponent > 0 && shift >= deg {
        return vec![zero(); deg];
    }

    let a0 = a[i0];
    let inv_a0 = a0.inv();
    let scalar_abs = a0.pow(exponent.unsigned_abs());
    let scalar = if exponent >= 0 {
        scalar_abs
    } else {
        scalar_abs.inv()
    };

    let mut base = mul_scalar(&shift_right(a, i0), inv_a0);
    base.resize(deg, zero());
    let mut logged = log_series(&base, deg);
    let k = Mint::new(exponent);
    for x in &mut logged {
        *x *= k;
    }
    let mut res = mul_scalar(&exp_series(&logged, deg), scalar);
    if exponent > 0 {
        res = shift_left(&res, shift);
    }
    res.truncate(deg);
    res
}

/// Polynomial division. Returns `(quotient, remainder)`.
pub fn div_rem(a: &[Mint], b: &[Mint]) -> (Fps, Fps) {
    let mut aa = a.to_vec();
    let mut bb = b.to_vec();
    shrink(&mut aa);
    shrink(&mut bb);
    assert!(!bb.is_empty(), "division by zero polynomial");
    if aa.len() < bb.len() {
        return (vec![], aa);
    }

    if bb.len() <= 64 {
        let n = aa.len();
        let m = bb.len();
        let inv_lead = bb[m - 1].inv();
        let mut q = vec![zero(); n - m + 1];
        for k in (0..=n - m).rev() {
            let coef = aa[k + m - 1] * inv_lead;
            q[k] = coef;
            for j in 0..m {
                aa[k + j] -= coef * bb[j];
            }
        }
        aa.truncate(m - 1);
        shrink(&mut aa);
        shrink(&mut q);
        return (q, aa);
    }

    let q_len = aa.len() - bb.len() + 1;
    let mut ar = reverse(&aa);
    let mut br = reverse(&bb);
    ar.truncate(q_len);
    br.truncate(q_len);
    let inv_br = inv_series(&br, q_len);
    let mut qr = convolution(&ar, &inv_br);
    qr.truncate(q_len);
    let mut q = reverse(&qr);
    shrink(&mut q);

    let mut prod = convolution(&bb, &q);
    prod.truncate(aa.len());
    for (i, x) in prod.into_iter().enumerate() {
        aa[i] -= x;
    }
    aa.truncate(bb.len() - 1);
    shrink(&mut aa);
    (q, aa)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vals(a: &[Mint]) -> Vec<u64> {
        a.iter().map(|x| x.val()).collect()
    }

    fn naive_mul(a: &[Mint], b: &[Mint], deg: usize) -> Fps {
        let mut res = vec![zero(); deg];
        for (i, &x) in a.iter().enumerate() {
            for (j, &y) in b.iter().enumerate() {
                if i + j < deg {
                    res[i + j] += x * y;
                }
            }
        }
        res
    }

    #[test]
    fn basic_operations() {
        let f = vec![Mint::new(3), Mint::new(4), Mint::new(5)];
        assert_eq!(vals(&derivative(&f)), vec![4, 10]);
        assert_eq!(integral(&derivative(&f))[1..], f[1..]);
        assert_eq!(eval(&f, Mint::new(2)), Mint::new(31));
        assert_eq!(
            vals(&add(&[Mint::new(1)], &[Mint::new(2), Mint::new(3)])),
            vec![3, 3]
        );
        assert_eq!(
            vals(&sub(&[Mint::new(1)], &[Mint::new(2), Mint::new(3)])),
            vec![998244352, 998244350]
        );
    }

    #[test]
    fn inverse_matches_identity() {
        let f = vec![Mint::new(1), Mint::new(-3), Mint::new(2), Mint::new(5)];
        let inv = inv_series(&f, 16);
        let prod = naive_mul(&f, &inv, 16);
        assert_eq!(prod[0], one());
        assert!(prod[1..].iter().all(|&x| x == zero()));
    }

    #[test]
    fn log_and_exp_are_inverse() {
        let f = vec![
            Mint::new(1),
            Mint::new(2),
            Mint::new(3),
            Mint::new(4),
            Mint::new(5),
        ];
        let log_f = log_series(&f, 8);
        assert_eq!(exp_series(&log_f, 8), prefix(&f, 8));

        let g = vec![Mint::new(0), Mint::new(7), Mint::new(-2), Mint::new(9)];
        let exp_g = exp_series(&g, 8);
        assert_eq!(log_series(&exp_g, 8), prefix(&g, 8));
    }

    #[test]
    fn exp_of_x_has_factorial_coefficients() {
        let exp_x = exp_series(&[zero(), one()], 8);
        let mut fact = one();
        for (i, &coef) in exp_x.iter().enumerate() {
            if i > 0 {
                fact *= Mint::from(i);
            }
            assert_eq!(coef, fact.inv());
        }
    }

    #[test]
    fn power_and_division() {
        let f = vec![Mint::new(1), Mint::new(1)];
        assert_eq!(vals(&pow_series(&f, 3, 5)), vec![1, 3, 3, 1, 0]);
        assert_eq!(
            vals(&pow_series(&f, -1, 5)),
            vec![1, 998244352, 1, 998244352, 1]
        );

        let dividend = vec![Mint::new(1), Mint::new(3), Mint::new(3), Mint::new(1)];
        let divisor = vec![Mint::new(1), Mint::new(1)];
        let (q, r) = div_rem(&dividend, &divisor);
        assert_eq!(vals(&q), vec![1, 2, 1]);
        assert!(r.is_empty());
    }

    #[test]
    fn random_inverse_against_naive() {
        let mut x = 123456789u64;
        let mut rng = || {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            x
        };
        for _ in 0..30 {
            let deg = (rng() % 30 + 1) as usize;
            let f: Vec<_> = (0..deg)
                .map(|i| {
                    if i == 0 {
                        Mint::new((rng() % 998244352 + 1) as i64)
                    } else {
                        Mint::raw(rng() % 998244353)
                    }
                })
                .collect();
            let inv = inv_series(&f, deg);
            let prod = naive_mul(&f, &inv, deg);
            assert_eq!(prod[0], one());
            assert!(prod[1..].iter().all(|&v| v == zero()));
        }
    }
}
