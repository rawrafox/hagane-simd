use std;
use ::*;

impl std::ops::Add for float2x3 {
  type Output = Self;

  #[inline]
  fn add(self, other: Self) -> Self {
    return float2x3::add(self, other);
  }
}

impl std::ops::Sub for float2x3 {
  type Output = Self;

  #[inline]
  fn sub(self, other: Self) -> Self {
    return float2x3::sub(self, other);
  }
}

impl std::ops::Mul<f32> for float2x3 {
  type Output = Self;

  #[inline]
  fn mul(self, other: f32) -> Self {
    return float2x3::scale(other, self);
  }
}

impl float2x3 {
  #[inline]
  pub fn scale(a: f32, x: float2x3) -> float2x3 {
    let a = float3::broadcast(a);

    return float2x3(a * x.0, a * x.1);
  }

  #[inline]
  pub fn linear_combination(a: f32, x: float2x3, b: f32, y: float2x3) -> float2x3 {
    let a = float3::broadcast(a);
    let b = float3::broadcast(b);
    return float2x3(a * x.0 + b * y.0, a * x.1 + b * y.1);
  }

  #[inline]
  pub fn add(x: float2x3, y: float2x3) -> float2x3 {
    return float2x3(x.0 + y.0, x.1 + y.1);
  }

  #[inline]
  pub fn sub(x: float2x3, y: float2x3) -> float2x3 {
    return float2x3(x.0 - y.0, x.1 - y.1);
  }

  #[inline]
  pub fn transpose(self) -> float3x2 {
    let c0 = float2((self.0).0, (self.1).0);
    let c1 = float2((self.0).1, (self.1).1);
    let c2 = float2((self.0).2, (self.1).2);

    return float3x2(c0, c1, c2);
  }
}
