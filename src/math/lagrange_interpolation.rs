//! ラグランジュ補間（mod 素数）。`math::modint` に依存。
//!
//! - `lagrange_consecutive`: 標本点が f(0), f(1), ..., f(n) のとき f(x) を O(n)。
//! - `lagrange_arbitrary`: 任意標本点 O(n^2)。
//!
//! ```
//! use cplib::math::modint::*;
//! use cplib::math::lagrange_interpolation::*;
//! // f(i) = i^2
//! let ys: Vec<Mint> = (0..4).map(|i| Mint::new(i * i)).collect();
//! assert_eq!(lagrange_consecutive(&ys, 10).val(), 100);
//! ```

use crate::math::modint::Mint;

/// ys = [f(0), f(1), ..., f(n)]（次数 n 以下の多項式）から f(x) を O(n) で求める。
pub fn lagrange_consecutive(ys: &[Mint], x: i64) -> Mint {
    let n = ys.len();
    assert!(n >= 1);
    let t = Mint::new(x);
    // prefix[i] = Π_{j<i} (t-j), suffix[i] = Π_{j>=i} (t-j)
    let mut prefix = vec![Mint::new(1); n + 1];
    for i in 0..n {
        prefix[i + 1] = prefix[i] * (t - Mint::new(i as i64));
    }
    let mut suffix = vec![Mint::new(1); n + 1];
    for i in (0..n).rev() {
        suffix[i] = suffix[i + 1] * (t - Mint::new(i as i64));
    }
    // 階乗逆元
    let mut fact = vec![Mint::new(1); n];
    for i in 1..n {
        fact[i] = fact[i - 1] * Mint::new(i as i64);
    }
    let mut finv = vec![Mint::new(1); n];
    finv[n - 1] = fact[n - 1].inv();
    for i in (1..n).rev() {
        finv[i - 1] = finv[i] * Mint::new(i as i64);
    }
    let mut res = Mint::new(0);
    for i in 0..n {
        let mut term = ys[i] * prefix[i] * suffix[i + 1] * finv[i] * finv[n - 1 - i];
        if (n - 1 - i) % 2 == 1 {
            term = Mint::new(0) - term;
        }
        res = res + term;
    }
    res
}

/// 任意の標本点 (xs[i], ys[i])（xs は相異なる）から f(x) を O(n^2) で求める。
pub fn lagrange_arbitrary(xs: &[Mint], ys: &[Mint], x: Mint) -> Mint {
    let n = xs.len();
    assert_eq!(n, ys.len());
    let mut res = Mint::new(0);
    for i in 0..n {
        let mut num = ys[i];
        let mut den = Mint::new(1);
        for j in 0..n {
            if i != j {
                num = num * (x - xs[j]);
                den = den * (xs[i] - xs[j]);
            }
        }
        res = res + num * den.inv();
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::misc::xorshift::XorShift;

    fn eval_poly(coef: &[Mint], x: Mint) -> Mint {
        let mut r = Mint::new(0);
        for &c in coef.iter().rev() {
            r = r * x + c;
        }
        r
    }

    #[test]
    fn test_consecutive_random_poly() {
        let mut rng = XorShift::new(31415);
        for _ in 0..100 {
            let deg = rng.next_range(8) as usize;
            let coef: Vec<Mint> =
                (0..=deg).map(|_| Mint::new(rng.next_range(1000) as i64)).collect();
            let ys: Vec<Mint> = (0..=deg as i64).map(|i| eval_poly(&coef, Mint::new(i))).collect();
            for _ in 0..5 {
                let x = rng.next_range(1_000_000_007) as i64 - 500_000_000;
                assert_eq!(lagrange_consecutive(&ys, x).val(), eval_poly(&coef, Mint::new(x)).val());
            }
            // 標本点そのもの
            assert_eq!(lagrange_consecutive(&ys, deg as i64).val(), ys[deg].val());
        }
    }

    #[test]
    fn test_arbitrary_matches_consecutive() {
        let mut rng = XorShift::new(9);
        for _ in 0..50 {
            let deg = rng.next_range(6) as usize;
            let coef: Vec<Mint> =
                (0..=deg).map(|_| Mint::new(rng.next_range(1000) as i64)).collect();
            let xs: Vec<Mint> = (0..=deg as i64).map(Mint::new).collect();
            let ys: Vec<Mint> = xs.iter().map(|&x| eval_poly(&coef, x)).collect();
            let x = Mint::new(rng.next_range(1_000_000) as i64);
            assert_eq!(
                lagrange_arbitrary(&xs, &ys, x).val(),
                lagrange_consecutive(&ys, x.val() as i64).val()
            );
        }
    }

    #[test]
    fn test_sum_of_squares() {
        // Σ_{i=0}^{x} i^2 は 3 次多項式 → 標本 4 点で決まる
        let mut acc = 0i64;
        let ys: Vec<Mint> = (0..4)
            .map(|i| {
                acc += i * i;
                Mint::new(acc)
            })
            .collect();
        let f = |x: i64| x * (x + 1) * (2 * x + 1) / 6 % 998_244_353;
        for x in [10i64, 100, 1000] {
            assert_eq!(lagrange_consecutive(&ys, x).val() as i64, f(x));
        }
    }
}
