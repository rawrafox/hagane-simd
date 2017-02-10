use std;
use ::*;

#[repr(C)]
#[repr(simd)]
#[derive(Copy, Clone, Debug)]
pub struct float2(pub f32, pub f32);
pub type vector_float2 = float2;

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

impl std::ops::Index<u32> for float2 {
  type Output = f32;

  #[inline]
  fn index(&self, index: u32) -> &f32 {
    return unsafe { simd_extract(self, index) };
  }
}

impl std::ops::Add for float2 {
  type Output = Self;

  #[inline]
  fn add(self, other: Self) -> Self {
    return unsafe { simd_add(self, other) };
  }
}

impl std::ops::Add<f32> for float2 {
  type Output = Self;

  #[inline]
  fn add(self, other: f32) -> Self {
    return unsafe { simd_add(self, float2::broadcast(other)) };
  }
}

impl std::ops::Add<float2> for f32 {
  type Output = float2;

  #[inline]
  fn add(self, other: float2) -> float2 {
    return unsafe { simd_add(float2::broadcast(self), other) };
  }
}

impl std::ops::Sub for float2 {
  type Output = Self;

  #[inline]
  fn sub(self, other: Self) -> Self {
    return unsafe { simd_sub(self, other) };
  }
}

impl std::ops::Sub<f32> for float2 {
  type Output = Self;

  #[inline]
  fn sub(self, other: f32) -> Self {
    return unsafe { simd_sub(self, float2::broadcast(other)) };
  }
}

impl std::ops::Sub<float2> for f32 {
  type Output = float2;

  #[inline]
  fn sub(self, other: float2) -> float2 {
    return unsafe { simd_sub(float2::broadcast(self), other) };
  }
}

impl std::ops::Mul for float2 {
  type Output = Self;

  #[inline]
  fn mul(self, other: Self) -> Self {
    return unsafe { simd_mul(self, other) };
  }
}

impl std::ops::Mul<f32> for float2 {
  type Output = Self;

  #[inline]
  fn mul(self, other: f32) -> Self {
    return unsafe { simd_mul(self, float2::broadcast(other)) };
  }
}

impl std::ops::Mul<float2> for f32 {
  type Output = float2;

  #[inline]
  fn mul(self, other: float2) -> float2 {
    return unsafe { simd_mul(float2::broadcast(self), other) };
  }
}

impl std::ops::Div for float2 {
  type Output = Self;

  #[inline]
  fn div(self, other: Self) -> Self {
    return unsafe { simd_div(self, other) };
  }
}

impl std::ops::Div<f32> for float2 {
  type Output = Self;

  #[inline]
  fn div(self, other: f32) -> Self {
    return unsafe { simd_div(self, float2::broadcast(other)) };
  }
}

impl std::ops::Div<float2> for f32 {
  type Output = float2;

  #[inline]
  fn div(self, other: float2) -> float2 {
    return unsafe { simd_div(float2::broadcast(self), other) };
  }
}

impl PartialEq for float2 {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    return int2::all(float2::eq(*self, *other));
  }

  #[inline]
  fn ne(&self, other: &Self) -> bool {
    return int2::all(float2::ne(*self, *other));
  }
}

impl Dot for float2 {
  type Output = f32;

  #[inline]
  fn dot(self, other: float2) -> f32 {
    return float2::reduce_add(self * other);
  }
}

impl float2 {
  #[inline]
  pub fn bitcast<T>(x: T) -> float2 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn broadcast(x: f32) -> float2 {
    return float2(x, x);
  }

  #[inline]
  pub fn extract(self, i: u32) -> f32 {
    return unsafe { simd_extract(self, i) };
  }

  #[inline]
  pub fn replace(self, i: u32, x: f32) -> float2 {
    return unsafe { simd_insert(self, i, x) };
  }

  #[inline]
  pub fn eq(x: float2, y: float2) -> int2 {
    return unsafe { simd_eq(x, y) };
  }

  #[inline]
  pub fn ne(x: float2, y: float2) -> int2 {
    return unsafe { simd_ne(x, y) };
  }

  #[inline]
  pub fn lt(x: float2, y: float2) -> int2 {
    return unsafe { simd_lt(x, y) };
  }

  #[inline]
  pub fn le(x: float2, y: float2) -> int2 {
    return unsafe { simd_le(x, y) };
  }

  #[inline]
  pub fn gt(x: float2, y: float2) -> int2 {
    return unsafe { simd_gt(x, y) };
  }

  #[inline]
  pub fn ge(x: float2, y: float2) -> int2 {
    return unsafe { simd_ge(x, y) };
  }

  #[inline]
  pub fn madd(x: float2, y: float2, z: float2) -> float2 {
    return x * y + z;
  }

  #[inline]
  pub fn abs(x: float2) -> float2 {
    return float2(x.0.abs(), x.1.abs());
  }

  #[inline]
  pub fn max(x: float2, y: float2) -> float2 {
    return float2(x.0.max(y.0), x.1.max(y.1));
  }

  #[inline]
  pub fn min(x: float2, y: float2) -> float2 {
    return float2(x.0.min(y.0), x.1.min(y.1));
  }

  #[inline]
  pub fn clamp(x: float2, min: float2, max: float2) -> float2 {
    return float2::min(float2::max(x, min), max);
  }

  #[inline]
  pub fn sign(x: float2) -> float2 {
    let (zero, one) = (float2::broadcast(0.0), float2::broadcast(1.0));
    return float2::bitselect(float2::copysign(one, x), zero, float2::eq(x, zero) | float2::ne(x, x));
  }

  #[inline]
  pub fn mix(x: float2, y: float2, t: float2) -> float2 {
    return x + t * (y - x);
  }

  #[inline]
  pub fn recip(x: float2) -> float2 {
    return 1.0 / x;
  }

  #[inline]
  pub fn rsqrt(x: float2) -> float2 {
    return 1.0 / float2::sqrt(x);
  }

  #[inline]
  pub fn fract(x: float2) -> float2 {
    return float2(x.0.fract(), x.1.fract());
  }

  #[inline]
  pub fn step(edge: float2, x: float2) -> float2 {
    return float2::bitselect(float2::broadcast(1.0), float2::broadcast(0.0), float2::lt(x, edge));
  }

  #[inline]
  pub fn smoothstep(edge0: float2, edge1: float2, x: float2) -> float2 {
    let t = float2::clamp((x - edge0) / (edge1 - edge0), float2::broadcast(0.0), float2::broadcast(1.0));

    return t * t * (3.0 - 2.0 * t);
  }

  #[inline]
  pub fn reduce_add(x: float2) -> f32 {
    return x.0 + x.1;
  }

  #[inline]
  pub fn reduce_min(x: float2) -> f32 {
    return x.0.min(x.1);
  }

  #[inline]
  pub fn reduce_max(x: float2) -> f32 {
    return x.0.max(x.1);
  }

  #[inline]
  pub fn copysign(x: float2, y: float2) -> float2 {
    return float2::bitselect(y, x, int2::broadcast(std::i32::MAX));
  }

  #[inline]
  pub fn sqrt(x: float2) -> float2 {
    return float2(x.0.sqrt(), x.1.sqrt());
  }

  #[inline]
  pub fn ceil(x: float2) -> float2 {
    return float2(x.0.ceil(), x.1.ceil());
  }

  #[inline]
  pub fn floor(x: float2) -> float2 {
    return float2(x.0.floor(), x.1.floor());
  }

  #[inline]
  pub fn trunc(x: float2) -> float2 {
    return float2(x.0.trunc(), x.1.trunc());
  }

  #[inline]
  pub fn sin(x: float2) -> float2 {
    return float2(x.0.sin(), x.1.sin());
  }

  #[inline]
  pub fn cos(x: float2) -> float2 {
    return float2(x.0.cos(), x.1.cos());
  }

  #[inline]
  pub fn select(x: float2, y: float2, z: int2) -> float2 {
    return float2::bitselect(x, y, z >> 31);
  }

  #[inline]
  pub fn bitselect(x: float2, y: float2, z: int2) -> float2 {
    return float2::bitcast(int2::bitselect(int2::bitcast(x), int2::bitcast(y), z));
  }

  #[inline]
  pub fn dot(x: float2, y: float2) -> f32 {
    return float2::reduce_add(x * y);
  }

  #[inline]
  pub fn project(x: float2, y: float2) -> float2 {
    return float2::dot(x, y) / float2::dot(y, y) * y;
  }

  #[inline]
  pub fn length(x: float2) -> f32 {
    return float2::length_squared(x).sqrt();
  }

  #[inline]
  pub fn length_squared(x: float2) -> f32 {
    return float2::dot(x, x);
  }

  #[inline]
  pub fn norm_one(x: float2) -> f32 {
    return float2::reduce_add(float2::abs(x));
  }

  #[inline]
  pub fn norm_inf(x: float2) -> f32 {
    return float2::reduce_max(float2::abs(x));
  }

  #[inline]
  pub fn distance(x: float2, y: float2) -> f32 {
    return float2::length(x - y);
  }

  #[inline]
  pub fn distance_squared(x: float2, y: float2) -> f32 {
    return float2::length_squared(x - y);
  }

  #[inline]
  pub fn normalize(x: float2) -> float2 {
    return x * float2::rsqrt(float2::broadcast(float2::length_squared(x)));
  }

  #[inline]
  pub fn cross(x: float2, y: float2) -> float3 {
    return float3(0.0, 0.0, x.0 * y.1 - x.1 * y.0);
  }

  #[inline]
  pub fn reflect(x: float2, n: float2) -> float2 {
    return x - 2.0 * float2::dot(x, n) * n;
  }

  #[inline]
  pub fn refract(x: float2, n: float2, eta: f32) -> float2 {
    let dp = float2::dot(x, n);
    let k = 1.0 - eta * eta * (1.0 - dp * dp);
    return if k >= 0.0 { eta * x - (eta * dp + k.sqrt()) } else { float2::broadcast(0.0) };
  }

  #[inline]
  pub fn lo(self) -> f32 {
    return self.0;
  }

  #[inline]
  pub fn hi(self) -> f32 {
    return self.1;
  }

  #[inline]
  pub fn odd(self) -> f32 {
    return self.1;
  }

  #[inline]
  pub fn even(self) -> f32 {
    return self.0;
  }
}
