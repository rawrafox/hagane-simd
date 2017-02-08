use std;
use ::*;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct float2x2(pub float2, pub float2);
pub type matrix_float2x2 = float2x2;

extern {
  fn __invert_f2(a: float2x2) -> float2x2;
}

impl std::ops::Add for float2x2 {
  type Output = Self;

  #[inline]
  fn add(self, other: Self) -> Self {
    return float2x2::add(self, other);
  }
}

impl std::ops::Sub for float2x2 {
  type Output = Self;

  #[inline]
  fn sub(self, other: Self) -> Self {
    return float2x2::sub(self, other);
  }
}

impl std::ops::Mul for float2x2 {
  type Output = Self;

  #[inline]
  fn mul(self, other: Self) -> Self {
    return self.dot(other);
  }
}

impl std::ops::Mul<float2> for float2x2 {
  type Output = float2;

  #[inline]
  fn mul(self, other: float2) -> float2 {
    return self.dot(other);
  }
}

impl std::ops::Mul<f32> for float2x2 {
  type Output = Self;

  #[inline]
  fn mul(self, other: f32) -> Self {
    return float2x2::scale(other, self);
  }
}

impl Dot for float2x2 {
  type Output = float2x2;

  #[inline]
  fn dot(self, other: float2x2) -> float2x2 {
    return float2x2(self.dot(other.0), self.dot(other.1));
  }
}

impl Dot<float2> for float2x2 {
  type Output = float2;

  #[inline]
  fn dot(self, other: float2) -> float2 {
    return self.0 * other.0 + self.1 * other.1;
  }
}

impl float2x2 {
  #[inline]
  pub fn identity(self) -> float2x2 {
    return float2x2(float2(1.0, 0.0), float2(0.0, 1.0));
  }

  #[inline]
  pub fn scale(a: f32, x: float2x2) -> float2x2 {
    let a = float2::broadcast(a);

    return float2x2(a * x.0, a * x.1);
  }

  #[inline]
  pub fn linear_combination(a: f32, x: float2x2, b: f32, y: float2x2) -> float2x2 {
    let a = float2::broadcast(a);
    let b = float2::broadcast(b);
    return float2x2(a * x.0 + b * y.0, a * x.1 + b * y.1);
  }

  #[inline]
  pub fn add(x: float2x2, y: float2x2) -> float2x2 {
    return float2x2(x.0 + y.0, x.1 + y.1);
  }

  #[inline]
  pub fn sub(x: float2x2, y: float2x2) -> float2x2 {
    return float2x2(x.0 - y.0, x.1 - y.1);
  }

  #[inline]
  pub fn transpose(self) -> float2x2 {
    let c0 = float2((self.0).0, (self.1).0);
    let c1 = float2((self.0).1, (self.1).1);

    return float2x2(c0, c1);
  }

  #[inline]
  pub fn inverse(self) -> float2x2 {
    return unsafe { __invert_f2(self) };
  }
}
