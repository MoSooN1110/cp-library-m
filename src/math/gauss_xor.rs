//! GF(2) 連立一次方程式（XOR 方程式）の解法（掃き出し法）。
//! 各式は変数ビットマスク（u64）＋右辺（0/1）で表す。
//!
//! ```
//! use cplib::math::gauss_xor::*;
//! // x0 ^ x1 = 1, x1 = 1  → x0=0, x1=1
//! let sol = solve_xor(&[(0b11, 1), (0b10, 1)], 2).unwrap();
//! assert_eq!(sol[0], false);
//! assert_eq!(sol[1], true);
//! ```

/// 方程式 `(mask, rhs)`：mask のビットが立つ変数の XOR = rhs。
/// 解の 1 つ（自由変数は 0）を返す。矛盾すれば None。
pub fn solve_xor(eqs: &[(u64, u8)], num_vars: usize) -> Option<Vec<bool>> {
    let mut rows: Vec<u128> = eqs
        .iter()
        .map(|&(m, r)| (m as u128) | ((r as u128 & 1) << 64))
        .collect();
    let mut where_pivot = vec![usize::MAX; num_vars];
    let mut r = 0usize;
    for col in 0..num_vars {
        // col をピボットに持つ行を探す
        let mut sel = None;
        for i in r..rows.len() {
            if (rows[i] >> col) & 1 == 1 {
                sel = Some(i);
                break;
            }
        }
        let Some(sel) = sel else { continue };
        rows.swap(r, sel);
        for i in 0..rows.len() {
            if i != r && (rows[i] >> col) & 1 == 1 {
                rows[i] ^= rows[r];
            }
        }
        where_pivot[col] = r;
        r += 1;
    }
    // 矛盾チェック: 0 = 1 の行
    for i in r..rows.len() {
        if (rows[i] & ((1u128 << 64) - 1)) == 0 && ((rows[i] >> 64) & 1) == 1 {
            return None;
        }
    }
    let mut ans = vec![false; num_vars];
    for col in 0..num_vars {
        if where_pivot[col] != usize::MAX {
            ans[col] = ((rows[where_pivot[col]] >> 64) & 1) == 1;
        }
    }
    Some(ans)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn brute_check() {
        let mut x: u64 = 13;
        let mut rng = || {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            x
        };
        for _ in 0..500 {
            let vars = 1 + (rng() as usize) % 6;
            let m = (rng() as usize) % 8;
            // ランダムな真の解を用意して整合方程式を作る（解ありケース）
            let truth: Vec<bool> = (0..vars).map(|_| rng() % 2 == 0).collect();
            let mut eqs = vec![];
            for _ in 0..m {
                let mask = (rng() % (1 << vars)) as u64;
                let rhs = (0..vars)
                    .filter(|&b| (mask >> b) & 1 == 1 && truth[b])
                    .count()
                    % 2;
                eqs.push((mask, rhs as u8));
            }
            let sol = solve_xor(&eqs, vars).expect("should be consistent");
            // 解が全式を満たす
            for &(mask, rhs) in &eqs {
                let got = (0..vars)
                    .filter(|&b| (mask >> b) & 1 == 1 && sol[b])
                    .count()
                    % 2;
                assert_eq!(got as u8, rhs);
            }
        }
        // 矛盾ケース: x0=0 と x0=1
        assert!(solve_xor(&[(1, 0), (1, 1)], 1).is_none());
    }
}
