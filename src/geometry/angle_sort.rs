//! 原点まわりの偏角ソート。
//!
//! 整数点は半平面と外積で比較するため、`atan2` を使わず exact に並べる。
//! 角度は正の x 軸から反時計回りに `[0, 2π)` の順。
//!
//! ```rust
//! use cplib::geometry::angle_sort::*;
//! use cplib::geometry::basic::Point;
//!
//! let mut points = [
//!     Point::new(0, -1),
//!     Point::new(1, 0),
//!     Point::new(-1, 0),
//!     Point::new(0, 1),
//! ];
//! sort_points_by_angle(&mut points);
//! assert_eq!(
//!     points,
//!     [
//!         Point::new(1, 0),
//!         Point::new(0, 1),
//!         Point::new(-1, 0),
//!         Point::new(0, -1),
//!     ]
//! );
//! ```

use crate::geometry::basic::Point;
use std::cmp::Ordering;

pub fn sort_points_by_angle(points: &mut [Point]) {
    points.sort_by(compare_angle);
}

pub fn angle_sort_indices(points: &[Point]) -> Vec<usize> {
    let mut indices: Vec<usize> = (0..points.len()).collect();
    indices.sort_by(|&i, &j| {
        compare_angle(&points[i], &points[j])
            .then_with(|| points[i].x.cmp(&points[j].x))
            .then_with(|| points[i].y.cmp(&points[j].y))
            .then_with(|| i.cmp(&j))
    });
    indices
}

pub fn compare_angle(a: &Point, b: &Point) -> Ordering {
    let ha = half(*a);
    let hb = half(*b);
    if ha != hb {
        return ha.cmp(&hb);
    }
    let cross = a.x as i128 * b.y as i128 - a.y as i128 * b.x as i128;
    if cross != 0 {
        return if cross > 0 {
            Ordering::Less
        } else {
            Ordering::Greater
        };
    }
    norm2(*a).cmp(&norm2(*b))
}

pub fn polar_angle(x: f64, y: f64) -> f64 {
    let mut theta = y.atan2(x);
    if theta < 0.0 {
        theta += std::f64::consts::TAU;
    }
    theta
}

pub fn sort_f64_points_by_angle(points: &mut [(f64, f64)]) {
    points.sort_by(|a, b| {
        polar_angle(a.0, a.1)
            .total_cmp(&polar_angle(b.0, b.1))
            .then_with(|| (a.0 * a.0 + a.1 * a.1).total_cmp(&(b.0 * b.0 + b.1 * b.1)))
    });
}

fn half(p: Point) -> u8 {
    if p.y > 0 || (p.y == 0 && p.x >= 0) {
        0
    } else {
        1
    }
}

fn norm2(p: Point) -> i128 {
    p.x as i128 * p.x as i128 + p.y as i128 * p.y as i128
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(x: i64, y: i64) -> Point {
        Point::new(x, y)
    }

    #[test]
    fn cardinal_directions() {
        let mut points = [p(0, -1), p(1, 0), p(-1, 0), p(0, 1)];
        sort_points_by_angle(&mut points);
        assert_eq!(points, [p(1, 0), p(0, 1), p(-1, 0), p(0, -1)]);
    }

    #[test]
    fn ties_by_distance() {
        let mut points = [p(2, 2), p(1, 1), p(-2, -2), p(-1, -1), p(0, 0)];
        sort_points_by_angle(&mut points);
        assert_eq!(points, [p(0, 0), p(1, 1), p(2, 2), p(-1, -1), p(-2, -2)]);
    }

    #[test]
    fn indices_are_stable_for_equal_points() {
        let points = [p(1, 0), p(1, 0), p(0, 1), p(-1, 0)];
        assert_eq!(angle_sort_indices(&points), vec![0, 1, 2, 3]);
    }

    #[test]
    fn random_matches_atan2_order() {
        let mut seed = 8642097531u64;
        let mut rng = || {
            seed ^= seed << 7;
            seed ^= seed >> 9;
            seed
        };
        for _ in 0..500 {
            let mut points = Vec::new();
            for _ in 0..30 {
                let x = (rng() % 21) as i64 - 10;
                let y = (rng() % 21) as i64 - 10;
                if x != 0 || y != 0 {
                    points.push(p(x, y));
                }
            }
            let mut got = points.clone();
            sort_points_by_angle(&mut got);

            let mut expected = points.clone();
            expected.sort_by(|a, b| {
                polar_angle(a.x as f64, a.y as f64)
                    .total_cmp(&polar_angle(b.x as f64, b.y as f64))
                    .then_with(|| norm2(*a).cmp(&norm2(*b)))
            });
            assert_eq!(got, expected);
        }
    }

    #[test]
    fn f64_points() {
        let mut points = [(0.0, -1.0), (1.0, 0.0), (-1.0, 0.0), (0.0, 1.0)];
        sort_f64_points_by_angle(&mut points);
        assert_eq!(points, [(1.0, 0.0), (0.0, 1.0), (-1.0, 0.0), (0.0, -1.0)]);
        assert_eq!(polar_angle(1.0, 0.0), 0.0);
        assert!((polar_angle(0.0, -1.0) - 1.5 * std::f64::consts::PI).abs() < 1e-12);
    }
}
