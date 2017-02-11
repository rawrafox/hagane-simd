use std;
use ::*;

impl std::ops::Add for double4x3 {
  type Output = Self;

  #[inline]
  fn add(self, other: Self) -> Self {
    return double4x3::add(self, other);
  }
}

impl std::ops::Sub for double4x3 {
  type Output = Self;

  #[inline]
  fn sub(self, other: Self) -> Self {
    return double4x3::sub(self, other);
  }
}

impl std::ops::Mul<f64> for double4x3 {
  type Output = Self;

  #[inline]
  fn mul(self, other: f64) -> Self {
    return double4x3::scale(other, self);
  }
}

impl double4x3 {
  #[inline]
  pub fn scale(a: f64, x: double4x3) -> double4x3 {
    let a: double3 = a.broadcast();

    return double4x3(a * x.0, a * x.1, a * x.2, a * x.3);
  }

  #[inline]
  pub fn linear_combination(a: f64, x: double4x3, b: f64, y: double4x3) -> double4x3 {
    let a: double3 = a.broadcast();
    let b: double3 = b.broadcast();
    return double4x3(a * x.0 + b * y.0, a * x.1 + b * y.1, a * x.2 + b * y.2, a * x.3 + b * y.3);
  }

  #[inline]
  pub fn add(x: double4x3, y: double4x3) -> double4x3 {
    return double4x3(x.0 + y.0, x.1 + y.1, x.2 + y.2, x.3 + y.3);
  }

  #[inline]
  pub fn sub(x: double4x3, y: double4x3) -> double4x3 {
    return double4x3(x.0 - y.0, x.1 - y.1, x.2 - y.2, x.3 - y.3);
  }

  #[inline]
  pub fn transpose(self) -> double3x4 {
    let c0 = double4((self.0).0, (self.1).0, (self.2).0, (self.3).0);
    let c1 = double4((self.0).1, (self.1).1, (self.2).1, (self.3).1);
    let c2 = double4((self.0).2, (self.1).2, (self.2).2, (self.3).2);

    return double3x4(c0, c1, c2);
  }
}
