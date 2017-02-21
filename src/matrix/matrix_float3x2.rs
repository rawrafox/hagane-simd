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

impl std::ops::Mul<float2x3> for float3x2 {
  type Output = float2x2;

  #[inline(always)]
  fn mul(self, other: float2x3) -> Self::Output {
    return self.dot(other);
  }
}

impl std::ops::Mul<float3> for float3x2 {
  type Output = float2;

  #[inline(always)]
  fn mul(self, other: float3) -> Self::Output {
    return self.dot(other);
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

impl Dot<float2x3> for float3x2 {
  type DotProduct = float2x2;

  #[inline(always)]
  fn dot(self, other: float2x3) -> Self::DotProduct {
    return float2x2(self.dot(other.0), self.dot(other.1));
  }
}

impl Dot<float3> for float3x2 {
  type DotProduct = float2;

  #[inline(always)]
  fn dot(self, other: float3) -> Self::DotProduct {
    return self.0 * other.0 + self.1 * other.1 + self.2 * other.2;
  }
}

impl PartialEq for float3x2 {
  #[inline]
  fn eq(&self, other: &float3x2) -> bool {
    return (self.0.eq(other.0) & self.1.eq(other.1) & self.2.eq(other.2)).all()
  }
}

impl float3x2 {
  #[inline(always)]
  pub fn from_columns(c0: float2, c1: float2, c2: float2) -> float3x2 {
    return float3x2(c0, c1, c2);
  }

  #[inline(always)]
  pub fn from_rows(r0: float3, r1: float3) -> float3x2 {
    return float2x3(r0, r1).transpose();
  }

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
