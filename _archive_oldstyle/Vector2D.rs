// source snippet: key=Vector2D  prefix=Vector2D
// (旧実装/CamelCase。lib_ 版の旧バージョンの可能性あり)

#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub struct Vector2D(f64, f64);
impl Vector2D {
    pub fn add(a: f64, b: f64) -> f64 {
        let c = a + b;
        if c.abs() < 1e-10 {
            0.0
        } else {
            c
        }
    }
    pub fn dot(self, other: Vector2D) -> f64 {
        Self::add(self.0 * other.0, self.1 * other.1)
    }
    pub fn det(self, other: Vector2D) -> f64 {
        Self::add(self.0 * other.1, -self.1 * other.0)
    }
    pub fn length(&self) -> f64 {
        f64::sqrt((self.0).powi(2) + (self.1).powi(2))
    }
    pub fn unit(self) -> Vector2D {
        let l = self.length();
        Vector2D(self.0 / l, self.1 / l)
    }
    pub fn normal(self) -> Vector2D {
        Vector2D(self.1, -self.0)
    }
}
impl std::ops::Add for Vector2D {
    type Output = Vector2D;
    fn add(self, rhs: Vector2D) -> Self::Output {
        Vector2D(Vector2D::add(self.0, rhs.0), Vector2D::add(self.1, rhs.1))
    }
}
impl std::ops::Sub for Vector2D {
    type Output = Vector2D;
    fn sub(self, rhs: Vector2D) -> Self::Output {
        Vector2D(Vector2D::add(self.0, -rhs.0), Vector2D::add(self.1, -rhs.1))
    }
}
impl std::ops::Mul<f64> for Vector2D {
    type Output = Vector2D;
    fn mul(self, rhs: f64) -> Self::Output {
        Vector2D(rhs * self.0, rhs * self.1)
    }
}
impl std::ops::Div<f64> for Vector2D {
    type Output = Vector2D;
    fn div(self, rhs: f64) -> Self::Output {
        Vector2D(self.0 / rhs, self.1 / rhs)
    }
}
