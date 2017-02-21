use std;
use ::*;

impl std::ops::Add for double3x4 {
  type Output = Self;

  #[inline(always)]
  fn add(self, other: Self) -> Self {
    return double3x4(self.0 + other.0, self.1 + other.1, self.2 + other.2);
  }
}

impl std::ops::Sub for double3x4 {
  type Output = Self;

  #[inline(always)]
  fn sub(self, other: Self) -> Self {
    return double3x4(self.0 - other.0, self.1 - other.1, self.2 - other.2);
  }
}

impl std::ops::Mul<double4x3> for double3x4 {
  type Output = double4x4;

  #[inline(always)]
  fn mul(self, other: double4x3) -> Self::Output {
    return self.dot(other);
  }
}

impl std::ops::Mul<double3> for double3x4 {
  type Output = double4;

  #[inline(always)]
  fn mul(self, other: double3) -> Self::Output {
    return self.dot(other);
  }
}

impl std::ops::Mul<f64> for double3x4 {
  type Output = Self;

  #[inline(always)]
  fn mul(self, other: f64) -> Self {
    let a = double4::broadcast(other);

    return double3x4(a * self.0, a * self.1, a * self.2);
  }
}

impl Dot<double4x3> for double3x4 {
  type DotProduct = double4x4;

  #[inline(always)]
  fn dot(self, other: double4x3) -> Self::DotProduct {
    return double4x4(self.dot(other.0), self.dot(other.1), self.dot(other.2), self.dot(other.3));
  }
}

impl Dot<double3> for double3x4 {
  type DotProduct = double4;

  #[inline(always)]
  fn dot(self, other: double3) -> Self::DotProduct {
    return self.0 * other.0 + self.1 * other.1 + self.2 * other.2;
  }
}

impl PartialEq for double3x4 {
  #[inline]
  fn eq(&self, other: &double3x4) -> bool {
    return (self.0.eq(other.0) & self.1.eq(other.1) & self.2.eq(other.2)).all()
  }
}

impl double3x4 {
  #[inline(always)]
  pub fn from_columns(c0: double4, c1: double4, c2: double4) -> double3x4 {
    return double3x4(c0, c1, c2);
  }

  #[inline(always)]
  pub fn from_rows(r0: double3, r1: double3, r2: double3, r3: double3) -> double3x4 {
    return double4x3(r0, r1, r2, r3).transpose();
  }

  #[inline(always)]
  pub fn linear_combination(a: f64, x: double3x4, b: f64, y: double3x4) -> double3x4 {
    let a = double4::broadcast(a);
    let b = double4::broadcast(b);
    return double3x4(a * x.0 + b * y.0, a * x.1 + b * y.1, a * x.2 + b * y.2);
  }

  #[inline(always)]
  pub fn transpose(self) -> double4x3 {
    let c0 = double3((self.0).0, (self.1).0, (self.2).0);
    let c1 = double3((self.0).1, (self.1).1, (self.2).1);
    let c2 = double3((self.0).2, (self.1).2, (self.2).2);
    let c3 = double3((self.0).3, (self.1).3, (self.2).3);

    return double4x3(c0, c1, c2, c3);
  }
}
