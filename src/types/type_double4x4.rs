use std;
use ::*;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct double4x4(pub double4, pub double4, pub double4, pub double4);
pub type matrix_double4x4 = double4x4;

extern {
  fn __invert_d4(a: double4x4) -> double4x4;
}

impl std::ops::Add for double4x4 {
  type Output = Self;

  #[inline]
  fn add(self, other: Self) -> Self {
    return double4x4::add(self, other);
  }
}

impl std::ops::Sub for double4x4 {
  type Output = Self;

  #[inline]
  fn sub(self, other: Self) -> Self {
    return double4x4::sub(self, other);
  }
}

impl std::ops::Mul for double4x4 {
  type Output = Self;

  #[inline]
  fn mul(self, other: Self) -> Self {
    return self.dot(other);
  }
}

impl std::ops::Mul<double4> for double4x4 {
  type Output = double4;

  #[inline]
  fn mul(self, other: double4) -> double4 {
    return self.dot(other);
  }
}

impl std::ops::Mul<f64> for double4x4 {
  type Output = Self;

  #[inline]
  fn mul(self, other: f64) -> Self {
    return double4x4::scale(other, self);
  }
}

impl Dot for double4x4 {
  type Output = double4x4;

  #[inline]
  fn dot(self, other: double4x4) -> double4x4 {
    return double4x4(self.dot(other.0), self.dot(other.1), self.dot(other.2), self.dot(other.3));
  }
}

impl Dot<double4> for double4x4 {
  type Output = double4;

  #[inline]
  fn dot(self, other: double4) -> double4 {
    return self.0 * other.0 + self.1 * other.1 + self.2 * other.2 + self.3 * other.3;
  }
}

impl double4x4 {
  #[inline]
  pub fn identity(self) -> double4x4 {
    return double4x4(double4(1.0, 0.0, 0.0, 0.0), double4(0.0, 1.0, 0.0, 0.0), double4(0.0, 0.0, 1.0, 0.0), double4(0.0, 0.0, 0.0, 1.0));
  }

  #[inline]
  pub fn scale(a: f64, x: double4x4) -> double4x4 {
    let a = double4::broadcast(a);

    return double4x4(a * x.0, a * x.1, a * x.2, a * x.3);
  }

  #[inline]
  pub fn linear_combination(a: f64, x: double4x4, b: f64, y: double4x4) -> double4x4 {
    let a = double4::broadcast(a);
    let b = double4::broadcast(b);
    return double4x4(a * x.0 + b * y.0, a * x.1 + b * y.1, a * x.2 + b * y.2, a * x.3 + b * y.3);
  }

  #[inline]
  pub fn add(x: double4x4, y: double4x4) -> double4x4 {
    return double4x4(x.0 + y.0, x.1 + y.1, x.2 + y.2, x.3 + y.3);
  }

  #[inline]
  pub fn sub(x: double4x4, y: double4x4) -> double4x4 {
    return double4x4(x.0 - y.0, x.1 - y.1, x.2 - y.2, x.3 - y.3);
  }

  #[inline]
  pub fn transpose(self) -> double4x4 {
    let c0 = double4((self.0).0, (self.1).0, (self.2).0, (self.3).0);
    let c1 = double4((self.0).1, (self.1).1, (self.2).1, (self.3).1);
    let c2 = double4((self.0).2, (self.1).2, (self.2).2, (self.3).2);
    let c3 = double4((self.0).3, (self.1).3, (self.2).3, (self.3).3);

    return double4x4(c0, c1, c2, c3);
  }

  #[inline]
  pub fn inverse(self) -> double4x4 {
    return unsafe { __invert_d4(self) };
  }
}
