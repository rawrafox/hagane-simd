use std;
use ::*;

impl std::ops::Add for float3x4 {
  type Output = Self;

  #[inline]
  fn add(self, other: Self) -> Self {
    return float3x4::add(self, other);
  }
}

impl std::ops::Sub for float3x4 {
  type Output = Self;

  #[inline]
  fn sub(self, other: Self) -> Self {
    return float3x4::sub(self, other);
  }
}

impl std::ops::Mul<f32> for float3x4 {
  type Output = Self;

  #[inline]
  fn mul(self, other: f32) -> Self {
    return float3x4::scale(other, self);
  }
}

impl float3x4 {
  #[inline]
  pub fn scale(a: f32, x: float3x4) -> float3x4 {
    let a: float4 = a.broadcast();

    return float3x4(a * x.0, a * x.1, a * x.2);
  }

  #[inline]
  pub fn linear_combination(a: f32, x: float3x4, b: f32, y: float3x4) -> float3x4 {
    let a: float4 = a.broadcast();
    let b: float4 = b.broadcast();
    return float3x4(a * x.0 + b * y.0, a * x.1 + b * y.1, a * x.2 + b * y.2);
  }

  #[inline]
  pub fn add(x: float3x4, y: float3x4) -> float3x4 {
    return float3x4(x.0 + y.0, x.1 + y.1, x.2 + y.2);
  }

  #[inline]
  pub fn sub(x: float3x4, y: float3x4) -> float3x4 {
    return float3x4(x.0 - y.0, x.1 - y.1, x.2 - y.2);
  }

  #[inline]
  pub fn transpose(self) -> float4x3 {
    let c0 = float3((self.0).0, (self.1).0, (self.2).0);
    let c1 = float3((self.0).1, (self.1).1, (self.2).1);
    let c2 = float3((self.0).2, (self.1).2, (self.2).2);
    let c3 = float3((self.0).3, (self.1).3, (self.2).3);

    return float4x3(c0, c1, c2, c3);
  }
}
