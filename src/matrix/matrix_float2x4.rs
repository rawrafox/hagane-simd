use std;
use ::*;

impl std::ops::Add for float2x4 {
  type Output = Self;

  #[inline(always)]
  fn add(self, other: Self) -> Self {
    return float2x4(self.0 + other.0, self.1 + other.1);
  }
}

impl std::ops::Sub for float2x4 {
  type Output = Self;

  #[inline(always)]
  fn sub(self, other: Self) -> Self {
    return float2x4(self.0 - other.0, self.1 - other.1);
  }
}

impl std::ops::Mul<f32> for float2x4 {
  type Output = Self;

  #[inline(always)]
  fn mul(self, other: f32) -> Self {
    let a = float4::broadcast(other);

    return float2x4(a * self.0, a * self.1);
  }
}

impl float2x4 {
  #[inline(always)]
  pub fn linear_combination(a: f32, x: float2x4, b: f32, y: float2x4) -> float2x4 {
    let a = float4::broadcast(a);
    let b = float4::broadcast(b);
    return float2x4(a * x.0 + b * y.0, a * x.1 + b * y.1);
  }

  #[inline(always)]
  pub fn transpose(self) -> float4x2 {
    let c0 = float2((self.0).0, (self.1).0);
    let c1 = float2((self.0).1, (self.1).1);
    let c2 = float2((self.0).2, (self.1).2);
    let c3 = float2((self.0).3, (self.1).3);

    return float4x2(c0, c1, c2, c3);
  }
}
