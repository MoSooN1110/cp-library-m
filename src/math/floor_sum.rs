//! floor_sum: `sum_{i=0}^{n-1} floor((a*i + b) / m)`（ACL 準拠、負も可）。
//!
//! ```
//! use cplib::math::floor_sum::*;
//! // sum_{i=0}^{3} floor((2i+1)/3) = 0+1+1+2 = ... i=0..4
//! let s: i64 = (0..4).map(|i| (2*i + 1) / 3).sum();
//! assert_eq!(floor_sum(4, 3, 2, 1), s);
//! ```

/// `sum_{i=0}^{n-1} floor((a*i + b) / m)`。n>=0, m>=1。a,b は負でも可。
pub fn floor_sum(n: i64, m: i64, mut a: i64, mut b: i64) -> i64 {
    assert!(n >= 0 && m >= 1);
    let mut ans: i64 = 0;
    if a < 0 {
        let a2 = a.rem_euclid(m);
        ans -= n * (n - 1) / 2 * ((a2 - a) / m);
        a = a2;
    }
    if b < 0 {
        let b2 = b.rem_euclid(m);
        ans -= n * ((b2 - b) / m);
        b = b2;
    }
    let mut n = n;
    let mut m = m;
    loop {
        if a >= m {
            ans += n * (n - 1) / 2 * (a / m);
            a %= m;
        }
        if b >= m {
            ans += n * (b / m);
            b %= m;
        }
        let y_max = a * n + b;
        if y_max < m {
            break;
        }
        let nn = y_max / m;
        let bb = y_max % m;
        // 変数変換
        let new_n = nn;
        let new_m = a;
        let new_a = m;
        let new_b = bb;
        n = new_n;
        m = new_m;
        a = new_a;
        b = new_b;
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;
    fn brute(n: i64, m: i64, a: i64, b: i64) -> i64 {
        (0..n).map(|i| (a * i + b).div_euclid(m)).sum()
    }
    #[test]
    fn matches_brute() {
        for n in 0..20i64 {
            for m in 1..12i64 {
                for a in -8..12i64 {
                    for b in -8..12i64 {
                        assert_eq!(floor_sum(n, m, a, b), brute(n, m, a, b), "n{n} m{m} a{a} b{b}");
                    }
                }
            }
        }
    }
}
