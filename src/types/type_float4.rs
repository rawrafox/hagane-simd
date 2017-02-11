use std;
use ::*;

#[repr(C)]
#[repr(simd)]
#[derive(Copy, Clone, Debug)]
pub struct float4(pub f32, pub f32, pub f32, pub f32);

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

  fn simd_cast<T, U>(x: T) -> U;

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
    return simd::all(float4::eq(*self, *other));
  }

  #[inline]
  fn ne(&self, other: &Self) -> bool {
    return simd::all(float4::ne(*self, *other));
  }
}

impl simd::Vector for float4 {
}

impl simd::Dot for float4 {
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
  pub fn madd(x: float4, y: float4, z: float4) -> float4 {
    return x * y + z;
  }

  #[inline]
  pub fn abs(x: float4) -> float4 {
    return float4::bitselect(float4::broadcast(0.0), x, int4::broadcast(std::i32::MAX));
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
    let (zero, one) = (float4::broadcast(0.0), float4::broadcast(1.0));
    return float4::bitselect(float4::copysign(one, x), zero, float4::eq(x, zero) | float4::ne(x, x));
  }

  #[inline]
  pub fn mix(x: float4, y: float4, t: float4) -> float4 {
    return x + t * (y - x);
  }

  #[inline]
  pub fn recip(x: float4) -> float4 {
    return 1.0 / x;
  }

  #[inline]
  pub fn rsqrt(x: float4) -> float4 {
    return 1.0 / float4::sqrt(x);
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
  pub fn copysign(x: float4, y: float4) -> float4 {
    return float4::bitselect(y, x, int4::broadcast(std::i32::MAX));
  }

  #[inline]
  pub fn sqrt(x: float4) -> float4 {
    return float4(x.0.sqrt(), x.1.sqrt(), x.2.sqrt(), x.3.sqrt());
  }

  #[inline]
  pub fn ceil(x: float4) -> float4 {
    return float4(x.0.ceil(), x.1.ceil(), x.2.ceil(), x.3.ceil());
  }

  #[inline]
  pub fn floor(x: float4) -> float4 {
    return float4(x.0.floor(), x.1.floor(), x.2.floor(), x.3.floor());
  }

  #[inline]
  pub fn trunc(x: float4) -> float4 {
    return float4(x.0.trunc(), x.1.trunc(), x.2.trunc(), x.3.trunc());
  }

  #[inline]
  pub fn sin(x: float4) -> float4 {
    return float4(x.0.sin(), x.1.sin(), x.2.sin(), x.3.sin());
  }

  #[inline]
  pub fn cos(x: float4) -> float4 {
    return float4(x.0.cos(), x.1.cos(), x.2.cos(), x.3.cos());
  }

  #[inline]
  pub fn dot(x: float4, y: float4) -> f32 {
    return float4::reduce_add(x * y);
  }

  #[inline]
  pub fn project(x: float4, y: float4) -> float4 {
    return float4::dot(x, y) / float4::dot(y, y) * y;
  }

  #[inline]
  pub fn length(x: float4) -> f32 {
    return float4::length_squared(x).sqrt();
  }

  #[inline]
  pub fn length_squared(x: float4) -> f32 {
    return float4::dot(x, x);
  }

  #[inline]
  pub fn norm_one(x: float4) -> f32 {
    return float4::reduce_add(float4::abs(x));
  }

  #[inline]
  pub fn norm_inf(x: float4) -> f32 {
    return float4::reduce_max(float4::abs(x));
  }

  #[inline]
  pub fn distance(x: float4, y: float4) -> f32 {
    return float4::length(x - y);
  }

  #[inline]
  pub fn distance_squared(x: float4, y: float4) -> f32 {
    return float4::length_squared(x - y);
  }

  #[inline]
  pub fn normalize(x: float4) -> float4 {
    return x * float4::rsqrt(float4::broadcast(float4::length_squared(x)));
  }

  #[inline]
  pub fn reflect(x: float4, n: float4) -> float4 {
    return x - 2.0 * float4::dot(x, n) * n;
  }

  #[inline]
  pub fn refract(x: float4, n: float4, eta: f32) -> float4 {
    let dp = float4::dot(x, n);
    let k = 1.0 - eta * eta * (1.0 - dp * dp);
    return if k >= 0.0 { eta * x - (eta * dp + k.sqrt()) } else { float4::broadcast(0.0) };
  }

  #[inline]
  pub fn to_char(x: float4) -> char4 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_char_sat(x: float4) -> char4 {
    return float4::to_char(float4::clamp(x, float4::broadcast(std::i8::MIN as f32), float4::broadcast(std::i8::MAX as f32)));
  }

  #[inline]
  pub fn to_uchar(x: float4) -> uchar4 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_uchar_sat(x: float4) -> uchar4 {
    return float4::to_uchar(float4::clamp(x, float4::broadcast(std::u8::MIN as f32), float4::broadcast(std::u8::MAX as f32)));
  }

  #[inline]
  pub fn to_short(x: float4) -> short4 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_short_sat(x: float4) -> short4 {
    return float4::to_short(float4::clamp(x, float4::broadcast(std::i16::MIN as f32), float4::broadcast(std::i16::MAX as f32)));
  }

  #[inline]
  pub fn to_ushort(x: float4) -> ushort4 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_ushort_sat(x: float4) -> ushort4 {
    return float4::to_ushort(float4::clamp(x, float4::broadcast(std::u16::MIN as f32), float4::broadcast(std::u16::MAX as f32)));
  }

  #[inline]
  pub fn to_int(x: float4) -> int4 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_int_sat(x: float4) -> int4 {
    return float4::to_int(float4::clamp(x, float4::broadcast(std::i32::MIN as f32), float4::broadcast(std::i32::MAX as f32)));
  }

  #[inline]
  pub fn to_uint(x: float4) -> uint4 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_uint_sat(x: float4) -> uint4 {
    return float4::to_uint(float4::clamp(x, float4::broadcast(std::u32::MIN as f32), float4::broadcast(std::u32::MAX as f32)));
  }

  #[inline]
  pub fn to_float(x: float4) -> float4 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_long(x: float4) -> long4 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_long_sat(x: float4) -> long4 {
    return float4::to_long(float4::clamp(x, float4::broadcast(std::i64::MIN as f32), float4::broadcast(std::i64::MAX as f32)));
  }

  #[inline]
  pub fn to_ulong(x: float4) -> ulong4 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_ulong_sat(x: float4) -> ulong4 {
    return float4::to_ulong(float4::clamp(x, float4::broadcast(std::u64::MIN as f32), float4::broadcast(std::u64::MAX as f32)));
  }

  #[inline]
  pub fn to_double(x: float4) -> double4 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn lo(self) -> float2 {
    return float2(self.0, self.1);
  }

  #[inline]
  pub fn hi(self) -> float2 {
    return float2(self.2, self.3);
  }

  #[inline]
  pub fn odd(self) -> float2 {
    return float2(self.1, self.3);
  }

  #[inline]
  pub fn even(self) -> float2 {
    return float2(self.0, self.2);
  }
}
