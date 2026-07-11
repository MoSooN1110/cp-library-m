//! 素数関連: 線形篩（最小素因数）、Miller-Rabin 素数判定、Pollard rho 素因数分解。
//!
//! ```
//! use cplib::math::prime::*;
//! assert!(is_prime(1_000_000_007));
//! assert!(!is_prime(1_000_000_009 * 3));
//! let mut f = factorize(60);
//! f.sort();
//! assert_eq!(f, vec![2, 2, 3, 5]);
//! ```

/// 最小素因数篩（0..=n）。`spf[x]` が x の最小素因数。
pub fn sieve_spf(n: usize) -> Vec<u32> {
    let mut spf = vec![0u32; n + 1];
    for i in 2..=n {
        if spf[i] == 0 {
            let mut j = i;
            while j <= n {
                if spf[j] == 0 {
                    spf[j] = i as u32;
                }
                j += i;
            }
        }
    }
    spf
}

/// エラトステネスで素数列挙（<= n）。
pub fn primes_upto(n: usize) -> Vec<usize> {
    if n < 2 {
        return vec![];
    }
    let mut is = vec![true; n + 1];
    is[0] = false;
    is[1] = false;
    let mut i = 2;
    while i * i <= n {
        if is[i] {
            let mut j = i * i;
            while j <= n {
                is[j] = false;
                j += i;
            }
        }
        i += 1;
    }
    (2..=n).filter(|&x| is[x]).collect()
}

#[inline]
fn mulmod(a: u64, b: u64, m: u64) -> u64 {
    ((a as u128 * b as u128) % m as u128) as u64
}

fn powmod(mut a: u64, mut e: u64, m: u64) -> u64 {
    let mut r = 1u64 % m;
    a %= m;
    while e > 0 {
        if e & 1 == 1 {
            r = mulmod(r, a, m);
        }
        a = mulmod(a, a, m);
        e >>= 1;
    }
    r
}

/// 決定的 Miller-Rabin（u64 全域で正確）。
pub fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for &p in &[2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if n % p == 0 {
            return n == p;
        }
    }
    let mut d = n - 1;
    let mut s = 0;
    while d & 1 == 0 {
        d >>= 1;
        s += 1;
    }
    for &a in &[2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        let mut x = powmod(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        let mut ok = false;
        for _ in 0..s - 1 {
            x = mulmod(x, x, n);
            if x == n - 1 {
                ok = true;
                break;
            }
        }
        if !ok {
            return false;
        }
    }
    true
}

fn pollard_rho(n: u64) -> u64 {
    if n & 1 == 0 {
        return 2;
    }
    if n % 3 == 0 {
        return 3;
    }
    let mut c = 1u64;
    loop {
        let f = |x: u64| (mulmod(x, x, n) + c) % n;
        let (mut x, mut y, mut d) = (2u64, 2u64, 1u64);
        while d == 1 {
            x = f(x);
            y = f(f(y));
            let diff = if x > y { x - y } else { y - x };
            d = gcd(diff, n);
        }
        if d != n {
            return d;
        }
        c += 1;
    }
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

/// 素因数分解（重複あり・未ソート）。Pollard rho により大きな数も可。
pub fn factorize(n: u64) -> Vec<u64> {
    if n <= 1 {
        return vec![];
    }
    if is_prime(n) {
        return vec![n];
    }
    let d = pollard_rho(n);
    let mut res = factorize(d);
    res.extend(factorize(n / d));
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sieve_and_primes() {
        let spf = sieve_spf(50);
        assert_eq!(spf[49], 7);
        assert_eq!(spf[13], 13);
        assert_eq!(primes_upto(20), vec![2, 3, 5, 7, 11, 13, 17, 19]);
    }
    #[test]
    fn prime_check() {
        assert!(is_prime(2));
        assert!(is_prime(1_000_000_007));
        assert!(is_prime(998_244_353));
        assert!(!is_prime(1));
        assert!(!is_prime(1_000_000_007u64 * 3));
        // 篩と一致
        let ps: std::collections::HashSet<u64> =
            primes_upto(2000).into_iter().map(|x| x as u64).collect();
        for n in 0..2000u64 {
            assert_eq!(is_prime(n), ps.contains(&n));
        }
    }
    #[test]
    fn factor() {
        for &n in &[1u64, 2, 60, 97, 1024, 999_999_937, 600_000_000_000_000_009] {
            let f = factorize(n);
            let prod: u64 = f.iter().product::<u64>().max(1);
            if n >= 1 {
                assert_eq!(if n == 1 { 1 } else { prod }, n);
            }
            for &p in &f {
                assert!(is_prime(p));
            }
        }
    }
}
