use std;
use ::*;

extern {
  fn __invert_d2(a: double2x2) -> double2x2;
}

impl std::ops::Add for double2x2 {
  type Output = Self;

  #[inline(always)]
  fn add(self, other: Self) -> Self {
    return double2x2(self.0 + other.0, self.1 + other.1);
  }
}

impl std::ops::Sub for double2x2 {
  type Output = Self;

  #[inline(always)]
  fn sub(self, other: Self) -> Self {
    return double2x2(self.0 - other.0, self.1 - other.1);
  }
}

impl std::ops::Mul for double2x2 {
  type Output = Self;

  #[inline(always)]
  fn mul(self, other: Self) -> Self {
    return self.dot(other);
  }
}

impl std::ops::Mul<double2> for double2x2 {
  type Output = double2;

  #[inline(always)]
  fn mul(self, other: double2) -> double2 {
    return self.dot(other);
  }
}

impl std::ops::Mul<f64> for double2x2 {
  type Output = Self;

  #[inline(always)]
  fn mul(self, other: f64) -> Self {
    let a = double2::broadcast(other);

    return double2x2(a * self.0, a * self.1);
  }
}

impl Dot<double2x2> for double2x2 {
  type DotProduct = double2x2;

  #[inline(always)]
  fn dot(self, other: double2x2) -> double2x2 {
    return double2x2(self.dot(other.0), self.dot(other.1));
  }
}

impl Dot<double2> for double2x2 {
  type DotProduct = double2;

  #[inline(always)]
  fn dot(self, other: double2) -> double2 {
    return self.0 * other.0 + self.1 * other.1;
  }
}

impl double2x2 {
  #[inline(always)]
  pub fn identity(self) -> double2x2 {
    return double2x2(double2(1.0, 0.0), double2(0.0, 1.0));
  }

  #[inline(always)]
  pub fn linear_combination(a: f64, x: double2x2, b: f64, y: double2x2) -> double2x2 {
    let a = double2::broadcast(a);
    let b = double2::broadcast(b);
    return double2x2(a * x.0 + b * y.0, a * x.1 + b * y.1);
  }

  #[inline(always)]
  pub fn transpose(self) -> double2x2 {
    let c0 = double2((self.0).0, (self.1).0);
    let c1 = double2((self.0).1, (self.1).1);

    return double2x2(c0, c1);
  }

  #[inline(always)]
  pub fn inverse(self) -> double2x2 {
    return unsafe { __invert_d2(self) };
  }
}
