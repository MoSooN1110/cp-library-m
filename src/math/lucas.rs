//! Lucas の定理による二項係数 C(n, r) mod p（p は小さな素数）。
//!
//! ```
//! use cplib::math::lucas::*;
//! let l = Lucas::new(7);
//! assert_eq!(l.c(10, 3), 120 % 7);   // 120 mod 7 = 1
//! assert_eq!(l.c(3, 5), 0);
//! ```

pub struct Lucas {
    p: u64,
    fact: Vec<u64>,
    finv: Vec<u64>,
}

fn powmod(mut a: u64, mut e: u64, m: u64) -> u64 {
    let mut r = 1 % m;
    a %= m;
    while e > 0 {
        if e & 1 == 1 {
            r = r * a % m;
        }
        a = a * a % m;
        e >>= 1;
    }
    r
}

impl Lucas {
    /// p は素数であること。
    pub fn new(p: u64) -> Self {
        let mut fact = vec![1u64; p as usize];
        for i in 1..p as usize {
            fact[i] = fact[i - 1] * i as u64 % p;
        }
        let mut finv = vec![1u64; p as usize];
        if p >= 2 {
            finv[p as usize - 1] = powmod(fact[p as usize - 1], p - 2, p);
            for i in (1..p as usize - 1).rev() {
                finv[i] = finv[i + 1] * (i as u64 + 1) % p;
            }
        }
        Lucas { p, fact, finv }
    }

    fn c_small(&self, n: u64, r: u64) -> u64 {
        if r > n {
            return 0;
        }
        self.fact[n as usize] * self.finv[r as usize] % self.p * self.finv[(n - r) as usize]
            % self.p
    }

    /// C(n, r) mod p
    pub fn c(&self, mut n: u64, mut r: u64) -> u64 {
        let mut res = 1u64;
        while n > 0 || r > 0 {
            let (ni, ri) = (n % self.p, r % self.p);
            if ri > ni {
                return 0;
            }
            res = res * self.c_small(ni, ri) % self.p;
            n /= self.p;
            r /= self.p;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn matches_pascal() {
        for &p in &[2u64, 3, 5, 7, 11, 13] {
            let l = Lucas::new(p);
            // Pascal でナイーブ
            let n = 60usize;
            let mut c = vec![vec![0u64; n + 1]; n + 1];
            for i in 0..=n {
                c[i][0] = 1;
                for j in 1..=i {
                    c[i][j] = (c[i - 1][j - 1] + c[i - 1][j]) % p;
                }
            }
            for i in 0..=n {
                for j in 0..=n {
                    let expect = if j <= i { c[i][j] } else { 0 };
                    assert_eq!(l.c(i as u64, j as u64), expect, "p{p} C({i},{j})");
                }
            }
        }
    }
}
