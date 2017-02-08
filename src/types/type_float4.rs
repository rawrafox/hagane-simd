use std;
use ::*;

#[repr(C)]
#[repr(simd)]
#[derive(Copy, Clone, Debug)]
pub struct float4(pub f32, pub f32, pub f32, pub f32);
pub type vector_float4 = float4;

extern "platform-intrinsic" {
  fn simd_add<T>(x: T, y: T) -> T;
  fn simd_sub<T>(x: T, y: T) -> T;
  fn simd_mul<T>(x: T, y: T) -> T;
  fn simd_div<T>(x: T, y: T) -> T;

  fn simd_eq<T, U>(x: T, y: T) -> U;
  fn simd_ne<T, U>(x: T, y: T) -> U;
  fn simd_lt<T, U>(x: T, y: T) -> U;
  fn simd_le<T, U>(x: T, y: T) -> U;
  fn simd_gt<T, U>(x: T, y: T) -> U;
  fn simd_ge<T, U>(x: T, y: T) -> U;

  fn simd_insert<T, E>(x: T, i: u32, e: E) -> T;
  fn simd_extract<T, E>(x: T, i: u32) -> E;
}

impl std::ops::Index<u32> for float4 {
  type Output = f32;

  #[inline]
  fn index(&self, index: u32) -> &f32 {
    return unsafe { simd_extract(self, index) };
  }
}

impl std::ops::Add for float4 {
  type Output = Self;

  #[inline]
  fn add(self, other: Self) -> Self {
    return unsafe { simd_add(self, other) };
  }
}

impl std::ops::Add<f32> for float4 {
  type Output = Self;

  #[inline]
  fn add(self, other: f32) -> Self {
    return unsafe { simd_add(self, float4::broadcast(other)) };
  }
}

impl std::ops::Add<float4> for f32 {
  type Output = float4;

  #[inline]
  fn add(self, other: float4) -> float4 {
    return unsafe { simd_add(float4::broadcast(self), other) };
  }
}

impl std::ops::Sub for float4 {
  type Output = Self;

  #[inline]
  fn sub(self, other: Self) -> Self {
    return unsafe { simd_sub(self, other) };
  }
}

impl std::ops::Sub<f32> for float4 {
  type Output = Self;

  #[inline]
  fn sub(self, other: f32) -> Self {
    return unsafe { simd_sub(self, float4::broadcast(other)) };
  }
}

impl std::ops::Sub<float4> for f32 {
  type Output = float4;

  #[inline]
  fn sub(self, other: float4) -> float4 {
    return unsafe { simd_sub(float4::broadcast(self), other) };
  }
}

impl std::ops::Mul for float4 {
  type Output = Self;

  #[inline]
  fn mul(self, other: Self) -> Self {
    return unsafe { simd_mul(self, other) };
  }
}

impl std::ops::Mul<f32> for float4 {
  type Output = Self;

  #[inline]
  fn mul(self, other: f32) -> Self {
    return unsafe { simd_mul(self, float4::broadcast(other)) };
  }
}

impl std::ops::Mul<float4> for f32 {
  type Output = float4;

  #[inline]
  fn mul(self, other: float4) -> float4 {
    return unsafe { simd_mul(float4::broadcast(self), other) };
  }
}

impl std::ops::Div for float4 {
  type Output = Self;

  #[inline]
  fn div(self, other: Self) -> Self {
    return unsafe { simd_div(self, other) };
  }
}

impl std::ops::Div<f32> for float4 {
  type Output = Self;

  #[inline]
  fn div(self, other: f32) -> Self {
    return unsafe { simd_div(self, float4::broadcast(other)) };
  }
}

impl std::ops::Div<float4> for f32 {
  type Output = float4;

  #[inline]
  fn div(self, other: float4) -> float4 {
    return unsafe { simd_div(float4::broadcast(self), other) };
  }
}

impl PartialEq for float4 {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    return int4::all(float4::eq(*self, *other));
  }

  #[inline]
  fn ne(&self, other: &Self) -> bool {
    return int4::all(float4::ne(*self, *other));
  }
}

impl Dot for float4 {
  type Output = f32;

  #[inline]
  fn dot(self, other: float4) -> f32 {
    return float4::reduce_add(self * other);
  }
}

impl float4 {
  #[inline]
  pub fn bitcast<T>(x: T) -> float4 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn broadcast(x: f32) -> float4 {
    return float4(x, x, x, x);
  }

  #[inline]
  pub fn extract(self, i: u32) -> f32 {
    return unsafe { simd_extract(self, i) };
  }

  #[inline]
  pub fn replace(self, i: u32, x: f32) -> float4 {
    return unsafe { simd_insert(self, i, x) };
  }

  #[inline]
  pub fn abs(x: float4) -> float4 {
    return float4(x.0.abs(), x.1.abs(), x.2.abs(), x.3.abs());
  }

  #[inline]
  pub fn max(x: float4, y: float4) -> float4 {
    return float4(x.0.max(y.0), x.1.max(y.1), x.2.max(y.2), x.3.max(y.3));
  }

  #[inline]
  pub fn min(x: float4, y: float4) -> float4 {
    return float4(x.0.min(y.0), x.1.min(y.1), x.2.min(y.2), x.3.min(y.3));
  }

  #[inline]
  pub fn clamp(x: float4, min: float4, max: float4) -> float4 {
    return float4::min(float4::max(x, min), max);
  }

  #[inline]
  pub fn sign(x: float4) -> float4 {
    return float4(if x.0 == 0.0 { 0.0} else { x.0.signum() }, if x.1 == 0.0 { 0.0} else { x.1.signum() }, if x.2 == 0.0 { 0.0} else { x.2.signum() }, if x.3 == 0.0 { 0.0} else { x.3.signum() });
  }

  #[inline]
  pub fn mix(x: float4, y: float4, t: float4) -> float4 {
    return x + t * (y - x);
  }

  #[inline]
  pub fn recip(x: float4) -> float4 {
    return float4::broadcast(1.0) / x;
  }

  #[inline]
  pub fn rsqrt(x: float4) -> float4 {
    return float4(1.0 / x.0.sqrt(), 1.0 / x.1.sqrt(), 1.0 / x.2.sqrt(), 1.0 / x.3.sqrt());
  }

  #[inline]
  pub fn fract(x: float4) -> float4 {
    return float4(x.0.fract(), x.1.fract(), x.2.fract(), x.3.fract());
  }

  #[inline]
  pub fn step(edge: float4, x: float4) -> float4 {
    return float4::bitselect(float4::broadcast(1.0), float4::broadcast(0.0), float4::lt(x, edge));
  }

  #[inline]
  pub fn smoothstep(edge0: float4, edge1: float4, x: float4) -> float4 {
    let t = float4::clamp((x - edge0) / (edge1 - edge0), float4::broadcast(0.0), float4::broadcast(1.0));

    return t * t * (3.0 - 2.0 * t);
  }

  #[inline]
  pub fn reduce_add(x: float4) -> f32 {
    return float2::reduce_add(x.lo() + x.hi());
  }

  #[inline]
  pub fn reduce_min(x: float4) -> f32 {
    return float2::reduce_min(float2::min(x.lo(), x.hi()));
  }

  #[inline]
  pub fn reduce_max(x: float4) -> f32 {
    return float2::reduce_max(float2::max(x.lo(), x.hi()));
  }

  #[inline]
  pub fn eq(x: float4, y: float4) -> int4 {
    return unsafe { simd_eq(x, y) };
  }

  #[inline]
  pub fn ne(x: float4, y: float4) -> int4 {
    return unsafe { simd_ne(x, y) };
  }

  #[inline]
  pub fn lt(x: float4, y: float4) -> int4 {
    return unsafe { simd_lt(x, y) };
  }

  #[inline]
  pub fn le(x: float4, y: float4) -> int4 {
    return unsafe { simd_le(x, y) };
  }

  #[inline]
  pub fn gt(x: float4, y: float4) -> int4 {
    return unsafe { simd_gt(x, y) };
  }

  #[inline]
  pub fn ge(x: float4, y: float4) -> int4 {
    return unsafe { simd_ge(x, y) };
  }

  #[inline]
  pub fn select(x: float4, y: float4, z: int4) -> float4 {
    return float4::bitselect(x, y, z >> 31);
  }

  #[inline]
  pub fn bitselect(x: float4, y: float4, z: int4) -> float4 {
    return float4::bitcast(int4::bitselect(int4::bitcast(x), int4::bitcast(y), z));
  }

  #[inline]
  pub fn lo(self) -> float2 {
    return float2(self.0, self.1);
  }

  #[inline]
  pub fn hi(self) -> float2 {
    return float2(self.2, self.3);
  }
}
