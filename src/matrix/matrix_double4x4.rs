use std;
use ::*;

extern {
  fn __invert_d4(a: double4x4) -> double4x4;
}

impl std::ops::Add for double4x4 {
  type Output = Self;

  #[inline(always)]
  fn add(self, other: Self) -> Self {
    return double4x4(self.0 + other.0, self.1 + other.1, self.2 + other.2, self.3 + other.3);
  }
}

impl std::ops::Sub for double4x4 {
  type Output = Self;

  #[inline(always)]
  fn sub(self, other: Self) -> Self {
    return double4x4(self.0 - other.0, self.1 - other.1, self.2 - other.2, self.3 - other.3);
  }
}

impl std::ops::Mul<double4x4> for double4x4 {
  type Output = double4x4;

  #[inline(always)]
  fn mul(self, other: double4x4) -> Self::Output {
    return self.dot(other);
  }
}

impl std::ops::Mul<double4> for double4x4 {
  type Output = double4;

  #[inline(always)]
  fn mul(self, other: double4) -> Self::Output {
    return self.dot(other);
  }
}

impl std::ops::Mul<f64> for double4x4 {
  type Output = Self;

  #[inline(always)]
  fn mul(self, other: f64) -> Self {
    let a = double4::broadcast(other);

    return double4x4(a * self.0, a * self.1, a * self.2, a * self.3);
  }
}

impl Dot<double4x4> for double4x4 {
  type DotProduct = double4x4;

  #[inline(always)]
  fn dot(self, other: double4x4) -> Self::DotProduct {
    return double4x4(self.dot(other.0), self.dot(other.1), self.dot(other.2), self.dot(other.3));
  }
}

impl Dot<double4> for double4x4 {
  type DotProduct = double4;

  #[inline(always)]
  fn dot(self, other: double4) -> Self::DotProduct {
    return self.0 * other.0 + self.1 * other.1 + self.2 * other.2 + self.3 * other.3;
  }
}

impl PartialEq for double4x4 {
  #[inline]
  fn eq(&self, other: &double4x4) -> bool {
    return (self.0.eq(other.0) & self.1.eq(other.1) & self.2.eq(other.2) & self.3.eq(other.3)).all()
  }
}

impl double4x4 {
  #[inline(always)]
  pub fn from_columns(c0: double4, c1: double4, c2: double4, c3: double4) -> double4x4 {
    return double4x4(c0, c1, c2, c3);
  }

  #[inline(always)]
  pub fn from_rows(r0: double4, r1: double4, r2: double4, r3: double4) -> double4x4 {
    return double4x4(r0, r1, r2, r3).transpose();
  }

  #[inline(always)]
  pub fn identity(self) -> double4x4 {
    return double4x4(double4(1.0, 0.0, 0.0, 0.0), double4(0.0, 1.0, 0.0, 0.0), double4(0.0, 0.0, 1.0, 0.0), double4(0.0, 0.0, 0.0, 1.0));
  }

  #[inline(always)]
  pub fn linear_combination(a: f64, x: double4x4, b: f64, y: double4x4) -> double4x4 {
    let a = double4::broadcast(a);
    let b = double4::broadcast(b);
    return double4x4(a * x.0 + b * y.0, a * x.1 + b * y.1, a * x.2 + b * y.2, a * x.3 + b * y.3);
  }

  #[inline(always)]
  pub fn transpose(self) -> double4x4 {
    let c0 = double4((self.0).0, (self.1).0, (self.2).0, (self.3).0);
    let c1 = double4((self.0).1, (self.1).1, (self.2).1, (self.3).1);
    let c2 = double4((self.0).2, (self.1).2, (self.2).2, (self.3).2);
    let c3 = double4((self.0).3, (self.1).3, (self.2).3, (self.3).3);

    return double4x4(c0, c1, c2, c3);
  }

  #[inline(always)]
  pub fn inverse(self) -> double4x4 {
    return unsafe { __invert_d4(self) };
  }
}
