use std;
use ::*;

impl std::ops::Add for float4x2 {
  type Output = Self;

  #[inline]
  fn add(self, other: Self) -> Self {
    return float4x2::add(self, other);
  }
}

impl std::ops::Sub for float4x2 {
  type Output = Self;

  #[inline]
  fn sub(self, other: Self) -> Self {
    return float4x2::sub(self, other);
  }
}

impl std::ops::Mul<f32> for float4x2 {
  type Output = Self;

  #[inline]
  fn mul(self, other: f32) -> Self {
    return float4x2::scale(other, self);
  }
}

impl float4x2 {
  #[inline]
  pub fn scale(a: f32, x: float4x2) -> float4x2 {
    let a = float2::broadcast(a);

    return float4x2(a * x.0, a * x.1, a * x.2, a * x.3);
  }

  #[inline]
  pub fn linear_combination(a: f32, x: float4x2, b: f32, y: float4x2) -> float4x2 {
    let a = float2::broadcast(a);
    let b = float2::broadcast(b);
    return float4x2(a * x.0 + b * y.0, a * x.1 + b * y.1, a * x.2 + b * y.2, a * x.3 + b * y.3);
  }

  #[inline]
  pub fn add(x: float4x2, y: float4x2) -> float4x2 {
    return float4x2(x.0 + y.0, x.1 + y.1, x.2 + y.2, x.3 + y.3);
  }

  #[inline]
  pub fn sub(x: float4x2, y: float4x2) -> float4x2 {
    return float4x2(x.0 - y.0, x.1 - y.1, x.2 - y.2, x.3 - y.3);
  }

  #[inline]
  pub fn transpose(self) -> float2x4 {
    let c0 = float4((self.0).0, (self.1).0, (self.2).0, (self.3).0);
    let c1 = float4((self.0).1, (self.1).1, (self.2).1, (self.3).1);

    return float2x4(c0, c1);
  }
}
