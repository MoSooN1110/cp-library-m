//! グリッド上の 4 近傍 BFS / 01 BFS。
//!
//! ```
//! use cplib::graph::grid_bfs::*;
//!
//! let passable = vec![
//!     vec![true, true, false],
//!     vec![false, true, true],
//! ];
//! let d = grid_bfs(&passable, (0, 0));
//! assert_eq!(d[1][2], 3);
//! assert_eq!(d[1][0], INF);
//! ```

use std::collections::VecDeque;

pub const INF: u32 = u32::MAX;
pub const DIR4: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

pub fn height<T>(grid: &[Vec<T>]) -> usize {
    grid.len()
}

pub fn width<T>(grid: &[Vec<T>]) -> usize {
    grid.first().map_or(0, Vec::len)
}

pub fn in_bounds<T>(grid: &[Vec<T>], r: usize, c: usize) -> bool {
    r < height(grid) && c < width(grid)
}

fn assert_rectangular<T>(grid: &[Vec<T>]) {
    let w = width(grid);
    assert!(grid.iter().all(|row| row.len() == w));
}

fn neighbor(
    grid_h: usize,
    grid_w: usize,
    r: usize,
    c: usize,
    dr: isize,
    dc: isize,
) -> Option<(usize, usize)> {
    let nr = r.checked_add_signed(dr)?;
    let nc = c.checked_add_signed(dc)?;
    if nr < grid_h && nc < grid_w {
        Some((nr, nc))
    } else {
        None
    }
}

/// `passable[r][c] == true` のマスだけを通れる単一始点 BFS。
pub fn grid_bfs(passable: &[Vec<bool>], start: (usize, usize)) -> Vec<Vec<u32>> {
    grid_multi_source_bfs(passable, [start])
}

/// `passable[r][c] == true` のマスだけを通れる多始点 BFS。
pub fn grid_multi_source_bfs<I>(passable: &[Vec<bool>], starts: I) -> Vec<Vec<u32>>
where
    I: IntoIterator<Item = (usize, usize)>,
{
    assert_rectangular(passable);
    let h = height(passable);
    let w = width(passable);
    let mut dist = vec![vec![INF; w]; h];
    let mut q = VecDeque::new();
    for (r, c) in starts {
        assert!(in_bounds(passable, r, c));
        if passable[r][c] && dist[r][c] == INF {
            dist[r][c] = 0;
            q.push_back((r, c));
        }
    }
    while let Some((r, c)) = q.pop_front() {
        for (dr, dc) in DIR4 {
            if let Some((nr, nc)) = neighbor(h, w, r, c, dr, dc) {
                if passable[nr][nc] && dist[nr][nc] == INF {
                    dist[nr][nc] = dist[r][c] + 1;
                    q.push_back((nr, nc));
                }
            }
        }
    }
    dist
}

/// 進入コストが 0/1 のグリッド 01 BFS。
///
/// `cost[r][c]` はそのマスへ入るコスト。`None` は通行不能。
/// 始点の距離は、始点マスのコストによらず 0。
pub fn grid_zero_one_bfs(cost: &[Vec<Option<u8>>], start: (usize, usize)) -> Vec<Vec<u32>> {
    assert_rectangular(cost);
    let h = height(cost);
    let w = width(cost);
    assert!(in_bounds(cost, start.0, start.1));
    let mut dist = vec![vec![INF; w]; h];
    let mut dq = VecDeque::new();
    if cost[start.0][start.1].is_none() {
        return dist;
    }
    dist[start.0][start.1] = 0;
    dq.push_back(start);
    while let Some((r, c)) = dq.pop_front() {
        for (dr, dc) in DIR4 {
            let Some((nr, nc)) = neighbor(h, w, r, c, dr, dc) else {
                continue;
            };
            let Some(w01) = cost[nr][nc] else {
                continue;
            };
            assert!(w01 <= 1);
            let nd = dist[r][c] + w01 as u32;
            if nd < dist[nr][nc] {
                dist[nr][nc] = nd;
                if w01 == 0 {
                    dq.push_front((nr, nc));
                } else {
                    dq.push_back((nr, nc));
                }
            }
        }
    }
    dist
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_bfs() {
        let passable = vec![
            vec![true, true, false],
            vec![false, true, true],
            vec![true, true, false],
        ];
        let d = grid_bfs(&passable, (0, 0));
        assert_eq!(d[0][0], 0);
        assert_eq!(d[1][2], 3);
        assert_eq!(d[2][0], 4);
        assert_eq!(d[0][2], INF);
    }

    #[test]
    fn multi_source() {
        let passable = vec![vec![true; 5]];
        let d = grid_multi_source_bfs(&passable, [(0, 0), (0, 4)]);
        assert_eq!(d[0], vec![0, 1, 2, 1, 0]);
    }

    #[test]
    fn zero_one_bfs() {
        let cost = vec![
            vec![Some(0), Some(1), Some(1), Some(1)],
            vec![Some(0), None, Some(0), Some(1)],
            vec![Some(0), Some(0), Some(0), Some(1)],
        ];
        let d = grid_zero_one_bfs(&cost, (0, 0));
        assert_eq!(d[0][0], 0);
        assert_eq!(d[0][1], 1);
        assert_eq!(d[2][2], 0);
        assert_eq!(d[1][2], 0);
        assert_eq!(d[0][3], 2);
    }

    #[test]
    fn random_bfs_matches_bruteforce_relaxation() {
        let mut seed = 42424242u64;
        let mut rng = || {
            seed ^= seed << 7;
            seed ^= seed >> 9;
            seed
        };
        for _ in 0..200 {
            let h = 1 + rng() as usize % 8;
            let w = 1 + rng() as usize % 8;
            let mut passable = vec![vec![false; w]; h];
            for row in &mut passable {
                for x in row {
                    *x = rng() % 100 < 70;
                }
            }
            let s = (rng() as usize % h, rng() as usize % w);
            passable[s.0][s.1] = true;
            let got = grid_bfs(&passable, s);
            let mut expected = vec![vec![INF; w]; h];
            expected[s.0][s.1] = 0;
            for _ in 0..h * w {
                let mut changed = false;
                for r in 0..h {
                    for c in 0..w {
                        if !passable[r][c] || expected[r][c] == INF {
                            continue;
                        }
                        for (dr, dc) in DIR4 {
                            if let Some((nr, nc)) = neighbor(h, w, r, c, dr, dc) {
                                if passable[nr][nc] && expected[r][c] + 1 < expected[nr][nc] {
                                    expected[nr][nc] = expected[r][c] + 1;
                                    changed = true;
                                }
                            }
                        }
                    }
                }
                if !changed {
                    break;
                }
            }
            assert_eq!(got, expected);
        }
    }
}
