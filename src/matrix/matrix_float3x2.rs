use std;
use ::*;

impl std::ops::Add for float3x2 {
  type Output = Self;

  #[inline(always)]
  fn add(self, other: Self) -> Self {
    return float3x2(self.0 + other.0, self.1 + other.1, self.2 + other.2);
  }
}

impl std::ops::Sub for float3x2 {
  type Output = Self;

  #[inline(always)]
  fn sub(self, other: Self) -> Self {
    return float3x2(self.0 - other.0, self.1 - other.1, self.2 - other.2);
  }
}

impl std::ops::Mul<f32> for float3x2 {
  type Output = Self;

  #[inline(always)]
  fn mul(self, other: f32) -> Self {
    let a = float2::broadcast(other);

    return float3x2(a * self.0, a * self.1, a * self.2);
  }
}

impl float3x2 {
  #[inline(always)]
  pub fn linear_combination(a: f32, x: float3x2, b: f32, y: float3x2) -> float3x2 {
    let a = float2::broadcast(a);
    let b = float2::broadcast(b);
    return float3x2(a * x.0 + b * y.0, a * x.1 + b * y.1, a * x.2 + b * y.2);
  }

  #[inline(always)]
  pub fn transpose(self) -> float2x3 {
    let c0 = float3((self.0).0, (self.1).0, (self.2).0);
    let c1 = float3((self.0).1, (self.1).1, (self.2).1);

    return float2x3(c0, c1);
  }
}
