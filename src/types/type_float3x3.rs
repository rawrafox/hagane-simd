use std;
use ::*;
use ::simd::*;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct float3x3(pub float3, pub float3, pub float3);

extern {
  fn __invert_f3(a: float3x3) -> float3x3;
}

impl std::ops::Add for float3x3 {
  type Output = Self;

  #[inline]
  fn add(self, other: Self) -> Self {
    return float3x3::add(self, other);
  }
}

impl std::ops::Sub for float3x3 {
  type Output = Self;

  #[inline]
  fn sub(self, other: Self) -> Self {
    return float3x3::sub(self, other);
  }
}

impl std::ops::Mul for float3x3 {
  type Output = Self;

  #[inline]
  fn mul(self, other: Self) -> Self {
    return self.dot(other);
  }
}

impl std::ops::Mul<float3> for float3x3 {
  type Output = float3;

  #[inline]
  fn mul(self, other: float3) -> float3 {
    return self.dot(other);
  }
}

impl std::ops::Mul<f32> for float3x3 {
  type Output = Self;

  #[inline]
  fn mul(self, other: f32) -> Self {
    return float3x3::scale(other, self);
  }
}

impl simd::Dot for float3x3 {
  type Output = float3x3;

  #[inline]
  fn dot(self, other: float3x3) -> float3x3 {
    return float3x3(self.dot(other.0), self.dot(other.1), self.dot(other.2));
  }
}

impl simd::Dot<float3> for float3x3 {
  type Output = float3;

  #[inline]
  fn dot(self, other: float3) -> float3 {
    return self.0 * other.0 + self.1 * other.1 + self.2 * other.2;
  }
}

impl float3x3 {
  #[inline]
  pub fn identity(self) -> float3x3 {
    return float3x3(float3(1.0, 0.0, 0.0), float3(0.0, 1.0, 0.0), float3(0.0, 0.0, 1.0));
  }

  #[inline]
  pub fn scale(a: f32, x: float3x3) -> float3x3 {
    let a = float3::broadcast(a);

    return float3x3(a * x.0, a * x.1, a * x.2);
  }

  #[inline]
  pub fn linear_combination(a: f32, x: float3x3, b: f32, y: float3x3) -> float3x3 {
    let a = float3::broadcast(a);
    let b = float3::broadcast(b);
    return float3x3(a * x.0 + b * y.0, a * x.1 + b * y.1, a * x.2 + b * y.2);
  }

  #[inline]
  pub fn add(x: float3x3, y: float3x3) -> float3x3 {
    return float3x3(x.0 + y.0, x.1 + y.1, x.2 + y.2);
  }

  #[inline]
  pub fn sub(x: float3x3, y: float3x3) -> float3x3 {
    return float3x3(x.0 - y.0, x.1 - y.1, x.2 - y.2);
  }

  #[inline]
  pub fn transpose(self) -> float3x3 {
    let c0 = float3((self.0).0, (self.1).0, (self.2).0);
    let c1 = float3((self.0).1, (self.1).1, (self.2).1);
    let c2 = float3((self.0).2, (self.1).2, (self.2).2);

    return float3x3(c0, c1, c2);
  }

  #[inline]
  pub fn inverse(self) -> float3x3 {
    return unsafe { __invert_f3(self) };
  }
}
