//! 線形漸化式の推定と高速 k 番目項計算。
//!
//! Berlekamp-Massey で最小線形漸化式を推定し、Bostan-Mori で
//! `a[k]` を `O(d log d log k)` で求める。係数は
//! `a[n] = c[0] a[n - 1] + c[1] a[n - 2] + ... + c[d - 1] a[n - d]`
//! の順で返す。
//!
//! ```rust
//! use cplib::math::linear_recurrence::*;
//! use cplib::math::modint::Mint;
//!
//! let seq: Vec<Mint> = [0, 1, 1, 2, 3, 5, 8, 13]
//!     .into_iter()
//!     .map(Mint::new)
//!     .collect();
//! let rec = berlekamp_massey(&seq);
//! assert_eq!(rec, vec![Mint::new(1), Mint::new(1)]);
//! assert_eq!(linear_recurrence_kth(&seq[..2], &rec, 10), Mint::new(55));
//! ```

use crate::math::convolution::convolution;
use crate::math::modint::Mint;

pub fn berlekamp_massey(s: &[Mint]) -> Vec<Mint> {
    let mut c = vec![Mint::raw(1)];
    let mut b = vec![Mint::raw(1)];
    let mut l = 0usize;
    let mut m = 1usize;
    let mut bb = Mint::raw(1);

    for i in 0..s.len() {
        let mut d = Mint::raw(0);
        for j in 0..=l {
            d += c[j] * s[i - j];
        }
        if d == Mint::raw(0) {
            m += 1;
            continue;
        }

        let old_c = c.clone();
        let coef = d / bb;
        if c.len() < b.len() + m {
            c.resize(b.len() + m, Mint::raw(0));
        }
        for j in 0..b.len() {
            c[j + m] -= coef * b[j];
        }
        if 2 * l <= i {
            l = i + 1 - l;
            b = old_c;
            bb = d;
            m = 1;
        } else {
            m += 1;
        }
    }

    c.remove(0);
    for x in &mut c {
        *x = -*x;
    }
    c
}

pub fn bostan_mori(mut numerator: Vec<Mint>, mut denominator: Vec<Mint>, mut k: u64) -> Mint {
    assert!(!denominator.is_empty());
    assert_ne!(denominator[0], Mint::raw(0));
    while k > 0 {
        let mut denominator_neg = denominator.clone();
        for i in (1..denominator_neg.len()).step_by(2) {
            denominator_neg[i] = -denominator_neg[i];
        }
        let p = convolution(&numerator, &denominator_neg);
        let q = convolution(&denominator, &denominator_neg);
        numerator = if k & 1 == 0 {
            p.into_iter().step_by(2).collect()
        } else {
            p.into_iter().skip(1).step_by(2).collect()
        };
        denominator = q.into_iter().step_by(2).collect();
        k >>= 1;
    }
    numerator.first().copied().unwrap_or(Mint::raw(0)) / denominator[0]
}

pub fn linear_recurrence_kth(initial: &[Mint], recurrence: &[Mint], k: u64) -> Mint {
    let d = recurrence.len();
    if d == 0 {
        return initial
            .first()
            .copied()
            .filter(|_| k == 0)
            .unwrap_or(Mint::raw(0));
    }
    if (k as usize) < initial.len() {
        return initial[k as usize];
    }
    assert!(initial.len() >= d);

    let mut denominator = vec![Mint::raw(1); d + 1];
    for i in 0..d {
        denominator[i + 1] = -recurrence[i];
    }
    let mut init = initial[..d].to_vec();
    let numerator_len = d;
    init.resize(d, Mint::raw(0));
    let mut numerator = convolution(&denominator, &init);
    numerator.truncate(numerator_len);
    bostan_mori(numerator, denominator, k)
}

pub fn guess_kth_term(samples: &[Mint], k: u64) -> Mint {
    let recurrence = berlekamp_massey(samples);
    linear_recurrence_kth(samples, &recurrence, k)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fib_fast_doubling(n: u64) -> (Mint, Mint) {
        if n == 0 {
            return (Mint::raw(0), Mint::raw(1));
        }
        let (a, b) = fib_fast_doubling(n >> 1);
        let c = a * (b + b - a);
        let d = a * a + b * b;
        if n & 1 == 0 {
            (c, d)
        } else {
            (d, c + d)
        }
    }

    fn naive_recurrence(initial: &[Mint], recurrence: &[Mint], n: usize) -> Vec<Mint> {
        let d = recurrence.len();
        let mut a = initial.to_vec();
        a.resize(n.max(a.len()), Mint::raw(0));
        for i in initial.len()..n {
            let mut x = Mint::raw(0);
            for j in 0..d {
                x += recurrence[j] * a[i - 1 - j];
            }
            a[i] = x;
        }
        a
    }

    #[test]
    fn fibonacci() {
        let seq: Vec<Mint> = (0..20).map(|i| fib_fast_doubling(i).0).collect();
        let recurrence = berlekamp_massey(&seq);
        assert_eq!(recurrence, vec![Mint::raw(1), Mint::raw(1)]);
        assert_eq!(
            linear_recurrence_kth(&seq[..2], &recurrence, 1_000_000),
            fib_fast_doubling(1_000_000).0
        );
    }

    #[test]
    fn arithmetic_progression() {
        let seq: Vec<Mint> = (0..20).map(|i| Mint::new(3 * i + 7)).collect();
        let recurrence = berlekamp_massey(&seq);
        assert_eq!(recurrence, vec![Mint::new(2), Mint::new(-1)]);
        assert_eq!(
            linear_recurrence_kth(&seq[..2], &recurrence, 12345),
            Mint::new(3 * 12345 + 7)
        );
    }

    #[test]
    fn bostan_mori_known_generating_function() {
        let numerator = vec![Mint::raw(1)];
        let denominator = vec![Mint::raw(1), Mint::new(-2)];
        for k in 0..50 {
            assert_eq!(
                bostan_mori(numerator.clone(), denominator.clone(), k),
                Mint::raw(2).pow(k)
            );
        }
    }

    #[test]
    fn random_recurrences_match_naive() {
        let mut seed = 975318642u64;
        let mut rng = || {
            seed ^= seed << 7;
            seed ^= seed >> 9;
            seed
        };
        for d in 1usize..=8 {
            for _ in 0..100 {
                let initial: Vec<Mint> = (0..d).map(|_| Mint::from(rng() % 1000)).collect();
                let recurrence: Vec<Mint> = (0..d).map(|_| Mint::from(rng() % 1000)).collect();
                let seq = naive_recurrence(&initial, &recurrence, 80);
                let guessed = berlekamp_massey(&seq[..2 * d + 10]);
                for k in 0..80u64 {
                    assert_eq!(
                        linear_recurrence_kth(&initial, &recurrence, k),
                        seq[k as usize]
                    );
                    assert_eq!(guess_kth_term(&seq[..2 * d + 10], k), seq[k as usize]);
                }
                assert!(guessed.len() <= d);
            }
        }
    }
}
