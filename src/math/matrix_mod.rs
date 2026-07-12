//! mod 998244353 上の行列計算（rank・行列式・逆行列・連立一次方程式）。
//! `math::modint` に依存する。
//!
//! ```
//! use cplib::math::matrix_mod::*;
//! use cplib::math::modint::Mint;
//!
//! let a = vec![vec![Mint::new(1), Mint::new(2)], vec![Mint::new(3), Mint::new(4)]];
//! assert_eq!(determinant(a.clone()).val(), 998244351);
//! let inv = inverse(a).unwrap();
//! assert_eq!((inv[0][0] * Mint::new(1) + inv[0][1] * Mint::new(3)).val(), 1);
//! ```

use crate::math::modint::Mint;

pub fn rank(mut a: Vec<Vec<Mint>>) -> usize {
    rref(&mut a).len()
}

pub fn determinant(mut a: Vec<Vec<Mint>>) -> Mint {
    let n = a.len();
    assert!(a.iter().all(|row| row.len() == n));
    let mut det = Mint::new(1);
    for col in 0..n {
        let pivot = (col..n).find(|&r| a[r][col].val() != 0);
        let Some(p) = pivot else {
            return Mint::new(0);
        };
        if p != col {
            a.swap(p, col);
            det = -det;
        }
        let pv = a[col][col];
        det *= pv;
        let inv = pv.inv();
        for j in col..n {
            a[col][j] *= inv;
        }
        for i in col + 1..n {
            let f = a[i][col];
            if f.val() == 0 {
                continue;
            }
            for j in col..n {
                a[i][j] = a[i][j] - f * a[col][j];
            }
        }
    }
    det
}

pub fn inverse(a: Vec<Vec<Mint>>) -> Option<Vec<Vec<Mint>>> {
    let n = a.len();
    assert!(a.iter().all(|row| row.len() == n));
    let mut aug = vec![vec![Mint::new(0); 2 * n]; n];
    for i in 0..n {
        for j in 0..n {
            aug[i][j] = a[i][j];
        }
        aug[i][n + i] = Mint::new(1);
    }
    let pivots = rref(&mut aug);
    if pivots.len() != n || pivots.iter().copied().ne(0..n) {
        return None;
    }
    Some(aug.into_iter().map(|row| row[n..].to_vec()).collect())
}

/// `a x = b` の解を 1 つ返す。自由変数は 0 に置く。解なしなら None。
pub fn solve_linear(a: Vec<Vec<Mint>>, b: Vec<Mint>) -> Option<Vec<Mint>> {
    let n = a.len();
    assert_eq!(n, b.len());
    let m = a.first().map_or(0, |row| row.len());
    assert!(a.iter().all(|row| row.len() == m));
    let mut aug = vec![vec![Mint::new(0); m + 1]; n];
    for i in 0..n {
        for j in 0..m {
            aug[i][j] = a[i][j];
        }
        aug[i][m] = b[i];
    }
    let pivots = rref(&mut aug);
    for row in &aug {
        if row[..m].iter().all(|x| x.val() == 0) && row[m].val() != 0 {
            return None;
        }
    }
    let mut x = vec![Mint::new(0); m];
    for (i, &col) in pivots.iter().enumerate() {
        if col < m {
            x[col] = aug[i][m];
        }
    }
    Some(x)
}

/// reduced row echelon form に変形し、pivot 列を返す。
pub fn rref(a: &mut [Vec<Mint>]) -> Vec<usize> {
    let h = a.len();
    let w = a.first().map_or(0, Vec::len);
    assert!(a.iter().all(|row| row.len() == w));
    let mut row = 0;
    let mut pivots = vec![];
    for col in 0..w {
        let pivot = (row..h).find(|&r| a[r][col].val() != 0);
        let Some(p) = pivot else {
            continue;
        };
        a.swap(row, p);
        let inv = a[row][col].inv();
        for j in col..w {
            a[row][j] *= inv;
        }
        for i in 0..h {
            if i == row {
                continue;
            }
            let f = a[i][col];
            if f.val() == 0 {
                continue;
            }
            for j in col..w {
                a[i][j] = a[i][j] - f * a[row][j];
            }
        }
        pivots.push(col);
        row += 1;
        if row == h {
            break;
        }
    }
    pivots
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::misc::xorshift::XorShift;

    fn mat_mul(a: &[Vec<Mint>], b: &[Vec<Mint>]) -> Vec<Vec<Mint>> {
        let n = a.len();
        let m = b[0].len();
        let k = b.len();
        let mut c = vec![vec![Mint::new(0); m]; n];
        for i in 0..n {
            for t in 0..k {
                for j in 0..m {
                    c[i][j] += a[i][t] * b[t][j];
                }
            }
        }
        c
    }

    #[test]
    fn known_values() {
        let a = vec![vec![Mint::new(1), Mint::new(2)], vec![Mint::new(3), Mint::new(4)]];
        assert_eq!(determinant(a.clone()), Mint::new(-2));
        assert_eq!(rank(a.clone()), 2);
        let inv = inverse(a.clone()).unwrap();
        assert_eq!(mat_mul(&a, &inv), vec![vec![Mint::new(1), Mint::new(0)], vec![Mint::new(0), Mint::new(1)]]);
        let x = solve_linear(a, vec![Mint::new(5), Mint::new(11)]).unwrap();
        assert_eq!(x, vec![Mint::new(1), Mint::new(2)]);
    }

    #[test]
    fn singular_and_inconsistent() {
        let a = vec![vec![Mint::new(1), Mint::new(2)], vec![Mint::new(2), Mint::new(4)]];
        assert_eq!(rank(a.clone()), 1);
        assert_eq!(determinant(a.clone()), Mint::new(0));
        assert!(inverse(a.clone()).is_none());
        assert!(solve_linear(a, vec![Mint::new(1), Mint::new(3)]).is_none());
    }

    #[test]
    fn random_inverse() {
        let mut rng = XorShift::new(9090);
        for _ in 0..100 {
            let n = 1 + rng.next_range(5) as usize;
            let a: Vec<Vec<Mint>> = (0..n)
                .map(|_| (0..n).map(|_| Mint::new(rng.next_range(20) as i64 - 10)).collect())
                .collect();
            if let Some(inv) = inverse(a.clone()) {
                let prod = mat_mul(&a, &inv);
                for i in 0..n {
                    for j in 0..n {
                        assert_eq!(prod[i][j], Mint::new((i == j) as i64));
                    }
                }
            } else {
                assert_eq!(determinant(a), Mint::new(0));
            }
        }
    }
}

