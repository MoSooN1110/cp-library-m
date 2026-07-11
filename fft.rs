// source snippet: key=lib_fft  prefix=lib_fft

mod complex {
    #[derive(Clone, Copy, Debug)]
    pub struct Complex {
        pub x: f64,
        pub y: f64,
    }

    impl Complex {
        pub fn new(x: f64, y: f64) -> Self {
            Complex { x: x, y: y }
        }
        pub fn polar(r: f64, theta: f64) -> Self {
            Complex::new(r * theta.cos(), r * theta.sin())
        }
        pub fn conj(&self) -> Self {
            Complex::new(self.x, -self.y)
        }
        pub fn abs(&self) -> f64 {
            (self.x * self.x + self.y * self.y).sqrt()
        }
        pub fn arg(&self) -> f64 {
            self.y.atan2(self.x)
        }
    }

    use std::ops::*;

    impl Add for Complex {
        type Output = Self;
        fn add(self, rhs: Self) -> Self {
            Complex::new(self.x + rhs.x, self.y + rhs.y)
        }
    }

    impl Sub for Complex {
        type Output = Self;
        fn sub(self, rhs: Self) -> Self {
            Complex::new(self.x - rhs.x, self.y - rhs.y)
        }
    }

    impl Mul for Complex {
        type Output = Self;
        fn mul(self, rhs: Self) -> Self {
            Complex::new(
                self.x * rhs.x - self.y * rhs.y,
                self.x * rhs.y + self.y * rhs.x,
            )
        }
    }

    impl Div for Complex {
        type Output = Self;
        fn div(self, rhs: Self) -> Self {
            let z = self * rhs.conj();
            let a = rhs.x * rhs.x + rhs.y * rhs.y;
            Complex::new(z.x / a, z.y / a)
        }
    }

    impl AddAssign for Complex {
        fn add_assign(&mut self, rhs: Self) {
            *self = *self + rhs
        }
    }
    impl SubAssign for Complex {
        fn sub_assign(&mut self, rhs: Self) {
            *self = *self - rhs
        }
    }
    impl MulAssign for Complex {
        fn mul_assign(&mut self, rhs: Self) {
            *self = *self * rhs
        }
    }
    impl DivAssign for Complex {
        fn div_assign(&mut self, rhs: Self) {
            *self = *self / rhs
        }
    }
}
pub type Complex = complex::Complex;

pub fn multiply(a: &[i64], b: &[i64], mo: i64) -> Vec<i64> {
    let n = a.len();
    let m = b.len();
    let mut fa = vec![];
    let mut fb = vec![];
    for i in 0..n {
        fa.push(a[i] as f64)
    }
    for i in 0..m {
        fb.push(b[i] as f64)
    }
    let fc = convolve(fa, fb);
    let mut c = vec![];
    for x in fc {
        let v = (x + 0.5) as i64;
        c.push(v % mo);
    }
    c
}

#[doc = "convolve two waves a[x],b[y] to c[x+y]. O(nlogn)"]
pub fn convolve(a: Vec<f64>, b: Vec<f64>) -> Vec<f64> {
    let n = a.len() + b.len() - 1;
    let mut m = 1;
    while m < n {
        m *= 2;
    }
    let mut x = vec![Complex::new(0., 0.); m];
    for i in 0..a.len() {
        x[i] = Complex::new(a[i], 0.);
    }
    let mut y = vec![Complex::new(0., 0.); m];
    for i in 0..b.len() {
        y[i] = Complex::new(b[i], 0.);
    }
    let X = fast_fourier_transform(x, false);
    let Y = fast_fourier_transform(y, false);
    let mut Z = vec![Complex::new(0., 0.); m];
    for i in 0..m {
        Z[i] = X[i] * Y[i];
    }
    let z = fast_fourier_transform(Z, true);
    let mut ret = vec![0.; m];
    for i in 0..m {
        ret[i] = z[i].x;
    }
    ret
}

pub fn fast_fourier_transform(arr: Vec<Complex>, inv: bool) -> Vec<Complex> {
    let n = arr.len();
    assert!(n.count_ones() == 1, "the length of array is not square");
    let mut a: Vec<_> = arr.to_vec();
    let mut tmp: Vec<_> = (0..n).map(|_| Complex::new(0., 0.)).collect();
    let mut ai: Vec<_> = (0..n).map(|i| i).collect();
    let mut ti: Vec<_> = (0..n).map(|_| 0).collect();
    let bit = n.trailing_zeros();
    let f = if inv { -1.0 } else { 1.0 };
    for si in (0..bit).rev() {
        let s = 1 << si;
        std::mem::swap(&mut a, &mut tmp);
        std::mem::swap(&mut ai, &mut ti);
        let zeta = Complex::polar(1.0, std::f64::consts::PI * 2.0 * f / (s << 1) as f64);
        let mut z_i = Complex::new(1.0, 0.0);
        let mut ev = 0;
        let mut od = 1;
        for i in 0..n {
            if (i & s) != 0 {
                a[i] = (tmp[i - s] - tmp[i]) * z_i;
                ai[i] = ti[od];
                od += 2;
                z_i *= zeta;
            } else {
                a[i] = tmp[i] + tmp[i + s];
                ai[i] = ti[ev];
                ev += 2;
                z_i = Complex::new(1.0, 0.0);
            }
        }
    }

    std::mem::swap(&mut a, &mut tmp);
    let inv_n = if inv { n as f64 } else { 1.0 };
    for i in 0..n {
        a[ai[i]] = Complex::new(tmp[i].x / inv_n, tmp[i].y / inv_n);
    }
    a
}
