//! グリッドの回転・転置・反転と、回転（+平行移動）による合同判定。
//!
//! 回転はすべて時計回り。非正方形グリッドにも対応（h×w → w×h）。
//!
//! ```
//! use cplib::algo::grid_transform::*;
//! let g = vec![vec![1, 2], vec![3, 4]];
//! assert_eq!(rotate90(&g), vec![vec![3, 1], vec![4, 2]]);
//! assert_eq!(transpose(&g), vec![vec![1, 3], vec![2, 4]]);
//! // '#' の集合として回転+平行移動で一致するか
//! let a: Vec<Vec<char>> = vec!["##".chars().collect(), "#.".chars().collect()];
//! let b: Vec<Vec<char>> = vec![".#".chars().collect(), "##".chars().collect()];
//! assert!(congruent_under_rotation(&a, &b, &'#'));
//! ```

/// 転置（h×w → w×h）。
pub fn transpose<T: Clone>(g: &[Vec<T>]) -> Vec<Vec<T>> {
    if g.is_empty() || g[0].is_empty() {
        return vec![];
    }
    let (h, w) = (g.len(), g[0].len());
    (0..w)
        .map(|j| (0..h).map(|i| g[i][j].clone()).collect())
        .collect()
}

/// 時計回り 90° 回転（h×w → w×h）。
pub fn rotate90<T: Clone>(g: &[Vec<T>]) -> Vec<Vec<T>> {
    if g.is_empty() || g[0].is_empty() {
        return vec![];
    }
    let (h, w) = (g.len(), g[0].len());
    (0..w)
        .map(|j| (0..h).rev().map(|i| g[i][j].clone()).collect())
        .collect()
}

/// 180° 回転。
pub fn rotate180<T: Clone>(g: &[Vec<T>]) -> Vec<Vec<T>> {
    g.iter()
        .rev()
        .map(|row| row.iter().rev().cloned().collect())
        .collect()
}

/// 時計回り 270°（反時計回り 90°）回転（h×w → w×h）。
pub fn rotate270<T: Clone>(g: &[Vec<T>]) -> Vec<Vec<T>> {
    if g.is_empty() || g[0].is_empty() {
        return vec![];
    }
    let (h, w) = (g.len(), g[0].len());
    (0..w)
        .rev()
        .map(|j| (0..h).map(|i| g[i][j].clone()).collect())
        .collect()
}

/// 左右反転（各行を逆順に）。
pub fn flip_horizontal<T: Clone>(g: &[Vec<T>]) -> Vec<Vec<T>> {
    g.iter()
        .map(|row| row.iter().rev().cloned().collect())
        .collect()
}

/// 上下反転（行の並びを逆順に）。
pub fn flip_vertical<T: Clone>(g: &[Vec<T>]) -> Vec<Vec<T>> {
    g.iter().rev().cloned().collect()
}

/// グリッド中の mark に一致するセルの (行, 列) 一覧。
pub fn marked_cells<T: PartialEq>(g: &[Vec<T>], mark: &T) -> Vec<(i64, i64)> {
    let mut cells = vec![];
    for (i, row) in g.iter().enumerate() {
        for (j, x) in row.iter().enumerate() {
            if x == mark {
                cells.push((i as i64, j as i64));
            }
        }
    }
    cells
}

/// セル集合を平行移動して最小の行・列を 0 に揃え、ソート・重複除去して返す。
pub fn normalize_cells(cells: &[(i64, i64)]) -> Vec<(i64, i64)> {
    if cells.is_empty() {
        return vec![];
    }
    let min_r = cells.iter().map(|&(r, _)| r).min().unwrap();
    let min_c = cells.iter().map(|&(_, c)| c).min().unwrap();
    let mut out: Vec<(i64, i64)> = cells.iter().map(|&(r, c)| (r - min_r, c - min_c)).collect();
    out.sort();
    out.dedup();
    out
}

/// mark セルの集合として、b を 0°/90°/180°/270° 回転＋平行移動したとき
/// a と一致するものがあるか。グリッドの外形サイズは無視される。
pub fn congruent_under_rotation<T: PartialEq>(a: &[Vec<T>], b: &[Vec<T>], mark: &T) -> bool {
    let pa = normalize_cells(&marked_cells(a, mark));
    let mut pb = marked_cells(b, mark);
    for _ in 0..4 {
        if normalize_cells(&pb) == pa {
            return true;
        }
        // 時計回り 90°: (r, c) -> (c, -r)
        pb = pb.iter().map(|&(r, c)| (c, -r)).collect();
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_transforms() {
        let g = vec![vec![1, 2, 3], vec![4, 5, 6]]; // 2x3
        assert_eq!(rotate90(&g), vec![vec![4, 1], vec![5, 2], vec![6, 3]]);
        assert_eq!(rotate180(&g), vec![vec![6, 5, 4], vec![3, 2, 1]]);
        assert_eq!(rotate270(&g), vec![vec![3, 6], vec![2, 5], vec![1, 4]]);
        assert_eq!(transpose(&g), vec![vec![1, 4], vec![2, 5], vec![3, 6]]);
        assert_eq!(flip_horizontal(&g), vec![vec![3, 2, 1], vec![6, 5, 4]]);
        assert_eq!(flip_vertical(&g), vec![vec![4, 5, 6], vec![1, 2, 3]]);
        let e: Vec<Vec<i32>> = vec![];
        assert!(rotate90(&e).is_empty());
        assert!(transpose(&e).is_empty());
    }

    #[test]
    fn involutions_random() {
        let mut x: u64 = 173205080756;
        let mut rng = move || {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            x
        };
        for _ in 0..30 {
            let h = 1 + (rng() as usize) % 6;
            let w = 1 + (rng() as usize) % 6;
            let g: Vec<Vec<u32>> = (0..h)
                .map(|_| (0..w).map(|_| (rng() % 100) as u32).collect())
                .collect();
            assert_eq!(rotate90(&rotate90(&rotate90(&rotate90(&g)))), g);
            assert_eq!(rotate180(&rotate180(&g)), g);
            assert_eq!(rotate270(&rotate90(&g)), g);
            assert_eq!(rotate90(&rotate90(&g)), rotate180(&g));
            assert_eq!(transpose(&transpose(&g)), g);
            assert_eq!(flip_horizontal(&flip_horizontal(&g)), g);
            assert_eq!(flip_vertical(&flip_vertical(&g)), g);
            assert_eq!(rotate90(&g), flip_horizontal(&transpose(&g)));
        }
    }

    #[test]
    fn congruence_known() {
        // S テトロミノと Z テトロミノは回転だけでは一致しない（鏡像）
        let s: Vec<Vec<char>> = vec![".##".chars().collect(), "##.".chars().collect()];
        let z: Vec<Vec<char>> = vec!["##.".chars().collect(), ".##".chars().collect()];
        assert!(!congruent_under_rotation(&s, &z, &'#'));
        assert!(congruent_under_rotation(&s, &s, &'#'));
        // S を 90° 回転して平行移動（大きいグリッドに埋め込み）
        let s_rot: Vec<Vec<char>> = vec![
            "....".chars().collect(),
            ".#..".chars().collect(),
            ".##.".chars().collect(),
            "..#.".chars().collect(),
        ];
        assert!(congruent_under_rotation(&s, &s_rot, &'#'));
        // マーク数が違えば不一致
        let one: Vec<Vec<char>> = vec!["#".chars().collect()];
        assert!(!congruent_under_rotation(&s, &one, &'#'));
        // 両方空なら一致
        let empty: Vec<Vec<char>> = vec!["..".chars().collect()];
        assert!(congruent_under_rotation(&empty, &empty, &'#'));
    }

    #[test]
    fn congruence_random() {
        let mut x: u64 = 223606797749;
        let mut rng = move || {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            x
        };
        for _ in 0..50 {
            let h = 1 + (rng() as usize) % 5;
            let w = 1 + (rng() as usize) % 5;
            let mut a = vec![vec![false; w]; h];
            let mut any = false;
            for row in a.iter_mut() {
                for cell in row.iter_mut() {
                    *cell = rng() % 3 == 0;
                    any |= *cell;
                }
            }
            if !any {
                a[0][0] = true;
            }
            // ランダム回転 + 大きいグリッドへ平行移動して埋め込み
            let mut b = a.clone();
            for _ in 0..(rng() % 4) {
                b = rotate90(&b);
            }
            let (bh, bw) = (b.len(), b[0].len());
            let (or, oc) = ((rng() as usize) % 3, (rng() as usize) % 3);
            let mut big = vec![vec![false; bw + 4]; bh + 4];
            for i in 0..bh {
                for j in 0..bw {
                    big[i + or][j + oc] = b[i][j];
                }
            }
            assert!(congruent_under_rotation(&a, &big, &true));
            // セルを 1 つ増やすと不一致（個数が違う）
            let mut extra = big.clone();
            'find: for i in 0..extra.len() {
                for j in 0..extra[0].len() {
                    if !extra[i][j] {
                        extra[i][j] = true;
                        break 'find;
                    }
                }
            }
            assert!(!congruent_under_rotation(&a, &extra, &true));
        }
    }
}
