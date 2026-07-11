//! オイラーのトーシェント関数 φ（単発・篩）。
//!
//! ```
//! use cplib::math::euler_phi::*;
//! assert_eq!(euler_phi(12), 4);   // 1,5,7,11
//! assert_eq!(euler_phi(1), 1);
//! let t = euler_phi_table(10);
//! assert_eq!(t[10], 4);
//! ```

pub fn euler_phi(mut n: u64) -> u64 {
    let mut res = n;
    let mut p = 2u64;
    while p * p <= n {
        if n % p == 0 {
            res -= res / p;
            while n % p == 0 {
                n /= p;
            }
        }
        p += 1;
    }
    if n > 1 {
        res -= res / n;
    }
    res
}

/// 0..=n の φ 値テーブル（φ[0]=0）。
pub fn euler_phi_table(n: usize) -> Vec<u32> {
    let mut phi: Vec<u32> = (0..=n as u32).collect();
    for i in 2..=n {
        if phi[i] == i as u32 {
            // i は素数
            let mut j = i;
            while j <= n {
                phi[j] -= phi[j] / i as u32;
                j += i;
            }
        }
    }
    phi
}

#[cfg(test)]
mod tests {
    use super::*;
    fn brute(n: u64) -> u64 {
        fn gcd(a: u64, b: u64) -> u64 {
            if b == 0 {
                a
            } else {
                gcd(b, a % b)
            }
        }
        if n == 1 {
            return 1;
        }
        (1..n).filter(|&k| gcd(k, n) == 1).count() as u64
    }
    #[test]
    fn single_and_table() {
        for n in 1..=500u64 {
            assert_eq!(euler_phi(n), brute(n));
        }
        let t = euler_phi_table(500);
        for n in 1..=500 {
            assert_eq!(t[n] as u64, brute(n as u64));
        }
    }
}
