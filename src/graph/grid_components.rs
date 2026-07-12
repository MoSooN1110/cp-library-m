//! グリッド上の 4 近傍連結成分分解。
//!
//! `passable[r][c] == true` のセルだけを対象に、連結成分番号を振る。
//!
//! ```
//! use cplib::graph::grid_components::*;
//!
//! let grid = vec![
//!     vec![true, true, false],
//!     vec![false, true, false],
//!     vec![true, false, true],
//! ];
//! let comp = grid_components(&grid);
//! assert_eq!(comp.count, 3);
//! assert!(is_grid_connected(&grid) == false);
//! assert!(char_grid_connected(&[vec!['o', 'o'], vec!['x', 'o']], 'o'));
//! ```

use std::collections::VecDeque;

const DIR4: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GridComponents {
    pub count: usize,
    pub id: Vec<Vec<Option<usize>>>,
    pub cells: Vec<Vec<(usize, usize)>>,
}

pub fn grid_components(passable: &[Vec<bool>]) -> GridComponents {
    assert_rectangular(passable);
    let h = passable.len();
    let w = passable.first().map_or(0, Vec::len);
    let mut id = vec![vec![None; w]; h];
    let mut cells = Vec::new();
    for r in 0..h {
        for c in 0..w {
            if !passable[r][c] || id[r][c].is_some() {
                continue;
            }
            let comp_id = cells.len();
            let mut comp = Vec::new();
            let mut q = VecDeque::new();
            id[r][c] = Some(comp_id);
            q.push_back((r, c));
            while let Some((vr, vc)) = q.pop_front() {
                comp.push((vr, vc));
                for (dr, dc) in DIR4 {
                    let Some(nr) = vr.checked_add_signed(dr) else {
                        continue;
                    };
                    let Some(nc) = vc.checked_add_signed(dc) else {
                        continue;
                    };
                    if nr < h && nc < w && passable[nr][nc] && id[nr][nc].is_none() {
                        id[nr][nc] = Some(comp_id);
                        q.push_back((nr, nc));
                    }
                }
            }
            cells.push(comp);
        }
    }
    GridComponents {
        count: cells.len(),
        id,
        cells,
    }
}

pub fn grid_component_count(passable: &[Vec<bool>]) -> usize {
    grid_components(passable).count
}

pub fn is_grid_connected(passable: &[Vec<bool>]) -> bool {
    grid_component_count(passable) <= 1
}

pub fn char_grid_components(grid: &[Vec<char>], mark: char) -> GridComponents {
    let passable: Vec<Vec<bool>> = grid
        .iter()
        .map(|row| row.iter().map(|&c| c == mark).collect())
        .collect();
    grid_components(&passable)
}

pub fn char_grid_connected(grid: &[Vec<char>], mark: char) -> bool {
    char_grid_components(grid, mark).count <= 1
}

fn assert_rectangular<T>(grid: &[Vec<T>]) {
    let Some(first) = grid.first() else {
        return;
    };
    assert!(grid.iter().all(|row| row.len() == first.len()));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn brute_count(passable: &[Vec<bool>]) -> usize {
        let h = passable.len();
        let w = passable.first().map_or(0, Vec::len);
        let mut seen = vec![vec![false; w]; h];
        let mut count = 0;
        for r in 0..h {
            for c in 0..w {
                if !passable[r][c] || seen[r][c] {
                    continue;
                }
                count += 1;
                let mut q = VecDeque::new();
                seen[r][c] = true;
                q.push_back((r, c));
                while let Some((vr, vc)) = q.pop_front() {
                    for (dr, dc) in DIR4 {
                        let Some(nr) = vr.checked_add_signed(dr) else {
                            continue;
                        };
                        let Some(nc) = vc.checked_add_signed(dc) else {
                            continue;
                        };
                        if nr < h && nc < w && passable[nr][nc] && !seen[nr][nc] {
                            seen[nr][nc] = true;
                            q.push_back((nr, nc));
                        }
                    }
                }
            }
        }
        count
    }

    #[test]
    fn known_components() {
        let grid = vec![
            vec![true, true, false],
            vec![false, true, false],
            vec![true, false, true],
        ];
        let comp = grid_components(&grid);
        assert_eq!(comp.count, 3);
        assert_eq!(comp.id[0][0], comp.id[0][1]);
        assert_eq!(comp.id[0][1], comp.id[1][1]);
        assert_ne!(comp.id[0][0], comp.id[2][0]);
        assert_ne!(comp.id[2][0], comp.id[2][2]);
        assert!(!is_grid_connected(&grid));
    }

    #[test]
    fn empty_and_all_blocked_are_connected() {
        assert!(is_grid_connected(&[]));
        assert!(is_grid_connected(&[vec![false, false], vec![false, false]]));
        assert_eq!(
            grid_components(&[vec![false, false], vec![false, false]]).count,
            0
        );
    }

    #[test]
    fn char_grid() {
        let connected = vec![vec!['o', 'o'], vec!['x', 'o']];
        let disconnected = vec![vec!['o', 'x', 'o']];
        assert!(char_grid_connected(&connected, 'o'));
        assert!(!char_grid_connected(&disconnected, 'o'));
        assert_eq!(char_grid_components(&disconnected, 'o').count, 2);
    }

    #[test]
    fn random_matches_brute() {
        let mut seed = 1357913579u64;
        let mut rng = || {
            seed ^= seed << 7;
            seed ^= seed >> 9;
            seed
        };
        for _ in 0..500 {
            let h = rng() as usize % 8;
            let w = rng() as usize % 8;
            let mut grid = vec![vec![false; w]; h];
            for row in &mut grid {
                for cell in row {
                    *cell = rng() % 100 < 45;
                }
            }
            let got = grid_components(&grid);
            assert_eq!(got.count, brute_count(&grid), "{grid:?}");
            for (comp_id, cells) in got.cells.iter().enumerate() {
                for &(r, c) in cells {
                    assert!(grid[r][c]);
                    assert_eq!(got.id[r][c], Some(comp_id));
                }
            }
        }
    }

    #[test]
    #[should_panic]
    fn non_rectangular_panics() {
        let _ = grid_components(&[vec![true], vec![true, false]]);
    }
}
