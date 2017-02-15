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

impl std::ops::Mul<f64> for double3x4 {
  type Output = Self;

  #[inline(always)]
  fn mul(self, other: f64) -> Self {
    let a = double4::broadcast(other);

    return double3x4(a * self.0, a * self.1, a * self.2);
  }
}

impl double3x4 {
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
