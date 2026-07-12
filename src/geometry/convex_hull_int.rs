//! 整数座標点の凸包（Andrew monotone chain）。重複点と退化ケースに対応する。
//!
//! ```
//! use cplib::geometry::basic::Point;
//! use cplib::geometry::convex_hull_int::*;
//!
//! let pts = [Point::new(0, 0), Point::new(1, 0), Point::new(0, 1), Point::new(1, 1)];
//! let hull = convex_hull(&pts, false);
//! assert_eq!(hull.len(), 4);
//! ```

use crate::geometry::basic::{cross, Point};

/// 凸包を反時計回りに返す。`keep_collinear` が true なら辺上の点も残す。
pub fn convex_hull(points: &[Point], keep_collinear: bool) -> Vec<Point> {
    let mut pts = points.to_vec();
    pts.sort_unstable();
    pts.dedup();
    if pts.len() <= 1 {
        return pts;
    }
    if keep_collinear && pts.iter().all(|&p| cross(pts[1] - pts[0], p - pts[0]) == 0) {
        return pts;
    }
    let mut lower = Vec::new();
    for &p in &pts {
        while lower.len() >= 2 {
            let n = lower.len();
            let cr = cross(lower[n - 1] - lower[n - 2], p - lower[n - 1]);
            if if keep_collinear { cr < 0 } else { cr <= 0 } {
                lower.pop();
            } else {
                break;
            }
        }
        lower.push(p);
    }
    let mut upper = Vec::new();
    for &p in pts.iter().rev() {
        while upper.len() >= 2 {
            let n = upper.len();
            let cr = cross(upper[n - 1] - upper[n - 2], p - upper[n - 1]);
            if if keep_collinear { cr < 0 } else { cr <= 0 } {
                upper.pop();
            } else {
                break;
            }
        }
        upper.push(p);
    }
    lower.pop();
    upper.pop();
    lower.extend(upper);
    if lower.is_empty() && !pts.is_empty() {
        lower.push(pts[0]);
    }
    lower
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::misc::xorshift::XorShift;

    fn p(x: i64, y: i64) -> Point {
        Point::new(x, y)
    }

    fn all_points_inside_or_on(hull: &[Point], pts: &[Point]) -> bool {
        if hull.len() <= 2 {
            return true;
        }
        pts.iter().all(|&q| {
            (0..hull.len()).all(|i| {
                cross(hull[(i + 1) % hull.len()] - hull[i], q - hull[i]) >= 0
            })
        })
    }

    #[test]
    fn square_and_collinear() {
        let pts = [p(0, 0), p(1, 0), p(1, 1), p(0, 1), p(0, 0), p(1, 0)];
        assert_eq!(convex_hull(&pts, false), vec![p(0, 0), p(1, 0), p(1, 1), p(0, 1)]);
        let line = [p(0, 0), p(1, 0), p(2, 0)];
        assert_eq!(convex_hull(&line, false), vec![p(0, 0), p(2, 0)]);
        assert_eq!(convex_hull(&line, true), vec![p(0, 0), p(1, 0), p(2, 0)]);
    }

    #[test]
    fn random_contains_all_points() {
        let mut rng = XorShift::new(4242);
        for _ in 0..200 {
            let n = 1 + rng.next_range(30) as usize;
            let pts: Vec<Point> = (0..n)
                .map(|_| p(rng.next_range(21) as i64 - 10, rng.next_range(21) as i64 - 10))
                .collect();
            let hull = convex_hull(&pts, false);
            assert!(all_points_inside_or_on(&hull, &pts));
        }
    }
}
