//! f64 FFT。実数列の畳み込みと、任意 mod（NTT-friendly でなくてよい）の乗算。
//!
//! `multiply` は 15bit 分割で精度を確保する（mod は 2^30 以下、
//! 出力長はおよそ 2×10^6 まで安全）。mod 998244353 なら NTT 版の方が高速・正確。
//!
//! ```
//! use cplib::math::fft::*;
//! let c = multiply(&[1, 2], &[3, 4], 1_000_000_007);
//! assert_eq!(c, vec![3, 10, 8]);
//! let r = convolve(&[1.0, 2.0, 3.0], &[4.0, 5.0, 6.0]);
//! assert!((r[2] - 28.0).abs() < 1e-6);
//! ```

#[derive(Clone, Copy, Debug, Default)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

impl Complex {
    pub fn new(re: f64, im: f64) -> Self {
        Complex { re, im }
    }
    pub fn polar(r: f64, theta: f64) -> Self {
        Complex::new(r * theta.cos(), r * theta.sin())
    }
    pub fn conj(self) -> Self {
        Complex::new(self.re, -self.im)
    }
}

impl std::ops::Add for Complex {
    type Output = Complex;
    fn add(self, r: Complex) -> Complex {
        Complex::new(self.re + r.re, self.im + r.im)
    }
}
impl std::ops::Sub for Complex {
    type Output = Complex;
    fn sub(self, r: Complex) -> Complex {
        Complex::new(self.re - r.re, self.im - r.im)
    }
}
impl std::ops::Mul for Complex {
    type Output = Complex;
    fn mul(self, r: Complex) -> Complex {
        Complex::new(
            self.re * r.re - self.im * r.im,
            self.re * r.im + self.im * r.re,
        )
    }
}

/// in-place FFT（invert=true で逆変換、1/n 倍まで行う）。長さは 2 冪。
pub fn fft(a: &mut [Complex], invert: bool) {
    let n = a.len();
    assert!(n.is_power_of_two(), "fft length must be a power of two");
    // ビット反転並べ替え
    let mut j = 0usize;
    for i in 1..n {
        let mut bit = n >> 1;
        while j & bit != 0 {
            j ^= bit;
            bit >>= 1;
        }
        j |= bit;
        if i < j {
            a.swap(i, j);
        }
    }
    let sign = if invert { -1.0 } else { 1.0 };
    let mut len = 2usize;
    while len <= n {
        let ang = sign * 2.0 * std::f64::consts::PI / len as f64;
        let wl = Complex::polar(1.0, ang);
        for i in (0..n).step_by(len) {
            let mut w = Complex::new(1.0, 0.0);
            for k in 0..len / 2 {
                let u = a[i + k];
                let v = a[i + k + len / 2] * w;
                a[i + k] = u + v;
                a[i + k + len / 2] = u - v;
                w = w * wl;
            }
        }
        len <<= 1;
    }
    if invert {
        let inv_n = 1.0 / n as f64;
        for x in a.iter_mut() {
            *x = Complex::new(x.re * inv_n, x.im * inv_n);
        }
    }
}

/// 実数列の畳み込み c[k] = Σ_{i+j=k} a[i]·b[j]。長さ |a|+|b|-1。O(n log n)。
pub fn convolve(a: &[f64], b: &[f64]) -> Vec<f64> {
    if a.is_empty() || b.is_empty() {
        return vec![];
    }
    let need = a.len() + b.len() - 1;
    let n = need.next_power_of_two();
    let mut fa = vec![Complex::default(); n];
    let mut fb = vec![Complex::default(); n];
    for (i, &x) in a.iter().enumerate() {
        fa[i].re = x;
    }
    for (i, &x) in b.iter().enumerate() {
        fb[i].re = x;
    }
    fft(&mut fa, false);
    fft(&mut fb, false);
    for i in 0..n {
        fa[i] = fa[i] * fb[i];
    }
    fft(&mut fa, true);
    fa.truncate(need);
    fa.iter().map(|z| z.re).collect()
}

/// 任意 mod の畳み込み乗算（15bit 分割で誤差を抑える）。
/// mo は 1..=2^30。入力は任意の i64（内部で mod に正規化）。結果は [0, mo)。
pub fn multiply(a: &[i64], b: &[i64], mo: i64) -> Vec<i64> {
    assert!(1 <= mo && mo <= 1 << 30, "modulus must be in 1..=2^30");
    if a.is_empty() || b.is_empty() {
        return vec![];
    }
    let ar: Vec<i64> = a.iter().map(|&x| x.rem_euclid(mo)).collect();
    let br: Vec<i64> = b.iter().map(|&x| x.rem_euclid(mo)).collect();
    // x = hi·2^15 + lo に分割
    let hi_lo = |v: &[i64]| -> (Vec<f64>, Vec<f64>) {
        (
            v.iter().map(|&x| (x >> 15) as f64).collect(),
            v.iter().map(|&x| (x & 32767) as f64).collect(),
        )
    };
    let (a1, a0) = hi_lo(&ar);
    let (b1, b0) = hi_lo(&br);
    let c11 = convolve(&a1, &b1);
    let c00 = convolve(&a0, &b0);
    let asum: Vec<f64> = a1.iter().zip(&a0).map(|(x, y)| x + y).collect();
    let bsum: Vec<f64> = b1.iter().zip(&b0).map(|(x, y)| x + y).collect();
    let cmid = convolve(&asum, &bsum); // = c11 + c00 + (a1*b0 + a0*b1)
    let need = a.len() + b.len() - 1;
    let mut res = vec![0i64; need];
    for i in 0..need {
        let hi = (c11[i].round() as i64) % mo;
        let mid = ((cmid[i] - c11[i] - c00[i]).round() as i64) % mo;
        let lo = (c00[i].round() as i64) % mo;
        res[i] = (((hi << 30) % mo + (mid << 15)) % mo + lo) % mo;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    fn naive_mod(a: &[i64], b: &[i64], mo: i64) -> Vec<i64> {
        if a.is_empty() || b.is_empty() {
            return vec![];
        }
        let mut c = vec![0i64; a.len() + b.len() - 1];
        for (i, &x) in a.iter().enumerate() {
            for (j, &y) in b.iter().enumerate() {
                c[i + j] = (c[i + j] + x.rem_euclid(mo) * y.rem_euclid(mo)) % mo;
            }
        }
        c
    }

    #[test]
    fn real_convolve_known() {
        let r = convolve(&[1.0, 2.0, 3.0], &[4.0, 5.0, 6.0]);
        let expect = [4.0, 13.0, 28.0, 27.0, 18.0];
        assert_eq!(r.len(), expect.len());
        for (x, e) in r.iter().zip(expect.iter()) {
            assert!((x - e).abs() < 1e-6, "{x} vs {e}");
        }
        assert!(convolve(&[], &[1.0]).is_empty());
    }

    #[test]
    fn multiply_random_vs_naive() {
        let mut x: u64 = 1000000007;
        let mut rng = move || {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            x
        };
        for &mo in &[1_000_000_007i64, 998_244_353, 97, 1 << 30] {
            for _ in 0..8 {
                let n = 1 + (rng() % 300) as usize;
                let m = 1 + (rng() % 300) as usize;
                let a: Vec<i64> = (0..n).map(|_| (rng() % (mo as u64)) as i64).collect();
                let b: Vec<i64> = (0..m).map(|_| (rng() % (mo as u64)) as i64).collect();
                assert_eq!(multiply(&a, &b, mo), naive_mod(&a, &b, mo), "mo={mo}");
            }
        }
        // 負数入力
        assert_eq!(multiply(&[-1, 5], &[3], 7), naive_mod(&[-1, 5], &[3], 7));
        assert!(multiply(&[], &[1], 7).is_empty());
    }

    #[test]
    fn agrees_with_ntt() {
        // NTT 版（mod 998244353）との一致
        let mut x: u64 = 555;
        let mut rng = move || {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            x
        };
        let mo = 998_244_353i64;
        for _ in 0..5 {
            let n = 50 + (rng() % 200) as usize;
            let m = 50 + (rng() % 200) as usize;
            let a: Vec<i64> = (0..n).map(|_| (rng() % (mo as u64)) as i64).collect();
            let b: Vec<i64> = (0..m).map(|_| (rng() % (mo as u64)) as i64).collect();
            assert_eq!(
                multiply(&a, &b, mo),
                crate::math::convolution::convolution_i64(&a, &b)
            );
        }
    }
}
