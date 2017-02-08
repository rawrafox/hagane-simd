use std;
use ::*;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct float4x3(pub float3, pub float3, pub float3, pub float3);
pub type matrix_float4x3 = float4x3;

impl std::ops::Add for float4x3 {
  type Output = Self;

  #[inline]
  fn add(self, other: Self) -> Self {
    return float4x3::add(self, other);
  }
}

impl std::ops::Sub for float4x3 {
  type Output = Self;

  #[inline]
  fn sub(self, other: Self) -> Self {
    return float4x3::sub(self, other);
  }
}

impl std::ops::Mul<f32> for float4x3 {
  type Output = Self;

  #[inline]
  fn mul(self, other: f32) -> Self {
    return float4x3::scale(other, self);
  }
}

impl float4x3 {
  #[inline]
  pub fn scale(a: f32, x: float4x3) -> float4x3 {
    let a = float3::broadcast(a);

    return float4x3(a * x.0, a * x.1, a * x.2, a * x.3);
  }

  #[inline]
  pub fn linear_combination(a: f32, x: float4x3, b: f32, y: float4x3) -> float4x3 {
    let a = float3::broadcast(a);
    let b = float3::broadcast(b);
    return float4x3(a * x.0 + b * y.0, a * x.1 + b * y.1, a * x.2 + b * y.2, a * x.3 + b * y.3);
  }

  #[inline]
  pub fn add(x: float4x3, y: float4x3) -> float4x3 {
    return float4x3(x.0 + y.0, x.1 + y.1, x.2 + y.2, x.3 + y.3);
  }

  #[inline]
  pub fn sub(x: float4x3, y: float4x3) -> float4x3 {
    return float4x3(x.0 - y.0, x.1 - y.1, x.2 - y.2, x.3 - y.3);
  }

  #[inline]
  pub fn transpose(self) -> float3x4 {
    let c0 = float4((self.0).0, (self.1).0, (self.2).0, (self.3).0);
    let c1 = float4((self.0).1, (self.1).1, (self.2).1, (self.3).1);
    let c2 = float4((self.0).2, (self.1).2, (self.2).2, (self.3).2);

    return float3x4(c0, c1, c2);
  }
}
