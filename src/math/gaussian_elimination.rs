//! Gaussian Elimination（実数行列の掃き出し法）。
//!
//! rank 計算、RREF 変換、連立一次方程式 `Ax=b` の解判定を行う。
//!
//! ```
//! use cplib::math::gaussian_elimination::*;
//!
//! let a = vec![vec![2.0, 1.0], vec![1.0, -1.0]];
//! let b = vec![5.0, 1.0];
//! match solve_linear(a, b, 1e-9) {
//!     LinearSystem::Unique(x) => {
//!         assert!((x[0] - 2.0).abs() < 1e-9);
//!         assert!((x[1] - 1.0).abs() < 1e-9);
//!     }
//!     _ => unreachable!(),
//! }
//! ```

#[derive(Clone, Debug, PartialEq)]
pub enum LinearSystem {
    /// 一意解。
    Unique(Vec<f64>),
    /// 無限個の解。返すベクトルは自由変数を 0 にした解の一つ。
    Infinite(Vec<f64>),
    /// 解なし。
    Inconsistent,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Rref {
    pub matrix: Vec<Vec<f64>>,
    pub rank: usize,
    pub pivots: Vec<usize>,
}

/// 行列を reduced row echelon form に変換する。
pub fn rref(mut a: Vec<Vec<f64>>, eps: f64) -> Rref {
    let w = a.first().map_or(0, Vec::len);
    rref_with_pivot_cols(&mut a, eps, w)
}

fn rref_with_pivot_cols(a: &mut [Vec<f64>], eps: f64, pivot_cols: usize) -> Rref {
    let h = a.len();
    let w = a.first().map_or(0, Vec::len);
    assert!(a.iter().all(|row| row.len() == w));
    assert!(pivot_cols <= w);
    assert!(eps >= 0.0);

    let mut rank = 0usize;
    let mut pivots = Vec::new();
    for col in 0..pivot_cols {
        let mut pivot = rank;
        for row in rank..h {
            if a[pivot][col].abs() < a[row][col].abs() {
                pivot = row;
            }
        }
        if pivot >= h || a[pivot][col].abs() <= eps {
            continue;
        }
        a.swap(rank, pivot);
        let div = a[rank][col];
        for x in col..w {
            a[rank][x] /= div;
        }
        for row in 0..h {
            if row == rank {
                continue;
            }
            let factor = a[row][col];
            if factor.abs() <= eps {
                continue;
            }
            for x in col..w {
                a[row][x] -= factor * a[rank][x];
            }
        }
        for row in 0..h {
            if a[row][col].abs() <= eps {
                a[row][col] = 0.0;
            }
        }
        pivots.push(col);
        rank += 1;
        if rank == h {
            break;
        }
    }

    for row in a.iter_mut() {
        for x in row {
            if x.abs() <= eps {
                *x = 0.0;
            }
        }
    }

    Rref {
        matrix: a.to_vec(),
        rank,
        pivots,
    }
}

/// 行列の rank。
pub fn rank(a: Vec<Vec<f64>>, eps: f64) -> usize {
    rref(a, eps).rank
}

/// 連立一次方程式 `Ax=b` を解く。
pub fn solve_linear(a: Vec<Vec<f64>>, b: Vec<f64>, eps: f64) -> LinearSystem {
    let n = a.len();
    assert_eq!(n, b.len());
    let m = a.first().map_or(0, Vec::len);
    assert!(a.iter().all(|row| row.len() == m));

    let mut aug = Vec::with_capacity(n);
    for (mut row, rhs) in a.into_iter().zip(b) {
        row.push(rhs);
        aug.push(row);
    }

    let Rref {
        matrix,
        rank: _,
        pivots,
    } = rref_with_pivot_cols(&mut aug, eps, m);

    for row in &matrix {
        let all_zero = row[..m].iter().all(|x| x.abs() <= eps);
        if all_zero && row[m].abs() > eps {
            return LinearSystem::Inconsistent;
        }
    }

    let mut x = vec![0.0; m];
    for (row, &col) in pivots.iter().enumerate() {
        if col < m {
            x[col] = matrix[row][m];
        }
    }
    let coefficient_rank = pivots.iter().filter(|&&col| col < m).count();
    if coefficient_rank == m {
        LinearSystem::Unique(x)
    } else {
        LinearSystem::Infinite(x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPS: f64 = 1e-9;

    fn mat_vec(a: &[Vec<f64>], x: &[f64]) -> Vec<f64> {
        a.iter()
            .map(|row| row.iter().zip(x).map(|(a, x)| a * x).sum())
            .collect()
    }

    fn close(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-7
    }

    #[test]
    fn unique_solution() {
        let a = vec![vec![2.0, 1.0], vec![1.0, -1.0]];
        let b = vec![5.0, 1.0];
        let LinearSystem::Unique(x) = solve_linear(a.clone(), b.clone(), EPS) else {
            panic!("expected unique solution");
        };
        assert!(close(x[0], 2.0));
        assert!(close(x[1], 1.0));
        let got = mat_vec(&a, &x);
        assert!(got.iter().zip(&b).all(|(&x, &y)| close(x, y)));
    }

    #[test]
    fn inconsistent() {
        let a = vec![vec![1.0, 1.0], vec![2.0, 2.0]];
        let b = vec![1.0, 3.0];
        assert_eq!(solve_linear(a, b, EPS), LinearSystem::Inconsistent);
    }

    #[test]
    fn infinite_solution() {
        let a = vec![vec![1.0, 1.0, 1.0], vec![2.0, 2.0, 2.0]];
        let b = vec![3.0, 6.0];
        let LinearSystem::Infinite(x) = solve_linear(a.clone(), b.clone(), EPS) else {
            panic!("expected infinite solutions");
        };
        let got = mat_vec(&a, &x);
        assert!(got.iter().zip(&b).all(|(&x, &y)| close(x, y)));
    }

    #[test]
    fn rank_examples() {
        assert_eq!(rank(vec![vec![1.0, 2.0], vec![2.0, 4.0]], EPS), 1);
        assert_eq!(
            rank(
                vec![
                    vec![1.0, 2.0, 3.0],
                    vec![0.0, 1.0, 4.0],
                    vec![5.0, 6.0, 0.0],
                ],
                EPS,
            ),
            3
        );
        assert_eq!(rank(Vec::<Vec<f64>>::new(), EPS), 0);
    }

    #[test]
    fn random_square_systems() {
        let mut seed = 19260817u64;
        let mut rng = || {
            seed ^= seed << 7;
            seed ^= seed >> 9;
            seed
        };
        for _ in 0..200 {
            let n = 1 + rng() as usize % 5;
            let mut a = vec![vec![0.0; n]; n];
            for (i, row) in a.iter_mut().enumerate() {
                row[i] = 1.0;
                for x in row {
                    *x += (rng() % 7) as f64 - 3.0;
                }
            }
            let truth: Vec<f64> = (0..n).map(|_| (rng() % 11) as f64 - 5.0).collect();
            let b = mat_vec(&a, &truth);
            if let LinearSystem::Unique(x) = solve_linear(a, b, EPS) {
                for (got, expected) in x.iter().zip(&truth) {
                    assert!(close(*got, *expected), "{got} != {expected}");
                }
            }
        }
    }
}
