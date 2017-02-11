use std;
use ::*;

impl std::ops::Add for double2x4 {
  type Output = Self;

  #[inline]
  fn add(self, other: Self) -> Self {
    return double2x4::add(self, other);
  }
}

impl std::ops::Sub for double2x4 {
  type Output = Self;

  #[inline]
  fn sub(self, other: Self) -> Self {
    return double2x4::sub(self, other);
  }
}

impl std::ops::Mul<f64> for double2x4 {
  type Output = Self;

  #[inline]
  fn mul(self, other: f64) -> Self {
    return double2x4::scale(other, self);
  }
}

impl double2x4 {
  #[inline]
  pub fn scale(a: f64, x: double2x4) -> double2x4 {
    let a = double4::broadcast(a);

    return double2x4(a * x.0, a * x.1);
  }

  #[inline]
  pub fn linear_combination(a: f64, x: double2x4, b: f64, y: double2x4) -> double2x4 {
    let a = double4::broadcast(a);
    let b = double4::broadcast(b);
    return double2x4(a * x.0 + b * y.0, a * x.1 + b * y.1);
  }

  #[inline]
  pub fn add(x: double2x4, y: double2x4) -> double2x4 {
    return double2x4(x.0 + y.0, x.1 + y.1);
  }

  #[inline]
  pub fn sub(x: double2x4, y: double2x4) -> double2x4 {
    return double2x4(x.0 - y.0, x.1 - y.1);
  }

  #[inline]
  pub fn transpose(self) -> double4x2 {
    let c0 = double2((self.0).0, (self.1).0);
    let c1 = double2((self.0).1, (self.1).1);
    let c2 = double2((self.0).2, (self.1).2);
    let c3 = double2((self.0).3, (self.1).3);

    return double4x2(c0, c1, c2, c3);
  }
}
