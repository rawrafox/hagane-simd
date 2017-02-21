use std;
use ::*;

impl std::ops::Add for double4x3 {
  type Output = Self;

  #[inline(always)]
  fn add(self, other: Self) -> Self {
    return double4x3(self.0 + other.0, self.1 + other.1, self.2 + other.2, self.3 + other.3);
  }
}

impl std::ops::Sub for double4x3 {
  type Output = Self;

  #[inline(always)]
  fn sub(self, other: Self) -> Self {
    return double4x3(self.0 - other.0, self.1 - other.1, self.2 - other.2, self.3 - other.3);
  }
}

impl std::ops::Mul<double3x4> for double4x3 {
  type Output = double3x3;

  #[inline(always)]
  fn mul(self, other: double3x4) -> Self::Output {
    return self.dot(other);
  }
}

impl std::ops::Mul<double4> for double4x3 {
  type Output = double3;

  #[inline(always)]
  fn mul(self, other: double4) -> Self::Output {
    return self.dot(other);
  }
}

impl std::ops::Mul<f64> for double4x3 {
  type Output = Self;

  #[inline(always)]
  fn mul(self, other: f64) -> Self {
    let a = double3::broadcast(other);

    return double4x3(a * self.0, a * self.1, a * self.2, a * self.3);
  }
}

impl Dot<double3x4> for double4x3 {
  type DotProduct = double3x3;

  #[inline(always)]
  fn dot(self, other: double3x4) -> Self::DotProduct {
    return double3x3(self.dot(other.0), self.dot(other.1), self.dot(other.2));
  }
}

impl Dot<double4> for double4x3 {
  type DotProduct = double3;

  #[inline(always)]
  fn dot(self, other: double4) -> Self::DotProduct {
    return self.0 * other.0 + self.1 * other.1 + self.2 * other.2 + self.3 * other.3;
  }
}

impl PartialEq for double4x3 {
  #[inline]
  fn eq(&self, other: &double4x3) -> bool {
    return (self.0.eq(other.0) & self.1.eq(other.1) & self.2.eq(other.2) & self.3.eq(other.3)).all()
  }
}

impl double4x3 {
  #[inline(always)]
  pub fn from_columns(c0: double3, c1: double3, c2: double3, c3: double3) -> double4x3 {
    return double4x3(c0, c1, c2, c3);
  }

  #[inline(always)]
  pub fn from_rows(r0: double4, r1: double4, r2: double4) -> double4x3 {
    return double3x4(r0, r1, r2).transpose();
  }

  #[inline(always)]
  pub fn linear_combination(a: f64, x: double4x3, b: f64, y: double4x3) -> double4x3 {
    let a = double3::broadcast(a);
    let b = double3::broadcast(b);
    return double4x3(a * x.0 + b * y.0, a * x.1 + b * y.1, a * x.2 + b * y.2, a * x.3 + b * y.3);
  }

  #[inline(always)]
  pub fn transpose(self) -> double3x4 {
    let c0 = double4((self.0).0, (self.1).0, (self.2).0, (self.3).0);
    let c1 = double4((self.0).1, (self.1).1, (self.2).1, (self.3).1);
    let c2 = double4((self.0).2, (self.1).2, (self.2).2, (self.3).2);

    return double3x4(c0, c1, c2);
  }
}
