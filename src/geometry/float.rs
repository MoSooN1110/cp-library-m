//! 浮動小数点幾何。
//!
//! `PointF` の基本演算、直線、円、凸包、最近点対を扱う。
//! 整数座標を exact に扱いたい場合は `geometry::basic` を使う。
//!
//! ```rust
//! use cplib::geometry::float::*;
//!
//! let a = PointF::new(0.0, 0.0);
//! let b = PointF::new(2.0, 0.0);
//! let c = PointF::new(0.0, 2.0);
//! let circle = CircleF::circumcircle(a, b, c).unwrap();
//! assert!(circle.center.approx_eq(PointF::new(1.0, 1.0), 1e-9));
//! assert!((circle.radius - 2f64.sqrt()).abs() < 1e-9);
//! ```

use std::cmp::Ordering;
use std::ops::{Add, Div, Mul, Neg, Sub};

pub const EPS: f64 = 1e-9;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PointF {
    pub x: f64,
    pub y: f64,
}

impl PointF {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn dot(self, rhs: Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y
    }

    pub fn cross(self, rhs: Self) -> f64 {
        self.x * rhs.y - self.y * rhs.x
    }

    pub fn norm2(self) -> f64 {
        self.dot(self)
    }

    pub fn norm(self) -> f64 {
        self.norm2().sqrt()
    }

    pub fn dist(self, rhs: Self) -> f64 {
        (self - rhs).norm()
    }

    pub fn unit(self) -> Self {
        self / self.norm()
    }

    pub fn normal(self) -> Self {
        Self::new(-self.y, self.x)
    }

    pub fn rotate_complex(self, rhs: Self) -> Self {
        Self::new(
            self.x * rhs.x - self.y * rhs.y,
            self.x * rhs.y + self.y * rhs.x,
        )
    }

    pub fn approx_eq(self, rhs: Self, eps: f64) -> bool {
        (self.x - rhs.x).abs() <= eps && (self.y - rhs.y).abs() <= eps
    }
}

impl Add for PointF {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for PointF {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Neg for PointF {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y)
    }
}

impl Mul<f64> for PointF {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl Div<f64> for PointF {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LineF {
    pub p: PointF,
    pub dir: PointF,
}

impl LineF {
    pub fn from_point_dir(p: PointF, dir: PointF) -> Self {
        assert!(dir.norm() > EPS);
        Self { p, dir }
    }

    pub fn from_two_points(a: PointF, b: PointF) -> Self {
        Self::from_point_dir(a, b - a)
    }

    pub fn intersection(self, rhs: Self) -> Option<PointF> {
        let det = self.dir.cross(rhs.dir);
        if det.abs() <= EPS {
            return None;
        }
        let t = (rhs.p - self.p).cross(rhs.dir) / det;
        Some(self.p + self.dir * t)
    }

    pub fn distance(self, p: PointF) -> f64 {
        (p - self.p).cross(self.dir).abs() / self.dir.norm()
    }

    pub fn projection(self, p: PointF) -> PointF {
        self.p + self.dir * ((p - self.p).dot(self.dir) / self.dir.norm2())
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CircleF {
    pub center: PointF,
    pub radius: f64,
}

impl CircleF {
    pub fn new(center: PointF, radius: f64) -> Self {
        assert!(radius >= 0.0);
        Self { center, radius }
    }

    pub fn incircle(a: PointF, b: PointF, c: PointF) -> Option<Self> {
        if (b - a).cross(c - a).abs() <= EPS {
            return None;
        }
        let la = b.dist(c);
        let lb = a.dist(c);
        let lc = a.dist(b);
        let per = la + lb + lc;
        let center = (a * la + b * lb + c * lc) / per;
        let radius = LineF::from_two_points(a, b).distance(center);
        Some(Self { center, radius })
    }

    pub fn circumcircle(a: PointF, b: PointF, c: PointF) -> Option<Self> {
        let d = 2.0 * (a.x * (b.y - c.y) + b.x * (c.y - a.y) + c.x * (a.y - b.y));
        if d.abs() <= EPS {
            return None;
        }
        let aa = a.norm2();
        let bb = b.norm2();
        let cc = c.norm2();
        let center = PointF::new(
            (aa * (b.y - c.y) + bb * (c.y - a.y) + cc * (a.y - b.y)) / d,
            (aa * (c.x - b.x) + bb * (a.x - c.x) + cc * (b.x - a.x)) / d,
        );
        Some(Self {
            center,
            radius: center.dist(a),
        })
    }

    pub fn intersections(self, rhs: Self) -> Vec<PointF> {
        let d = self.center.dist(rhs.center);
        if d <= EPS || d > self.radius + rhs.radius + EPS {
            return vec![];
        }
        if d + self.radius + EPS < rhs.radius || d + rhs.radius + EPS < self.radius {
            return vec![];
        }
        let x = (d * d + self.radius * self.radius - rhs.radius * rhs.radius) / (2.0 * d);
        let h2 = self.radius * self.radius - x * x;
        if h2 < -EPS {
            return vec![];
        }
        let base_dir = (rhs.center - self.center) / d;
        let base = self.center + base_dir * x;
        if h2 <= EPS {
            return vec![base];
        }
        let offset = base_dir.normal() * h2.sqrt();
        vec![base + offset, base - offset]
    }
}

pub fn segments_intersect(a: PointF, b: PointF, c: PointF, d: PointF) -> bool {
    let ab = b - a;
    let cd = d - c;
    let c1 = ab.cross(c - a);
    let c2 = ab.cross(d - a);
    let c3 = cd.cross(a - c);
    let c4 = cd.cross(b - c);
    if c1.abs() <= EPS && c2.abs() <= EPS {
        return ranges_overlap(a.x, b.x, c.x, d.x) && ranges_overlap(a.y, b.y, c.y, d.y);
    }
    c1 * c2 <= EPS && c3 * c4 <= EPS
}

pub fn convex_hull(mut points: Vec<PointF>) -> Vec<PointF> {
    points.sort_by(compare_xy);
    points.dedup_by(|a, b| a.approx_eq(*b, EPS));
    if points.len() <= 1 {
        return points;
    }
    let mut lower: Vec<PointF> = Vec::new();
    for &p in &points {
        while lower.len() >= 2
            && (lower[lower.len() - 1] - lower[lower.len() - 2]).cross(p - lower[lower.len() - 1])
                <= EPS
        {
            lower.pop();
        }
        lower.push(p);
    }
    let mut upper: Vec<PointF> = Vec::new();
    for &p in points.iter().rev() {
        while upper.len() >= 2
            && (upper[upper.len() - 1] - upper[upper.len() - 2]).cross(p - upper[upper.len() - 1])
                <= EPS
        {
            upper.pop();
        }
        upper.push(p);
    }
    lower.pop();
    upper.pop();
    lower.extend(upper);
    lower
}

pub fn closest_pair(points: &[PointF]) -> Option<(PointF, PointF, f64)> {
    if points.len() < 2 {
        return None;
    }
    let mut pts = points.to_vec();
    pts.sort_by(compare_xy);
    let (a, b, d2) = closest_pair_rec(&pts);
    Some((a, b, d2.sqrt()))
}

fn closest_pair_rec(points: &[PointF]) -> (PointF, PointF, f64) {
    let n = points.len();
    if n <= 3 {
        let mut best = (points[0], points[1], points[0].dist(points[1]).powi(2));
        for i in 0..n {
            for j in i + 1..n {
                let d2 = (points[i] - points[j]).norm2();
                if d2 < best.2 {
                    best = (points[i], points[j], d2);
                }
            }
        }
        return best;
    }
    let mid = n / 2;
    let x_mid = points[mid].x;
    let left = closest_pair_rec(&points[..mid]);
    let right = closest_pair_rec(&points[mid..]);
    let mut best = if left.2 <= right.2 { left } else { right };
    let d = best.2.sqrt();
    let mut strip: Vec<PointF> = points
        .iter()
        .copied()
        .filter(|p| (p.x - x_mid).abs() <= d)
        .collect();
    strip.sort_by(|a, b| a.y.total_cmp(&b.y).then_with(|| a.x.total_cmp(&b.x)));
    for i in 0..strip.len() {
        for j in i + 1..strip.len().min(i + 8) {
            let d2 = (strip[i] - strip[j]).norm2();
            if d2 < best.2 {
                best = (strip[i], strip[j], d2);
            }
        }
    }
    best
}

fn compare_xy(a: &PointF, b: &PointF) -> Ordering {
    a.x.total_cmp(&b.x).then_with(|| a.y.total_cmp(&b.y))
}

fn ranges_overlap(a: f64, b: f64, c: f64, d: f64) -> bool {
    let (ab_l, ab_r) = if a <= b { (a, b) } else { (b, a) };
    let (cd_l, cd_r) = if c <= d { (c, d) } else { (d, c) };
    ab_l <= cd_r + EPS && cd_l <= ab_r + EPS
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(x: f64, y: f64) -> PointF {
        PointF::new(x, y)
    }

    #[test]
    fn vector_and_line() {
        let a = p(3.0, 4.0);
        assert!((a.norm() - 5.0).abs() < EPS);
        assert_eq!(p(1.0, 0.0).dot(p(0.0, 1.0)), 0.0);
        assert_eq!(p(1.0, 0.0).cross(p(0.0, 1.0)), 1.0);
        let l1 = LineF::from_two_points(p(0.0, 0.0), p(2.0, 2.0));
        let l2 = LineF::from_two_points(p(0.0, 2.0), p(2.0, 0.0));
        assert!(l1.intersection(l2).unwrap().approx_eq(p(1.0, 1.0), EPS));
        assert!(
            (LineF::from_two_points(p(-1.0, 1.0), p(1.0, 1.0)).distance(p(0.0, 0.0)) - 1.0).abs()
                < EPS
        );
    }

    #[test]
    fn circles() {
        let c = CircleF::circumcircle(p(0.0, 0.0), p(0.0, 2.0), p(2.0, 0.0)).unwrap();
        assert!(c.center.approx_eq(p(1.0, 1.0), EPS));
        assert!((c.radius - 2f64.sqrt()).abs() < EPS);
        let inc = CircleF::incircle(p(0.0, 0.0), p(2.0, 0.0), p(0.0, 2.0)).unwrap();
        assert!(inc
            .center
            .approx_eq(p(2.0 - 2f64.sqrt(), 2.0 - 2f64.sqrt()), EPS));
        let xs = CircleF::new(p(1.0, 0.0), 2.0).intersections(CircleF::new(p(-1.0, 0.0), 2.0));
        assert_eq!(xs.len(), 2);
        assert!(xs.iter().any(|&q| q.approx_eq(p(0.0, 3f64.sqrt()), EPS)));
        assert!(xs.iter().any(|&q| q.approx_eq(p(0.0, -3f64.sqrt()), EPS)));
    }

    #[test]
    fn segment_intersection() {
        assert!(segments_intersect(
            p(0.0, 0.0),
            p(2.0, 0.0),
            p(1.0, -1.0),
            p(1.0, 1.0)
        ));
        assert!(segments_intersect(
            p(0.0, 0.0),
            p(2.0, 0.0),
            p(1.0, 0.0),
            p(3.0, 0.0)
        ));
        assert!(!segments_intersect(
            p(0.0, 0.0),
            p(1.0, 0.0),
            p(2.0, 0.0),
            p(3.0, 0.0)
        ));
    }

    #[test]
    fn hull_square() {
        let hull = convex_hull(vec![
            p(-1.0, -1.0),
            p(-1.0, 1.0),
            p(1.0, 1.0),
            p(1.0, -1.0),
            p(0.0, 0.0),
            p(0.1, 0.1),
        ]);
        assert_eq!(hull.len(), 4);
        for q in [p(-1.0, -1.0), p(-1.0, 1.0), p(1.0, 1.0), p(1.0, -1.0)] {
            assert!(hull.iter().any(|&r| r.approx_eq(q, EPS)));
        }
    }

    #[test]
    fn closest_pair_matches_brute() {
        let mut seed = 123456789u64;
        let mut rng = || {
            seed ^= seed << 7;
            seed ^= seed >> 9;
            seed
        };
        for _ in 0..200 {
            let n = 2 + rng() as usize % 30;
            let points: Vec<PointF> = (0..n)
                .map(|_| p((rng() % 1000) as f64 / 10.0, (rng() % 1000) as f64 / 10.0))
                .collect();
            let got = closest_pair(&points).unwrap().2;
            let mut expected = f64::INFINITY;
            for i in 0..n {
                for j in i + 1..n {
                    expected = expected.min(points[i].dist(points[j]));
                }
            }
            assert!((got - expected).abs() < EPS, "{got} {expected}");
        }
    }
}
