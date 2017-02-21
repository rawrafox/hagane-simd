use std;
use ::*;

impl std::ops::Add for double2x3 {
  type Output = Self;

  #[inline(always)]
  fn add(self, other: Self) -> Self {
    return double2x3(self.0 + other.0, self.1 + other.1);
  }
}

impl std::ops::Sub for double2x3 {
  type Output = Self;

  #[inline(always)]
  fn sub(self, other: Self) -> Self {
    return double2x3(self.0 - other.0, self.1 - other.1);
  }
}

impl std::ops::Mul<double3x2> for double2x3 {
  type Output = double3x3;

  #[inline(always)]
  fn mul(self, other: double3x2) -> Self::Output {
    return self.dot(other);
  }
}

impl std::ops::Mul<double2> for double2x3 {
  type Output = double3;

  #[inline(always)]
  fn mul(self, other: double2) -> Self::Output {
    return self.dot(other);
  }
}

impl std::ops::Mul<f64> for double2x3 {
  type Output = Self;

  #[inline(always)]
  fn mul(self, other: f64) -> Self {
    let a = double3::broadcast(other);

    return double2x3(a * self.0, a * self.1);
  }
}

impl Dot<double3x2> for double2x3 {
  type DotProduct = double3x3;

  #[inline(always)]
  fn dot(self, other: double3x2) -> Self::DotProduct {
    return double3x3(self.dot(other.0), self.dot(other.1), self.dot(other.2));
  }
}

impl Dot<double2> for double2x3 {
  type DotProduct = double3;

  #[inline(always)]
  fn dot(self, other: double2) -> Self::DotProduct {
    return self.0 * other.0 + self.1 * other.1;
  }
}

impl PartialEq for double2x3 {
  #[inline]
  fn eq(&self, other: &double2x3) -> bool {
    return (self.0.eq(other.0) & self.1.eq(other.1)).all()
  }
}

impl double2x3 {
  #[inline(always)]
  pub fn from_columns(c0: double3, c1: double3) -> double2x3 {
    return double2x3(c0, c1);
  }

  #[inline(always)]
  pub fn from_rows(r0: double2, r1: double2, r2: double2) -> double2x3 {
    return double3x2(r0, r1, r2).transpose();
  }

  #[inline(always)]
  pub fn linear_combination(a: f64, x: double2x3, b: f64, y: double2x3) -> double2x3 {
    let a = double3::broadcast(a);
    let b = double3::broadcast(b);
    return double2x3(a * x.0 + b * y.0, a * x.1 + b * y.1);
  }

  #[inline(always)]
  pub fn transpose(self) -> double3x2 {
    let c0 = double2((self.0).0, (self.1).0);
    let c1 = double2((self.0).1, (self.1).1);
    let c2 = double2((self.0).2, (self.1).2);

    return double3x2(c0, c1, c2);
  }
}
