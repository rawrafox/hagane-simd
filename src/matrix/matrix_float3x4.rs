use std;
use ::*;

impl std::ops::Add for float3x4 {
  type Output = Self;

  #[inline(always)]
  fn add(self, other: Self) -> Self {
    return float3x4(self.0 + other.0, self.1 + other.1, self.2 + other.2);
  }
}

impl std::ops::Sub for float3x4 {
  type Output = Self;

  #[inline(always)]
  fn sub(self, other: Self) -> Self {
    return float3x4(self.0 - other.0, self.1 - other.1, self.2 - other.2);
  }
}

impl std::ops::Mul<float4x3> for float3x4 {
  type Output = float4x4;

  #[inline(always)]
  fn mul(self, other: float4x3) -> Self::Output {
    return self.dot(other);
  }
}

impl std::ops::Mul<float3> for float3x4 {
  type Output = float4;

  #[inline(always)]
  fn mul(self, other: float3) -> Self::Output {
    return self.dot(other);
  }
}

impl std::ops::Mul<f32> for float3x4 {
  type Output = Self;

  #[inline(always)]
  fn mul(self, other: f32) -> Self {
    let a = float4::broadcast(other);

    return float3x4(a * self.0, a * self.1, a * self.2);
  }
}

impl Dot<float4x3> for float3x4 {
  type DotProduct = float4x4;

  #[inline(always)]
  fn dot(self, other: float4x3) -> Self::DotProduct {
    return float4x4(self.dot(other.0), self.dot(other.1), self.dot(other.2), self.dot(other.3));
  }
}

impl Dot<float3> for float3x4 {
  type DotProduct = float4;

  #[inline(always)]
  fn dot(self, other: float3) -> Self::DotProduct {
    return self.0 * other.0 + self.1 * other.1 + self.2 * other.2;
  }
}

impl PartialEq for float3x4 {
  #[inline]
  fn eq(&self, other: &float3x4) -> bool {
    return (self.0.eq(other.0) & self.1.eq(other.1) & self.2.eq(other.2)).all()
  }
}

impl float3x4 {
  #[inline(always)]
  pub fn from_columns(c0: float4, c1: float4, c2: float4) -> float3x4 {
    return float3x4(c0, c1, c2);
  }

  #[inline(always)]
  pub fn from_rows(r0: float3, r1: float3, r2: float3, r3: float3) -> float3x4 {
    return float4x3(r0, r1, r2, r3).transpose();
  }

  #[inline(always)]
  pub fn linear_combination(a: f32, x: float3x4, b: f32, y: float3x4) -> float3x4 {
    let a = float4::broadcast(a);
    let b = float4::broadcast(b);
    return float3x4(a * x.0 + b * y.0, a * x.1 + b * y.1, a * x.2 + b * y.2);
  }

  #[inline(always)]
  pub fn transpose(self) -> float4x3 {
    let c0 = float3((self.0).0, (self.1).0, (self.2).0);
    let c1 = float3((self.0).1, (self.1).1, (self.2).1);
    let c2 = float3((self.0).2, (self.1).2, (self.2).2);
    let c3 = float3((self.0).3, (self.1).3, (self.2).3);

    return float4x3(c0, c1, c2, c3);
  }
}
