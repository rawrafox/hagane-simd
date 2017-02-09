use std;
use ::*;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct double2x2(pub double2, pub double2);
pub type matrix_double2x2 = double2x2;

extern {
  fn __invert_d2(a: double2x2) -> double2x2;
}

impl std::ops::Add for double2x2 {
  type Output = Self;

  #[inline]
  fn add(self, other: Self) -> Self {
    return double2x2::add(self, other);
  }
}

impl std::ops::Sub for double2x2 {
  type Output = Self;

  #[inline]
  fn sub(self, other: Self) -> Self {
    return double2x2::sub(self, other);
  }
}

impl std::ops::Mul for double2x2 {
  type Output = Self;

  #[inline]
  fn mul(self, other: Self) -> Self {
    return self.dot(other);
  }
}

impl std::ops::Mul<double2> for double2x2 {
  type Output = double2;

  #[inline]
  fn mul(self, other: double2) -> double2 {
    return self.dot(other);
  }
}

impl std::ops::Mul<f64> for double2x2 {
  type Output = Self;

  #[inline]
  fn mul(self, other: f64) -> Self {
    return double2x2::scale(other, self);
  }
}

impl Dot for double2x2 {
  type Output = double2x2;

  #[inline]
  fn dot(self, other: double2x2) -> double2x2 {
    return double2x2(self.dot(other.0), self.dot(other.1));
  }
}

impl Dot<double2> for double2x2 {
  type Output = double2;

  #[inline]
  fn dot(self, other: double2) -> double2 {
    return self.0 * other.0 + self.1 * other.1;
  }
}

impl double2x2 {
  #[inline]
  pub fn identity(self) -> double2x2 {
    return double2x2(double2(1.0, 0.0), double2(0.0, 1.0));
  }

  #[inline]
  pub fn scale(a: f64, x: double2x2) -> double2x2 {
    let a = double2::broadcast(a);

    return double2x2(a * x.0, a * x.1);
  }

  #[inline]
  pub fn linear_combination(a: f64, x: double2x2, b: f64, y: double2x2) -> double2x2 {
    let a = double2::broadcast(a);
    let b = double2::broadcast(b);
    return double2x2(a * x.0 + b * y.0, a * x.1 + b * y.1);
  }

  #[inline]
  pub fn add(x: double2x2, y: double2x2) -> double2x2 {
    return double2x2(x.0 + y.0, x.1 + y.1);
  }

  #[inline]
  pub fn sub(x: double2x2, y: double2x2) -> double2x2 {
    return double2x2(x.0 - y.0, x.1 - y.1);
  }

  #[inline]
  pub fn transpose(self) -> double2x2 {
    let c0 = double2((self.0).0, (self.1).0);
    let c1 = double2((self.0).1, (self.1).1);

    return double2x2(c0, c1);
  }

  #[inline]
  pub fn inverse(self) -> double2x2 {
    return unsafe { __invert_d2(self) };
  }
}
