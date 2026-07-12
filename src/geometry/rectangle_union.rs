//! 軸平行矩形の和集合面積。
//!
//! 半開矩形 `[x1, x2) * [y1, y2)` の列に対し、座標圧縮付きスイープラインで
//! union area を求める。端点は逆順でもよく、幅または高さが 0 の矩形は無視する。
//!
//! ```
//! use cplib::geometry::rectangle_union::*;
//!
//! let rects = [
//!     Rectangle::new(0, 0, 3, 2),
//!     Rectangle::new(1, 1, 4, 3),
//! ];
//! assert_eq!(rectangle_union_area(&rects), 10);
//! ```

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rectangle {
    pub x1: i64,
    pub y1: i64,
    pub x2: i64,
    pub y2: i64,
}

impl Rectangle {
    pub fn new(x1: i64, y1: i64, x2: i64, y2: i64) -> Self {
        let (x1, x2) = if x1 <= x2 { (x1, x2) } else { (x2, x1) };
        let (y1, y2) = if y1 <= y2 { (y1, y2) } else { (y2, y1) };
        Self { x1, y1, x2, y2 }
    }

    pub fn is_empty(self) -> bool {
        self.x1 == self.x2 || self.y1 == self.y2
    }
}

pub fn rectangle_union_area(rectangles: &[Rectangle]) -> i128 {
    let rects: Vec<Rectangle> = rectangles
        .iter()
        .copied()
        .filter(|rect| !rect.is_empty())
        .collect();
    if rects.is_empty() {
        return 0;
    }

    let mut ys = Vec::with_capacity(rects.len() * 2);
    for rect in &rects {
        ys.push(rect.y1);
        ys.push(rect.y2);
    }
    ys.sort_unstable();
    ys.dedup();

    let mut events = Vec::with_capacity(rects.len() * 2);
    for rect in rects {
        let y1 = ys.binary_search(&rect.y1).unwrap();
        let y2 = ys.binary_search(&rect.y2).unwrap();
        events.push((rect.x1, y1, y2, 1i32));
        events.push((rect.x2, y1, y2, -1i32));
    }
    events.sort_unstable_by_key(|event| event.0);

    let mut seg = CoverSegTree::new(ys);
    let mut area = 0i128;
    let mut prev_x = events[0].0;
    let mut i = 0;
    while i < events.len() {
        let x = events[i].0;
        area += seg.covered_len() * (x as i128 - prev_x as i128);
        while i < events.len() && events[i].0 == x {
            let (_, y1, y2, delta) = events[i];
            seg.add(y1, y2, delta);
            i += 1;
        }
        prev_x = x;
    }
    area
}

pub fn union_area(rectangles: &[Rectangle]) -> i128 {
    rectangle_union_area(rectangles)
}

struct CoverSegTree {
    ys: Vec<i64>,
    cover: Vec<i32>,
    len: Vec<i128>,
}

impl CoverSegTree {
    fn new(ys: Vec<i64>) -> Self {
        let n = ys.len().saturating_sub(1).max(1);
        Self {
            ys,
            cover: vec![0; 4 * n],
            len: vec![0; 4 * n],
        }
    }

    fn covered_len(&self) -> i128 {
        self.len[1]
    }

    fn add(&mut self, l: usize, r: usize, delta: i32) {
        if l < r {
            self.add_rec(l, r, delta, 1, 0, self.ys.len() - 1);
        }
    }

    fn add_rec(&mut self, ql: usize, qr: usize, delta: i32, k: usize, l: usize, r: usize) {
        if r <= ql || qr <= l {
            return;
        }
        if ql <= l && r <= qr {
            self.cover[k] += delta;
            self.pull(k, l, r);
            return;
        }
        let mid = (l + r) / 2;
        self.add_rec(ql, qr, delta, 2 * k, l, mid);
        self.add_rec(ql, qr, delta, 2 * k + 1, mid, r);
        self.pull(k, l, r);
    }

    fn pull(&mut self, k: usize, l: usize, r: usize) {
        if self.cover[k] > 0 {
            self.len[k] = self.ys[r] as i128 - self.ys[l] as i128;
        } else if r - l == 1 {
            self.len[k] = 0;
        } else {
            self.len[k] = self.len[2 * k] + self.len[2 * k + 1];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn brute(rects: &[Rectangle]) -> i128 {
        let mut area = 0;
        for x in -5..=5 {
            for y in -5..=5 {
                if rects
                    .iter()
                    .any(|r| r.x1 <= x && x < r.x2 && r.y1 <= y && y < r.y2)
                {
                    area += 1;
                }
            }
        }
        area
    }

    #[test]
    fn sample_case() {
        let rects = [Rectangle::new(0, 0, 3, 2), Rectangle::new(1, 1, 4, 3)];
        assert_eq!(rectangle_union_area(&rects), 10);
        assert_eq!(union_area(&rects), 10);
    }

    #[test]
    fn handles_empty_and_reversed_rectangles() {
        let rects = [
            Rectangle::new(3, 3, 1, 1),
            Rectangle::new(0, 0, 0, 5),
            Rectangle::new(-1, -1, 1, 1),
        ];
        assert_eq!(rectangle_union_area(&rects), 8);
    }

    #[test]
    fn random_small_matches_brute() {
        let mut seed = 246813579u64;
        let mut next_u64 = || {
            seed ^= seed << 7;
            seed ^= seed >> 9;
            seed
        };
        for _ in 0..1_000 {
            let n = (next_u64() % 7) as usize;
            let mut rects = Vec::with_capacity(n);
            for _ in 0..n {
                let x1 = (next_u64() % 11) as i64 - 5;
                let y1 = (next_u64() % 11) as i64 - 5;
                let x2 = (next_u64() % 11) as i64 - 5;
                let y2 = (next_u64() % 11) as i64 - 5;
                rects.push(Rectangle::new(x1, y1, x2, y2));
            }
            assert_eq!(rectangle_union_area(&rects), brute(&rects), "{rects:?}");
        }
    }
}
