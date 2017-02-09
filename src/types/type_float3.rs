use std;
use ::*;

#[repr(C)]
#[repr(simd)]
#[derive(Copy, Clone, Debug)]
pub struct float3(pub f32, pub f32, pub f32);
pub type vector_float3 = float3;

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

impl std::ops::Index<u32> for float3 {
  type Output = f32;

  #[inline]
  fn index(&self, index: u32) -> &f32 {
    return unsafe { simd_extract(self, index) };
  }
}

impl std::ops::Add for float3 {
  type Output = Self;

  #[inline]
  fn add(self, other: Self) -> Self {
    return unsafe { simd_add(self, other) };
  }
}

impl std::ops::Add<f32> for float3 {
  type Output = Self;

  #[inline]
  fn add(self, other: f32) -> Self {
    return unsafe { simd_add(self, float3::broadcast(other)) };
  }
}

impl std::ops::Add<float3> for f32 {
  type Output = float3;

  #[inline]
  fn add(self, other: float3) -> float3 {
    return unsafe { simd_add(float3::broadcast(self), other) };
  }
}

impl std::ops::Sub for float3 {
  type Output = Self;

  #[inline]
  fn sub(self, other: Self) -> Self {
    return unsafe { simd_sub(self, other) };
  }
}

impl std::ops::Sub<f32> for float3 {
  type Output = Self;

  #[inline]
  fn sub(self, other: f32) -> Self {
    return unsafe { simd_sub(self, float3::broadcast(other)) };
  }
}

impl std::ops::Sub<float3> for f32 {
  type Output = float3;

  #[inline]
  fn sub(self, other: float3) -> float3 {
    return unsafe { simd_sub(float3::broadcast(self), other) };
  }
}

impl std::ops::Mul for float3 {
  type Output = Self;

  #[inline]
  fn mul(self, other: Self) -> Self {
    return unsafe { simd_mul(self, other) };
  }
}

impl std::ops::Mul<f32> for float3 {
  type Output = Self;

  #[inline]
  fn mul(self, other: f32) -> Self {
    return unsafe { simd_mul(self, float3::broadcast(other)) };
  }
}

impl std::ops::Mul<float3> for f32 {
  type Output = float3;

  #[inline]
  fn mul(self, other: float3) -> float3 {
    return unsafe { simd_mul(float3::broadcast(self), other) };
  }
}

impl std::ops::Div for float3 {
  type Output = Self;

  #[inline]
  fn div(self, other: Self) -> Self {
    return unsafe { simd_div(self, other) };
  }
}

impl std::ops::Div<f32> for float3 {
  type Output = Self;

  #[inline]
  fn div(self, other: f32) -> Self {
    return unsafe { simd_div(self, float3::broadcast(other)) };
  }
}

impl std::ops::Div<float3> for f32 {
  type Output = float3;

  #[inline]
  fn div(self, other: float3) -> float3 {
    return unsafe { simd_div(float3::broadcast(self), other) };
  }
}

impl PartialEq for float3 {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    return int3::all(float3::eq(*self, *other));
  }

  #[inline]
  fn ne(&self, other: &Self) -> bool {
    return int3::all(float3::ne(*self, *other));
  }
}

impl Dot for float3 {
  type Output = f32;

  #[inline]
  fn dot(self, other: float3) -> f32 {
    return float3::reduce_add(self * other);
  }
}

impl float3 {
  #[inline]
  pub fn bitcast<T>(x: T) -> float3 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn broadcast(x: f32) -> float3 {
    return float3(x, x, x);
  }

  #[inline]
  pub fn extract(self, i: u32) -> f32 {
    return unsafe { simd_extract(self, i) };
  }

  #[inline]
  pub fn replace(self, i: u32, x: f32) -> float3 {
    return unsafe { simd_insert(self, i, x) };
  }

  #[inline]
  pub fn eq(x: float3, y: float3) -> int3 {
    return unsafe { simd_eq(x, y) };
  }

  #[inline]
  pub fn ne(x: float3, y: float3) -> int3 {
    return unsafe { simd_ne(x, y) };
  }

  #[inline]
  pub fn lt(x: float3, y: float3) -> int3 {
    return unsafe { simd_lt(x, y) };
  }

  #[inline]
  pub fn le(x: float3, y: float3) -> int3 {
    return unsafe { simd_le(x, y) };
  }

  #[inline]
  pub fn gt(x: float3, y: float3) -> int3 {
    return unsafe { simd_gt(x, y) };
  }

  #[inline]
  pub fn ge(x: float3, y: float3) -> int3 {
    return unsafe { simd_ge(x, y) };
  }

  #[inline]
  pub fn abs(x: float3) -> float3 {
    return float3(x.0.abs(), x.1.abs(), x.2.abs());
  }

  #[inline]
  pub fn max(x: float3, y: float3) -> float3 {
    return float3(x.0.max(y.0), x.1.max(y.1), x.2.max(y.2));
  }

  #[inline]
  pub fn min(x: float3, y: float3) -> float3 {
    return float3(x.0.min(y.0), x.1.min(y.1), x.2.min(y.2));
  }

  #[inline]
  pub fn clamp(x: float3, min: float3, max: float3) -> float3 {
    return float3::min(float3::max(x, min), max);
  }

  #[inline]
  pub fn sign(x: float3) -> float3 {
    return float3(if x.0 == 0.0 { 0.0} else { x.0.signum() }, if x.1 == 0.0 { 0.0} else { x.1.signum() }, if x.2 == 0.0 { 0.0} else { x.2.signum() });
  }

  #[inline]
  pub fn mix(x: float3, y: float3, t: float3) -> float3 {
    return x + t * (y - x);
  }

  #[inline]
  pub fn recip(x: float3) -> float3 {
    return float3::broadcast(1.0) / x;
  }

  #[inline]
  pub fn rsqrt(x: float3) -> float3 {
    return float3::broadcast(1.0) / float3::sqrt(x);
  }

  #[inline]
  pub fn fract(x: float3) -> float3 {
    return float3(x.0.fract(), x.1.fract(), x.2.fract());
  }

  #[inline]
  pub fn step(edge: float3, x: float3) -> float3 {
    return float3::bitselect(float3::broadcast(1.0), float3::broadcast(0.0), float3::lt(x, edge));
  }

  #[inline]
  pub fn smoothstep(edge0: float3, edge1: float3, x: float3) -> float3 {
    let t = float3::clamp((x - edge0) / (edge1 - edge0), float3::broadcast(0.0), float3::broadcast(1.0));

    return t * t * (3.0 - 2.0 * t);
  }

  #[inline]
  pub fn reduce_add(x: float3) -> f32 {
    return x.0 + x.1 + x.2;
  }

  #[inline]
  pub fn reduce_min(x: float3) -> f32 {
    return x.2.min(float2::reduce_min(x.lo()));
  }

  #[inline]
  pub fn reduce_max(x: float3) -> f32 {
    return x.2.max(float2::reduce_max(x.lo()));
  }

  #[inline]
  pub fn copysign(x: float3, y: float3) -> float3 {
    return float3::bitselect(y, x, int3::broadcast(std::i32::MAX));
  }

  #[inline]
  pub fn sqrt(x: float3) -> float3 {
    return float3(x.0.sqrt(), x.1.sqrt(), x.2.sqrt());
  }

  #[inline]
  pub fn ceil(x: float3) -> float3 {
    return float3(x.0.ceil(), x.1.ceil(), x.2.ceil());
  }

  #[inline]
  pub fn floor(x: float3) -> float3 {
    return float3(x.0.floor(), x.1.floor(), x.2.floor());
  }

  #[inline]
  pub fn trunc(x: float3) -> float3 {
    return float3(x.0.trunc(), x.1.trunc(), x.2.trunc());
  }

  #[inline]
  pub fn sin(x: float3) -> float3 {
    return float3(x.0.sin(), x.1.sin(), x.2.sin());
  }

  #[inline]
  pub fn cos(x: float3) -> float3 {
    return float3(x.0.cos(), x.1.cos(), x.2.cos());
  }

  #[inline]
  pub fn select(x: float3, y: float3, z: int3) -> float3 {
    return float3::bitselect(x, y, z >> 31);
  }

  #[inline]
  pub fn bitselect(x: float3, y: float3, z: int3) -> float3 {
    return float3::bitcast(int3::bitselect(int3::bitcast(x), int3::bitcast(y), z));
  }

  #[inline]
  pub fn lo(self) -> float2 {
    return float2(self.0, self.1);
  }

  #[inline]
  pub fn hi(self) -> float2 {
    return float2(self.2, 0.0);
  }
}
