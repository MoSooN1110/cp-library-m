// source snippet: key=lib_martix_pow  prefix=lib_martix_pow

pub mod matrix {
    #[derive(Clone)]
    pub struct Matrix {
        pub v: Vec<Vec<i64>>,
    }
    impl Matrix {
        pub fn identity(n: usize) -> Self {
            let mut v = vec![vec![0; n]; n];
            for i in 0..n {
                v[i][i] = 1;
            }
            Matrix { v: v }
        }
        pub fn m(&self) -> usize {
            self.v.len()
        }
        pub fn n(&self) -> usize {
            self.v[0].len()
        }
        pub fn mul_rem(&self, other: &Self, mo: i64) -> Self {
            assert!(self.n() == other.m());
            let K = self.n();
            let M = self.m();
            let N = other.n();
            let mut r = vec![vec![0; N]; M];
            for i in 0..M {
                for j in 0..N {
                    let mut v = 0;
                    for k in 0..K {
                        v += self.v[i][k] * other.v[k][j] % mo;
                        v %= mo;
                    }
                    r[i][j] = v;
                }
            }
            Matrix { v: r }
        }
        pub fn pow(&self, k: u64, mo: i64) -> Self {
            assert!(self.m() == self.n());
            let mut k = k;
            let mut x = Self::identity(self.m());
            let mut y = self.clone();
            while k > 0 {
                if k & 1 > 0 {
                    x = y.mul_rem(&x, mo);
                    x %= mo;
                }
                y = y.mul_rem(&y, mo);
                y %= mo;
                k >>= 1;
            }
            x
        }
    }

    use std::ops::*;

    impl Add for Matrix {
        type Output = Self;
        fn add(self, other: Self) -> Self {
            let mut r = self.v.clone();
            for i in 0..self.m() {
                for j in 0..self.n() {
                    r[i][j] += other.v[i][j];
                }
            }
            Matrix { v: r }
        }
    }
    impl Sub for Matrix {
        type Output = Self;
        fn sub(self, other: Self) -> Self {
            let mut r = self.v.clone();
            for i in 0..self.m() {
                for j in 0..self.n() {
                    r[i][j] -= other.v[i][j];
                }
            }
            Matrix { v: r }
        }
    }
    impl Mul for Matrix {
        type Output = Self;
        fn mul(self, other: Self) -> Self {
            assert!(self.n() == other.m());
            let K = self.n();
            let M = self.m();
            let N = other.n();
            let mut r = vec![vec![0; N]; M];
            for i in 0..M {
                for j in 0..N {
                    let mut v = 0;
                    for k in 0..K {
                        v += self.v[i][k] * other.v[k][j]; // mod over flow?
                    }
                    r[i][j] = v;
                }
            }
            Matrix { v: r }
        }
    }
    impl Rem<i64> for Matrix {
        type Output = Self;
        fn rem(self, mo: i64) -> Self {
            let mut r = self.v.clone();
            for i in 0..self.m() {
                for j in 0..self.n() {
                    r[i][j] %= mo;
                }
            }
            Matrix { v: r }
        }
    }
    impl RemAssign<i64> for Matrix {
        fn rem_assign(&mut self, mo: i64) {
            for i in 0..self.m() {
                for j in 0..self.n() {
                    self.v[i][j] %= mo;
                }
            }
        }
    }
}
fn matrix_pow(a: &Vec<i64>, mat: &Vec<Vec<i64>>, k: u64, mo: i64) -> Vec<i64> {
    //matrix pow res =  mat(n*n).pow(k) * a(n,1)
    let mut m = matrix::Matrix { v: mat.clone() };
    let n = a.len();
    let m = m.pow(k as u64, mo);
    let mut va = vec![vec![0 as i64; (1) as usize]; (n) as usize];
    for i in 0..n {
        va[i][0] = a[i];
    }
    let aa = matrix::Matrix { v: va };
    let p = m * aa;
    let mut r = vec![0 as i64; (n) as usize];
    for i in 0..n {
        r[i] = p.v[i][0];
    }
    r
}
