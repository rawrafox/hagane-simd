use std;
use ::*;

impl std::ops::Add for double2x3 {
  type Output = Self;

  #[inline]
  fn add(self, other: Self) -> Self {
    return double2x3(self.0 + other.0, self.1 + other.1);
  }
}

impl std::ops::Sub for double2x3 {
  type Output = Self;

  #[inline]
  fn sub(self, other: Self) -> Self {
    return double2x3(self.0 - other.0, self.1 - other.1);
  }
}

impl std::ops::Mul<f64> for double2x3 {
  type Output = Self;

  #[inline]
  fn mul(self, other: f64) -> Self {
    let a = double3::broadcast(other);

    return double2x3(a * self.0, a * self.1);
  }
}

impl double2x3 {
  #[inline]
  pub fn linear_combination(a: f64, x: double2x3, b: f64, y: double2x3) -> double2x3 {
    let a = double3::broadcast(a);
    let b = double3::broadcast(b);
    return double2x3(a * x.0 + b * y.0, a * x.1 + b * y.1);
  }

  #[inline]
  pub fn transpose(self) -> double3x2 {
    let c0 = double2((self.0).0, (self.1).0);
    let c1 = double2((self.0).1, (self.1).1);
    let c2 = double2((self.0).2, (self.1).2);

    return double3x2(c0, c1, c2);
  }
}
