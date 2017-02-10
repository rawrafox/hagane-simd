use std;
use ::*;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct float3x2(pub float2, pub float2, pub float2);

impl std::ops::Add for float3x2 {
  type Output = Self;

  #[inline]
  fn add(self, other: Self) -> Self {
    return float3x2::add(self, other);
  }
}

impl std::ops::Sub for float3x2 {
  type Output = Self;

  #[inline]
  fn sub(self, other: Self) -> Self {
    return float3x2::sub(self, other);
  }
}

impl std::ops::Mul<f32> for float3x2 {
  type Output = Self;

  #[inline]
  fn mul(self, other: f32) -> Self {
    return float3x2::scale(other, self);
  }
}

impl float3x2 {
  #[inline]
  pub fn scale(a: f32, x: float3x2) -> float3x2 {
    let a = float2::broadcast(a);

    return float3x2(a * x.0, a * x.1, a * x.2);
  }

  #[inline]
  pub fn linear_combination(a: f32, x: float3x2, b: f32, y: float3x2) -> float3x2 {
    let a = float2::broadcast(a);
    let b = float2::broadcast(b);
    return float3x2(a * x.0 + b * y.0, a * x.1 + b * y.1, a * x.2 + b * y.2);
  }

  #[inline]
  pub fn add(x: float3x2, y: float3x2) -> float3x2 {
    return float3x2(x.0 + y.0, x.1 + y.1, x.2 + y.2);
  }

  #[inline]
  pub fn sub(x: float3x2, y: float3x2) -> float3x2 {
    return float3x2(x.0 - y.0, x.1 - y.1, x.2 - y.2);
  }

  #[inline]
  pub fn transpose(self) -> float2x3 {
    let c0 = float3((self.0).0, (self.1).0, (self.2).0);
    let c1 = float3((self.0).1, (self.1).1, (self.2).1);

    return float2x3(c0, c1);
  }
}
