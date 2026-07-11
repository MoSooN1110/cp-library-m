//! Walsh–Hadamard 変換と XOR/AND/OR 畳み込み（i64、オーバーフローは呼び出し側の責任）。
//!
//! 各畳み込みは c[i op j] += a[i]·b[j]。結果長は max(|a|,|b|) 以上の最小 2 冪。
//! 値の大きさは n·max|a|·max|b| が i64 に収まる範囲で使うこと。
//!
//! ```
//! use cplib::math::hadamard::*;
//! let c = xor_convolution(&[1, 2, 3, 4], &[5, 6, 7, 8]);
//! assert_eq!(c, vec![70, 68, 62, 60]);
//! let c = and_convolution(&[1, 2], &[3, 4]);
//! assert_eq!(c, vec![1 * 3 + 1 * 4 + 2 * 3, 2 * 4]);
//! ```

/// Walsh–Hadamard 変換（in-place）。長さは 2 冪。
/// invert=true は逆変換（len で割る。順変換の結果に対してのみ整数で正確）。
pub fn walsh_hadamard(a: &mut [i64], invert: bool) {
    let n = a.len();
    assert!(n.is_power_of_two(), "length must be a power of two");
    let mut step = 1;
    while step < n {
        for i in (0..n).step_by(step * 2) {
            for j in i..i + step {
                let u = a[j];
                let v = a[j + step];
                a[j] = u + v;
                a[j + step] = u - v;
            }
        }
        step *= 2;
    }
    if invert {
        for x in a.iter_mut() {
            *x /= n as i64;
        }
    }
}

fn pad_pow2(a: &[i64], b: &[i64]) -> Option<(Vec<i64>, Vec<i64>)> {
    if a.is_empty() || b.is_empty() {
        return None;
    }
    let n = a.len().max(b.len()).next_power_of_two();
    let mut fa = a.to_vec();
    let mut fb = b.to_vec();
    fa.resize(n, 0);
    fb.resize(n, 0);
    Some((fa, fb))
}

/// XOR 畳み込み c[i^j] += a[i]·b[j]。O(n log n)。
pub fn xor_convolution(a: &[i64], b: &[i64]) -> Vec<i64> {
    let Some((mut fa, mut fb)) = pad_pow2(a, b) else {
        return vec![];
    };
    walsh_hadamard(&mut fa, false);
    walsh_hadamard(&mut fb, false);
    for (x, y) in fa.iter_mut().zip(&fb) {
        *x *= y;
    }
    walsh_hadamard(&mut fa, true);
    fa
}

// 上位集合和ゼータ変換（AND 用）とその逆
fn superset_zeta(a: &mut [i64], invert: bool) {
    let n = a.len();
    let mut bit = 1;
    while bit < n {
        for i in 0..n {
            if i & bit == 0 {
                if invert {
                    a[i] -= a[i | bit];
                } else {
                    a[i] += a[i | bit];
                }
            }
        }
        bit <<= 1;
    }
}

// 部分集合和ゼータ変換（OR 用）とその逆
fn subset_zeta(a: &mut [i64], invert: bool) {
    let n = a.len();
    let mut bit = 1;
    while bit < n {
        for i in 0..n {
            if i & bit != 0 {
                if invert {
                    a[i] -= a[i ^ bit];
                } else {
                    a[i] += a[i ^ bit];
                }
            }
        }
        bit <<= 1;
    }
}

/// AND 畳み込み c[i&j] += a[i]·b[j]。O(n log n)。
pub fn and_convolution(a: &[i64], b: &[i64]) -> Vec<i64> {
    let Some((mut fa, mut fb)) = pad_pow2(a, b) else {
        return vec![];
    };
    superset_zeta(&mut fa, false);
    superset_zeta(&mut fb, false);
    for (x, y) in fa.iter_mut().zip(&fb) {
        *x *= y;
    }
    superset_zeta(&mut fa, true);
    fa
}

/// OR 畳み込み c[i|j] += a[i]·b[j]。O(n log n)。
pub fn or_convolution(a: &[i64], b: &[i64]) -> Vec<i64> {
    let Some((mut fa, mut fb)) = pad_pow2(a, b) else {
        return vec![];
    };
    subset_zeta(&mut fa, false);
    subset_zeta(&mut fb, false);
    for (x, y) in fa.iter_mut().zip(&fb) {
        *x *= y;
    }
    subset_zeta(&mut fa, true);
    fa
}

#[cfg(test)]
mod tests {
    use super::*;

    fn naive(a: &[i64], b: &[i64], op: fn(usize, usize) -> usize) -> Vec<i64> {
        if a.is_empty() || b.is_empty() {
            return vec![];
        }
        let n = a.len().max(b.len()).next_power_of_two();
        let mut c = vec![0i64; n];
        for (i, &x) in a.iter().enumerate() {
            for (j, &y) in b.iter().enumerate() {
                c[op(i, j)] += x * y;
            }
        }
        c
    }

    #[test]
    fn known_small() {
        assert_eq!(
            xor_convolution(&[1, 2, 3, 4], &[5, 6, 7, 8]),
            vec![70, 68, 62, 60]
        );
        assert_eq!(and_convolution(&[1, 2], &[3, 4]), vec![13, 8]);
        assert_eq!(or_convolution(&[1, 2], &[3, 4]), vec![3, 18]);
        assert!(xor_convolution(&[], &[1]).is_empty());
        assert_eq!(xor_convolution(&[3], &[5]), vec![15]);
    }

    #[test]
    fn wht_roundtrip() {
        let mut a = vec![3, -1, 4, 1, -5, 9, 2, 6];
        let orig = a.clone();
        walsh_hadamard(&mut a, false);
        walsh_hadamard(&mut a, true);
        assert_eq!(a, orig);
    }

    #[test]
    fn random_vs_naive() {
        let mut x: u64 = 271828;
        let mut rng = move || {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            x
        };
        for _ in 0..100 {
            let n = 1 + (rng() % 33) as usize;
            let m = 1 + (rng() % 33) as usize;
            // 負数も含む
            let a: Vec<i64> = (0..n).map(|_| (rng() % 201) as i64 - 100).collect();
            let b: Vec<i64> = (0..m).map(|_| (rng() % 201) as i64 - 100).collect();
            assert_eq!(xor_convolution(&a, &b), naive(&a, &b, |i, j| i ^ j), "xor");
            assert_eq!(and_convolution(&a, &b), naive(&a, &b, |i, j| i & j), "and");
            assert_eq!(or_convolution(&a, &b), naive(&a, &b, |i, j| i | j), "or");
        }
    }
}
