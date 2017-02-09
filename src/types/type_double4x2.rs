use std;
use ::*;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct double4x2(pub double2, pub double2, pub double2, pub double2);
pub type matrix_double4x2 = double4x2;

impl std::ops::Add for double4x2 {
  type Output = Self;

  #[inline]
  fn add(self, other: Self) -> Self {
    return double4x2::add(self, other);
  }
}

impl std::ops::Sub for double4x2 {
  type Output = Self;

  #[inline]
  fn sub(self, other: Self) -> Self {
    return double4x2::sub(self, other);
  }
}

impl std::ops::Mul<f64> for double4x2 {
  type Output = Self;

  #[inline]
  fn mul(self, other: f64) -> Self {
    return double4x2::scale(other, self);
  }
}

impl double4x2 {
  #[inline]
  pub fn scale(a: f64, x: double4x2) -> double4x2 {
    let a = double2::broadcast(a);

    return double4x2(a * x.0, a * x.1, a * x.2, a * x.3);
  }

  #[inline]
  pub fn linear_combination(a: f64, x: double4x2, b: f64, y: double4x2) -> double4x2 {
    let a = double2::broadcast(a);
    let b = double2::broadcast(b);
    return double4x2(a * x.0 + b * y.0, a * x.1 + b * y.1, a * x.2 + b * y.2, a * x.3 + b * y.3);
  }

  #[inline]
  pub fn add(x: double4x2, y: double4x2) -> double4x2 {
    return double4x2(x.0 + y.0, x.1 + y.1, x.2 + y.2, x.3 + y.3);
  }

  #[inline]
  pub fn sub(x: double4x2, y: double4x2) -> double4x2 {
    return double4x2(x.0 - y.0, x.1 - y.1, x.2 - y.2, x.3 - y.3);
  }

  #[inline]
  pub fn transpose(self) -> double2x4 {
    let c0 = double4((self.0).0, (self.1).0, (self.2).0, (self.3).0);
    let c1 = double4((self.0).1, (self.1).1, (self.2).1, (self.3).1);

    return double2x4(c0, c1);
  }
}
