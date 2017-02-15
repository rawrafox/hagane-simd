use std;
use ::*;

extern {
  fn __invert_d3(a: double3x3) -> double3x3;
}

impl std::ops::Add for double3x3 {
  type Output = Self;

  #[inline(always)]
  fn add(self, other: Self) -> Self {
    return double3x3(self.0 + other.0, self.1 + other.1, self.2 + other.2);
  }
}

impl std::ops::Sub for double3x3 {
  type Output = Self;

  #[inline(always)]
  fn sub(self, other: Self) -> Self {
    return double3x3(self.0 - other.0, self.1 - other.1, self.2 - other.2);
  }
}

impl std::ops::Mul for double3x3 {
  type Output = Self;

  #[inline(always)]
  fn mul(self, other: Self) -> Self {
    return self.dot(other);
  }
}

impl std::ops::Mul<double3> for double3x3 {
  type Output = double3;

  #[inline(always)]
  fn mul(self, other: double3) -> double3 {
    return self.dot(other);
  }
}

impl std::ops::Mul<f64> for double3x3 {
  type Output = Self;

  #[inline(always)]
  fn mul(self, other: f64) -> Self {
    let a = double3::broadcast(other);

    return double3x3(a * self.0, a * self.1, a * self.2);
  }
}

impl Dot<double3x3> for double3x3 {
  type DotProduct = double3x3;

  #[inline(always)]
  fn dot(self, other: double3x3) -> double3x3 {
    return double3x3(self.dot(other.0), self.dot(other.1), self.dot(other.2));
  }
}

impl Dot<double3> for double3x3 {
  type DotProduct = double3;

  #[inline(always)]
  fn dot(self, other: double3) -> double3 {
    return self.0 * other.0 + self.1 * other.1 + self.2 * other.2;
  }
}

impl double3x3 {
  #[inline(always)]
  pub fn identity(self) -> double3x3 {
    return double3x3(double3(1.0, 0.0, 0.0), double3(0.0, 1.0, 0.0), double3(0.0, 0.0, 1.0));
  }

  #[inline(always)]
  pub fn linear_combination(a: f64, x: double3x3, b: f64, y: double3x3) -> double3x3 {
    let a = double3::broadcast(a);
    let b = double3::broadcast(b);
    return double3x3(a * x.0 + b * y.0, a * x.1 + b * y.1, a * x.2 + b * y.2);
  }

  #[inline(always)]
  pub fn transpose(self) -> double3x3 {
    let c0 = double3((self.0).0, (self.1).0, (self.2).0);
    let c1 = double3((self.0).1, (self.1).1, (self.2).1);
    let c2 = double3((self.0).2, (self.1).2, (self.2).2);

    return double3x3(c0, c1, c2);
  }

  #[inline(always)]
  pub fn inverse(self) -> double3x3 {
    return unsafe { __invert_d3(self) };
  }
}
