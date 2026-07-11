//! Mint 上の行列（積・累乗）。線形漸化式の高速化などに。
//!
//! ```
//! use cplib::math::matrix::*;
//! let a = Matrix::from(vec![vec![1i64.into(), 1i64.into()], vec![1i64.into(), 0i64.into()]]);
//! let f = a.pow(10);   // フィボナッチ行列の 10 乗
//! ```
use crate::math::modint::Mint;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Matrix {
    pub h: usize,
    pub w: usize,
    pub a: Vec<Vec<Mint>>,
}

impl Matrix {
    pub fn zero(h: usize, w: usize) -> Self {
        Matrix {
            h,
            w,
            a: vec![vec![Mint::raw(0); w]; h],
        }
    }
    /// n 次単位行列
    pub fn identity(n: usize) -> Self {
        let mut m = Self::zero(n, n);
        for i in 0..n {
            m.a[i][i] = Mint::from(1i64);
        }
        m
    }
    pub fn from(a: Vec<Vec<Mint>>) -> Self {
        let h = a.len();
        let w = if h == 0 { 0 } else { a[0].len() };
        Matrix { h, w, a }
    }
    pub fn mul(&self, rhs: &Matrix) -> Matrix {
        assert_eq!(self.w, rhs.h);
        let mut c = Matrix::zero(self.h, rhs.w);
        for i in 0..self.h {
            for k in 0..self.w {
                let aik = self.a[i][k];
                if aik.val() == 0 {
                    continue;
                }
                for j in 0..rhs.w {
                    c.a[i][j] += aik * rhs.a[k][j];
                }
            }
        }
        c
    }
    /// 正方行列の e 乗
    pub fn pow(&self, mut e: u64) -> Matrix {
        assert_eq!(self.h, self.w);
        let mut r = Matrix::identity(self.h);
        let mut b = self.clone();
        while e > 0 {
            if e & 1 == 1 {
                r = r.mul(&b);
            }
            b = b.mul(&b);
            e >>= 1;
        }
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fib() {
        // [[1,1],[1,0]]^n の [0][1] が fib(n)
        let a = Matrix::from(vec![
            vec![Mint::from(1i64), Mint::from(1i64)],
            vec![Mint::from(1i64), Mint::from(0i64)],
        ]);
        let f = a.pow(10);
        assert_eq!(f.a[0][1].val(), 55); // fib(10)=55
        assert_eq!(f.a[0][0].val(), 89); // fib(11)=89
    }
    #[test]
    fn identity_mul() {
        let a = Matrix::from(vec![
            vec![Mint::from(2i64), Mint::from(3i64)],
            vec![Mint::from(4i64), Mint::from(5i64)],
        ]);
        let id = Matrix::identity(2);
        assert_eq!(a.mul(&id), a);
    }
}
