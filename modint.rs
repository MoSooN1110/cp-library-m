// modint + combinatorics (standalone / copy-paste ready)
//
// - MOD はこのファイル内で完結（外部の const MOD に依存しない）
// - 素数 MOD を仮定（inv は Fermat の小定理）。1e9+7 / 998244353 を切り替え可
// - 組合せ（階乗テーブル）は Comb に内包。nCr / nPr / nHr / 階乗 / 逆元を O(1) 参照
//
// 使い方:
//   let x = Mint::new(3) * Mint::new(5);   // 15
//   let c = Comb::new(200_000);
//   let ans = c.c(n, r);                   // nCr
//   println!("{}", ans);

use std::ops::*;

const MOD: u64 = 998_244_353;
// const MOD: u64 = 1_000_000_007;

#[derive(Clone, Copy, PartialEq, Eq, Default, Hash)]
pub struct Mint(u64); // 常に [0, MOD)

impl Mint {
    #[inline]
    pub fn new<T: Into<i64>>(x: T) -> Self {
        Mint(x.into().rem_euclid(MOD as i64) as u64)
    }
    /// 0 <= v < MOD を保証できる場合の生成（剰余を取らない）
    #[inline]
    pub fn raw(v: u64) -> Self {
        Mint(v)
    }
    #[inline]
    pub fn val(self) -> u64 {
        self.0
    }
    #[inline]
    pub fn pow(self, mut e: u64) -> Self {
        let mut a = self;
        let mut r = Mint(1);
        while e > 0 {
            if e & 1 == 1 {
                r *= a;
            }
            a *= a;
            e >>= 1;
        }
        r
    }
    /// 逆元（MOD は素数、self != 0 が前提）
    #[inline]
    pub fn inv(self) -> Self {
        self.pow(MOD - 2)
    }
}

impl std::fmt::Display for Mint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::fmt::Debug for Mint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<i64> for Mint {
    fn from(x: i64) -> Self {
        Mint::new(x)
    }
}
impl From<usize> for Mint {
    fn from(x: usize) -> Self {
        Mint(x as u64 % MOD)
    }
}
impl From<u64> for Mint {
    fn from(x: u64) -> Self {
        Mint(x % MOD)
    }
}
impl From<i32> for Mint {
    fn from(x: i32) -> Self {
        Mint::new(x as i64)
    }
}

impl Add for Mint {
    type Output = Mint;
    #[inline]
    fn add(self, r: Mint) -> Mint {
        let mut v = self.0 + r.0;
        if v >= MOD {
            v -= MOD;
        }
        Mint(v)
    }
}
impl Sub for Mint {
    type Output = Mint;
    #[inline]
    fn sub(self, r: Mint) -> Mint {
        let mut v = self.0 + MOD - r.0;
        if v >= MOD {
            v -= MOD;
        }
        Mint(v)
    }
}
impl Mul for Mint {
    type Output = Mint;
    #[inline]
    fn mul(self, r: Mint) -> Mint {
        Mint(self.0 * r.0 % MOD)
    }
}
impl Div for Mint {
    type Output = Mint;
    #[inline]
    fn div(self, r: Mint) -> Mint {
        self * r.inv()
    }
}
impl Neg for Mint {
    type Output = Mint;
    #[inline]
    fn neg(self) -> Mint {
        Mint(if self.0 == 0 { 0 } else { MOD - self.0 })
    }
}
impl AddAssign for Mint {
    #[inline]
    fn add_assign(&mut self, r: Mint) {
        *self = *self + r;
    }
}
impl SubAssign for Mint {
    #[inline]
    fn sub_assign(&mut self, r: Mint) {
        *self = *self - r;
    }
}
impl MulAssign for Mint {
    #[inline]
    fn mul_assign(&mut self, r: Mint) {
        *self = *self * r;
    }
}
impl DivAssign for Mint {
    #[inline]
    fn div_assign(&mut self, r: Mint) {
        *self = *self / r;
    }
}
impl std::iter::Sum for Mint {
    fn sum<I: Iterator<Item = Mint>>(iter: I) -> Mint {
        iter.fold(Mint(0), |a, b| a + b)
    }
}
impl std::iter::Product for Mint {
    fn product<I: Iterator<Item = Mint>>(iter: I) -> Mint {
        iter.fold(Mint(1), |a, b| a * b)
    }
}

/// 組合せ（階乗・逆階乗を前計算）。MOD は素数、n < MOD が前提。
pub struct Comb {
    fact: Vec<Mint>,
    finv: Vec<Mint>,
}

impl Comb {
    /// 0..=max_n までの階乗を前計算
    pub fn new(max_n: usize) -> Self {
        let n = max_n + 1;
        let mut fact = vec![Mint(1); n];
        let mut finv = vec![Mint(1); n];
        for i in 1..n {
            fact[i] = fact[i - 1] * Mint::from(i);
        }
        finv[n - 1] = fact[n - 1].inv();
        for i in (1..n).rev() {
            finv[i - 1] = finv[i] * Mint::from(i);
        }
        Comb { fact, finv }
    }
    #[inline]
    pub fn fact(&self, n: usize) -> Mint {
        self.fact[n]
    }
    #[inline]
    pub fn finv(&self, n: usize) -> Mint {
        self.finv[n]
    }
    /// n の逆元（1 <= n）
    #[inline]
    pub fn inv(&self, n: usize) -> Mint {
        self.finv[n] * self.fact[n - 1]
    }
    /// nCr
    #[inline]
    pub fn c(&self, n: usize, r: usize) -> Mint {
        if r > n {
            return Mint(0);
        }
        self.fact[n] * self.finv[r] * self.finv[n - r]
    }
    /// nPr
    #[inline]
    pub fn p(&self, n: usize, r: usize) -> Mint {
        if r > n {
            return Mint(0);
        }
        self.fact[n] * self.finv[n - r]
    }
    /// nHr = 重複組合せ = C(n+r-1, r)
    #[inline]
    pub fn h(&self, n: usize, r: usize) -> Mint {
        if n == 0 {
            return if r == 0 { Mint(1) } else { Mint(0) };
        }
        self.c(n + r - 1, r)
    }
}
