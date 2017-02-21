use std;
use ::*;

extern {
  fn __invert_f2(a: float2x2) -> float2x2;
}

impl std::ops::Add for float2x2 {
  type Output = Self;

  #[inline(always)]
  fn add(self, other: Self) -> Self {
    return float2x2(self.0 + other.0, self.1 + other.1);
  }
}

impl std::ops::Sub for float2x2 {
  type Output = Self;

  #[inline(always)]
  fn sub(self, other: Self) -> Self {
    return float2x2(self.0 - other.0, self.1 - other.1);
  }
}

impl std::ops::Mul<float2x2> for float2x2 {
  type Output = float2x2;

  #[inline(always)]
  fn mul(self, other: float2x2) -> Self::Output {
    return self.dot(other);
  }
}

impl std::ops::Mul<float2> for float2x2 {
  type Output = float2;

  #[inline(always)]
  fn mul(self, other: float2) -> Self::Output {
    return self.dot(other);
  }
}

impl std::ops::Mul<f32> for float2x2 {
  type Output = Self;

  #[inline(always)]
  fn mul(self, other: f32) -> Self {
    let a = float2::broadcast(other);

    return float2x2(a * self.0, a * self.1);
  }
}

impl Dot<float2x2> for float2x2 {
  type DotProduct = float2x2;

  #[inline(always)]
  fn dot(self, other: float2x2) -> Self::DotProduct {
    return float2x2(self.dot(other.0), self.dot(other.1));
  }
}

impl Dot<float2> for float2x2 {
  type DotProduct = float2;

  #[inline(always)]
  fn dot(self, other: float2) -> Self::DotProduct {
    return self.0 * other.0 + self.1 * other.1;
  }
}

impl PartialEq for float2x2 {
  #[inline]
  fn eq(&self, other: &float2x2) -> bool {
    return (self.0.eq(other.0) & self.1.eq(other.1)).all()
  }
}

impl float2x2 {
  #[inline(always)]
  pub fn from_columns(c0: float2, c1: float2) -> float2x2 {
    return float2x2(c0, c1);
  }

  #[inline(always)]
  pub fn from_rows(r0: float2, r1: float2) -> float2x2 {
    return float2x2(r0, r1).transpose();
  }

  #[inline(always)]
  pub fn identity(self) -> float2x2 {
    return float2x2(float2(1.0, 0.0), float2(0.0, 1.0));
  }

  #[inline(always)]
  pub fn linear_combination(a: f32, x: float2x2, b: f32, y: float2x2) -> float2x2 {
    let a = float2::broadcast(a);
    let b = float2::broadcast(b);
    return float2x2(a * x.0 + b * y.0, a * x.1 + b * y.1);
  }

  #[inline(always)]
  pub fn transpose(self) -> float2x2 {
    let c0 = float2((self.0).0, (self.1).0);
    let c1 = float2((self.0).1, (self.1).1);

    return float2x2(c0, c1);
  }

  #[inline(always)]
  pub fn inverse(self) -> float2x2 {
    return unsafe { __invert_f2(self) };
  }
}
