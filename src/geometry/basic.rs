//! 整数座標の基本幾何。
//!
//! `Point` の加減算、内積・外積、三点の位置関係、線分交差を exact に扱う。
//!
//! ```
//! use cplib::geometry::basic::*;
//!
//! let a = Point::new(0, 0);
//! let b = Point::new(4, 0);
//! let c = Point::new(2, -1);
//! let d = Point::new(2, 3);
//! assert_eq!(cross(b - a, d - c), 16);
//! assert!(segments_intersect(a, b, c, d));
//! assert_eq!(ccw(a, b, Point::new(2, 0)), Ccw::OnSegment);
//! ```

use std::ops::{Add, AddAssign, Neg, Sub, SubAssign};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub fn dot(self, rhs: Self) -> i128 {
        dot(self, rhs)
    }

    pub fn cross(self, rhs: Self) -> i128 {
        cross(self, rhs)
    }

    pub fn norm2(self) -> i128 {
        dot(self, self)
    }

    pub fn dist2(self, rhs: Self) -> i128 {
        (self - rhs).norm2()
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Neg for Point {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y)
    }
}

pub fn dot(a: Point, b: Point) -> i128 {
    a.x as i128 * b.x as i128 + a.y as i128 * b.y as i128
}

pub fn cross(a: Point, b: Point) -> i128 {
    a.x as i128 * b.y as i128 - a.y as i128 * b.x as i128
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccw {
    CounterClockwise,
    Clockwise,
    OnlineBack,
    OnlineFront,
    OnSegment,
}

/// 有向線分 a->b に対する点 c の位置関係。
pub fn ccw(a: Point, b: Point, c: Point) -> Ccw {
    let ab = b - a;
    let ac = c - a;
    let cr = cross(ab, ac);
    if cr > 0 {
        Ccw::CounterClockwise
    } else if cr < 0 {
        Ccw::Clockwise
    } else if dot(ab, ac) < 0 {
        Ccw::OnlineBack
    } else if ab.norm2() < ac.norm2() {
        Ccw::OnlineFront
    } else {
        Ccw::OnSegment
    }
}

pub fn on_segment(a: Point, b: Point, p: Point) -> bool {
    ccw(a, b, p) == Ccw::OnSegment
}

/// 閉線分 ab と cd が共有点を持つか。
pub fn segments_intersect(a: Point, b: Point, c: Point, d: Point) -> bool {
    if a == b {
        return on_segment(c, d, a);
    }
    if c == d {
        return on_segment(a, b, c);
    }
    let ab_c = cross(b - a, c - a);
    let ab_d = cross(b - a, d - a);
    let cd_a = cross(d - c, a - c);
    let cd_b = cross(d - c, b - c);
    if ab_c == 0 && ab_d == 0 {
        return ranges_overlap(a.x, b.x, c.x, d.x) && ranges_overlap(a.y, b.y, c.y, d.y);
    }
    ab_c.signum() * ab_d.signum() <= 0 && cd_a.signum() * cd_b.signum() <= 0
}

fn ranges_overlap(a: i64, b: i64, c: i64, d: i64) -> bool {
    let (ab_l, ab_r) = if a <= b { (a, b) } else { (b, a) };
    let (cd_l, cd_r) = if c <= d { (c, d) } else { (d, c) };
    ab_l <= cd_r && cd_l <= ab_r
}

/// 多角形の符号付き面積の 2 倍。
pub fn signed_area2(poly: &[Point]) -> i128 {
    let n = poly.len();
    if n < 3 {
        return 0;
    }
    (0..n).map(|i| cross(poly[i], poly[(i + 1) % n])).sum()
}

/// 多角形の面積の 2 倍。
pub fn area2(poly: &[Point]) -> i128 {
    signed_area2(poly).abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(x: i64, y: i64) -> Point {
        Point::new(x, y)
    }

    fn brute_segments_intersect(a: Point, b: Point, c: Point, d: Point) -> bool {
        if a == b {
            return on_segment(c, d, a);
        }
        if c == d {
            return on_segment(a, b, c);
        }
        for ix in -20..=20 {
            for iy in -20..=20 {
                let q = p(ix, iy);
                if on_segment(a, b, q) && on_segment(c, d, q) {
                    return true;
                }
            }
        }
        segments_intersect_by_orientation(a, b, c, d)
    }

    fn segments_intersect_by_orientation(a: Point, b: Point, c: Point, d: Point) -> bool {
        let x1 = cross(b - a, c - a);
        let x2 = cross(b - a, d - a);
        let y1 = cross(d - c, a - c);
        let y2 = cross(d - c, b - c);
        x1.signum() * x2.signum() <= 0 && y1.signum() * y2.signum() <= 0
    }

    #[test]
    fn ccw_cases() {
        let a = p(0, 0);
        let b = p(2, 0);
        assert_eq!(ccw(a, b, p(1, 1)), Ccw::CounterClockwise);
        assert_eq!(ccw(a, b, p(1, -1)), Ccw::Clockwise);
        assert_eq!(ccw(a, b, p(-1, 0)), Ccw::OnlineBack);
        assert_eq!(ccw(a, b, p(3, 0)), Ccw::OnlineFront);
        assert_eq!(ccw(a, b, p(1, 0)), Ccw::OnSegment);
    }

    #[test]
    fn segment_intersection_cases() {
        assert!(segments_intersect(p(0, 0), p(4, 0), p(2, -1), p(2, 1)));
        assert!(segments_intersect(p(0, 0), p(4, 0), p(2, 0), p(6, 0)));
        assert!(segments_intersect(p(0, 0), p(0, 0), p(-1, -1), p(1, 1)));
        assert!(!segments_intersect(p(0, 0), p(1, 0), p(2, 0), p(3, 0)));
        assert!(!segments_intersect(p(0, 0), p(1, 1), p(2, 0), p(3, 1)));
    }

    #[test]
    fn random_segments_match_brute() {
        let mut seed = 987654321u64;
        let mut rng = || {
            seed ^= seed << 7;
            seed ^= seed >> 9;
            seed
        };
        for _ in 0..2_000 {
            let mut next = || (rng() % 21) as i64 - 10;
            let a = p(next(), next());
            let b = p(next(), next());
            let c = p(next(), next());
            let d = p(next(), next());
            assert_eq!(
                segments_intersect(a, b, c, d),
                brute_segments_intersect(a, b, c, d),
                "{a:?} {b:?} {c:?} {d:?}"
            );
        }
    }

    #[test]
    fn polygon_area() {
        let tri = [p(0, 0), p(4, 0), p(0, 3)];
        assert_eq!(signed_area2(&tri), 12);
        assert_eq!(area2(&tri), 12);
        let rev = [p(0, 0), p(0, 3), p(4, 0)];
        assert_eq!(signed_area2(&rev), -12);
        assert_eq!(area2(&rev), 12);
    }
}
