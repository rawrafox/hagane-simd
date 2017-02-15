use std;
use ::*;

impl std::ops::Add for double3x2 {
  type Output = Self;

  #[inline]
  fn add(self, other: Self) -> Self {
    return double3x2(self.0 + other.0, self.1 + other.1, self.2 + other.2);
  }
}

impl std::ops::Sub for double3x2 {
  type Output = Self;

  #[inline]
  fn sub(self, other: Self) -> Self {
    return double3x2(self.0 - other.0, self.1 - other.1, self.2 - other.2);
  }
}

impl std::ops::Mul<f64> for double3x2 {
  type Output = Self;

  #[inline]
  fn mul(self, other: f64) -> Self {
    let a = double2::broadcast(other);

    return double3x2(a * self.0, a * self.1, a * self.2);
  }
}

impl double3x2 {
  #[inline]
  pub fn linear_combination(a: f64, x: double3x2, b: f64, y: double3x2) -> double3x2 {
    let a = double2::broadcast(a);
    let b = double2::broadcast(b);
    return double3x2(a * x.0 + b * y.0, a * x.1 + b * y.1, a * x.2 + b * y.2);
  }

  #[inline]
  pub fn transpose(self) -> double2x3 {
    let c0 = double3((self.0).0, (self.1).0, (self.2).0);
    let c1 = double3((self.0).1, (self.1).1, (self.2).1);

    return double2x3(c0, c1);
  }
}
