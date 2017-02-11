use std;
use ::*;

impl std::ops::Add for double3x2 {
  type Output = Self;

  #[inline]
  fn add(self, other: Self) -> Self {
    return double3x2::add(self, other);
  }
}

impl std::ops::Sub for double3x2 {
  type Output = Self;

  #[inline]
  fn sub(self, other: Self) -> Self {
    return double3x2::sub(self, other);
  }
}

impl std::ops::Mul<f64> for double3x2 {
  type Output = Self;

  #[inline]
  fn mul(self, other: f64) -> Self {
    return double3x2::scale(other, self);
  }
}

impl double3x2 {
  #[inline]
  pub fn scale(a: f64, x: double3x2) -> double3x2 {
    let a = double2::broadcast(a);

    return double3x2(a * x.0, a * x.1, a * x.2);
  }

  #[inline]
  pub fn linear_combination(a: f64, x: double3x2, b: f64, y: double3x2) -> double3x2 {
    let a = double2::broadcast(a);
    let b = double2::broadcast(b);
    return double3x2(a * x.0 + b * y.0, a * x.1 + b * y.1, a * x.2 + b * y.2);
  }

  #[inline]
  pub fn add(x: double3x2, y: double3x2) -> double3x2 {
    return double3x2(x.0 + y.0, x.1 + y.1, x.2 + y.2);
  }

  #[inline]
  pub fn sub(x: double3x2, y: double3x2) -> double3x2 {
    return double3x2(x.0 - y.0, x.1 - y.1, x.2 - y.2);
  }

  #[inline]
  pub fn transpose(self) -> double2x3 {
    let c0 = double3((self.0).0, (self.1).0, (self.2).0);
    let c1 = double3((self.0).1, (self.1).1, (self.2).1);

    return double2x3(c0, c1);
  }
}