//! 高速素数計数 π(n)（Lucy_Hedgehog 系、O(n^(3/4)) 時間・O(√n) 空間）。
//!
//! n = 10^11 程度まで実用的。
//!
//! ```
//! use cplib::math::prime_count::*;
//! assert_eq!(prime_pi(100), 25);
//! assert_eq!(prime_pi(1_000_000), 78498);
//! ```

/// n 以下の素数の個数 π(n)。
pub fn prime_pi(n: u64) -> u64 {
    prime_pi_usize(n as usize) as u64
}

fn prime_pi_usize(n: usize) -> usize {
    if n < 9 {
        // 2, 3, 5, 7
        return [0, 0, 1, 2, 2, 3, 3, 4, 4][n];
    }
    let v = floor_sqrt(n);
    // smalls[i] = π'(i)（奇数だけ数えた近似から篩で補正していく）
    let mut smalls: Vec<usize> = (0..=v).map(|i| (i + 1) / 2).collect();
    let mut s = (v + 1) / 2;
    // roughs[k] = まだ篩われていない k 番目の奇数、larges[k] = n/roughs[k] までの計数
    let mut roughs: Vec<usize> = (0..s).map(|i| 2 * i + 1).collect();
    let mut larges: Vec<usize> = (0..s).map(|i| (n / (2 * i + 1) + 1) / 2).collect();
    let mut skip = vec![false; v + 1];

    let mut pc = 0; // 処理済みの奇素数の個数
    for p in (3..=v).step_by(2) {
        if skip[p] {
            continue;
        }
        let q = p * p;
        pc += 1;
        if q * q > n {
            break;
        }
        skip[p] = true;
        for i in (q..=v).step_by(2 * p) {
            skip[i] = true;
        }
        let mut ns = 0;
        for k in 0..s {
            let i = roughs[k];
            if skip[i] {
                continue;
            }
            let d = i * p;
            let x = if d <= v {
                larges[smalls[d] - pc]
            } else {
                smalls[n / d]
            };
            larges[ns] = larges[k] + pc - x;
            roughs[ns] = i;
            ns += 1;
        }
        s = ns;
        let mut i = v;
        for j in (p..=v / p).rev() {
            let c = smalls[j] - pc;
            let e = j * p;
            while i >= e {
                smalls[i] -= c;
                i -= 1;
            }
        }
    }

    let mut res: usize =
        larges[0] + (s + 2 * (pc - 1)) * (s - 1) / 2 - larges[1..s].iter().sum::<usize>();

    for l in 1..s {
        let q = roughs[l];
        let m = n / q;
        let e = smalls[m / q] - pc;
        if e <= l {
            break;
        }
        let t: usize = roughs[l + 1..=e].iter().map(|&r| smalls[m / r]).sum();
        res += t - (e - l) * (pc + l - 1);
    }
    res
}

fn floor_sqrt(n: usize) -> usize {
    if n <= 1 {
        return n;
    }
    let mut lo = 1;
    let mut hi = n;
    while hi - lo > 1 {
        let mid = lo + (hi - lo) / 2;
        match mid.overflowing_mul(mid) {
            (x, false) if x <= n => lo = mid,
            _ => hi = mid,
        }
    }
    lo
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_values() {
        assert_eq!(prime_pi(0), 0);
        assert_eq!(prime_pi(1), 0);
        assert_eq!(prime_pi(2), 1);
        assert_eq!(prime_pi(3), 2);
        assert_eq!(prime_pi(10), 4);
        assert_eq!(prime_pi(100), 25);
        assert_eq!(prime_pi(1000), 168);
        assert_eq!(prime_pi(1_000_000), 78498);
        assert_eq!(prime_pi(100_000_000), 5_761_455);
    }

    #[test]
    fn matches_sieve_exhaustive() {
        let limit = 3000usize;
        let primes = crate::math::prime::primes_upto(limit);
        let mut pi = vec![0u64; limit + 1];
        for &p in &primes {
            pi[p] += 1;
        }
        for i in 1..=limit {
            pi[i] += pi[i - 1];
        }
        for n in 0..=limit {
            assert_eq!(prime_pi(n as u64), pi[n], "pi({n})");
        }
    }

    #[test]
    fn floor_sqrt_check() {
        for n in 0..1000usize {
            let r = floor_sqrt(n);
            assert!(r * r <= n && (r + 1) * (r + 1) > n, "sqrt({n})={r}");
        }
        assert_eq!(floor_sqrt(usize::MAX), (1usize << 32) - 1);
    }
}
