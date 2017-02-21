use std;
use ::*;

impl std::ops::Add for double3x2 {
  type Output = Self;

  #[inline(always)]
  fn add(self, other: Self) -> Self {
    return double3x2(self.0 + other.0, self.1 + other.1, self.2 + other.2);
  }
}

impl std::ops::Sub for double3x2 {
  type Output = Self;

  #[inline(always)]
  fn sub(self, other: Self) -> Self {
    return double3x2(self.0 - other.0, self.1 - other.1, self.2 - other.2);
  }
}

impl std::ops::Mul<double2x3> for double3x2 {
  type Output = double2x2;

  #[inline(always)]
  fn mul(self, other: double2x3) -> Self::Output {
    return self.dot(other);
  }
}

impl std::ops::Mul<double3> for double3x2 {
  type Output = double2;

  #[inline(always)]
  fn mul(self, other: double3) -> Self::Output {
    return self.dot(other);
  }
}

impl std::ops::Mul<f64> for double3x2 {
  type Output = Self;

  #[inline(always)]
  fn mul(self, other: f64) -> Self {
    let a = double2::broadcast(other);

    return double3x2(a * self.0, a * self.1, a * self.2);
  }
}

impl Dot<double2x3> for double3x2 {
  type DotProduct = double2x2;

  #[inline(always)]
  fn dot(self, other: double2x3) -> Self::DotProduct {
    return double2x2(self.dot(other.0), self.dot(other.1));
  }
}

impl Dot<double3> for double3x2 {
  type DotProduct = double2;

  #[inline(always)]
  fn dot(self, other: double3) -> Self::DotProduct {
    return self.0 * other.0 + self.1 * other.1 + self.2 * other.2;
  }
}

impl PartialEq for double3x2 {
  #[inline]
  fn eq(&self, other: &double3x2) -> bool {
    return (self.0.eq(other.0) & self.1.eq(other.1) & self.2.eq(other.2)).all()
  }
}

impl double3x2 {
  #[inline(always)]
  pub fn from_columns(c0: double2, c1: double2, c2: double2) -> double3x2 {
    return double3x2(c0, c1, c2);
  }

  #[inline(always)]
  pub fn from_rows(r0: double3, r1: double3) -> double3x2 {
    return double2x3(r0, r1).transpose();
  }

  #[inline(always)]
  pub fn linear_combination(a: f64, x: double3x2, b: f64, y: double3x2) -> double3x2 {
    let a = double2::broadcast(a);
    let b = double2::broadcast(b);
    return double3x2(a * x.0 + b * y.0, a * x.1 + b * y.1, a * x.2 + b * y.2);
  }

  #[inline(always)]
  pub fn transpose(self) -> double2x3 {
    let c0 = double3((self.0).0, (self.1).0, (self.2).0);
    let c1 = double3((self.0).1, (self.1).1, (self.2).1);

    return double2x3(c0, c1);
  }
}
