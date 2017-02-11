use std;
use ::*;

#[repr(C)]
#[repr(simd)]
#[derive(Copy, Clone, Debug)]
pub struct float2(pub f32, pub f32);

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
    return simd::all(float2::eq(*self, *other));
  }

  #[inline]
  fn ne(&self, other: &Self) -> bool {
    return simd::all(float2::ne(*self, *other));
  }
}

impl simd::Vector for float2 {
  type Scalar = f32;
  #[inline(always)]
  fn extract(self, i: u32) -> Self::Scalar {
    return unsafe { simd_extract(self, i) };
  }

  #[inline(always)]
  fn replace(self, i: u32, x: Self::Scalar) -> Self {
    return unsafe { simd_insert(self, i, x) };
  }

  #[inline(always)]
  fn abs(self) -> Self {
    return simd::bitselect(int2::broadcast(std::i32::MAX), float2::broadcast(0.0), self);
  }

  #[inline(always)]
  fn max(self, other: Self) -> Self {
    return float2(self.0.max(other.0), self.1.max(other.1));
  }

  #[inline(always)]
  fn min(self, other: Self) -> Self {
    return float2(self.0.min(other.0), self.1.min(other.1));
  }
}

impl simd::Dot for float2 {
  type Output = f32;

  #[inline(always)]
  fn dot(self, other: Self) -> Self::Output {
    return simd::reduce_add(self * other);
  }
}

impl simd::Float for float2 {
  #[inline(always)]
  fn sign(self) -> Self {
    let (zero, one) = (float2::broadcast(0.0), float2::broadcast(1.0));

    return simd::bitselect(float2::eq(self, zero) | float2::ne(self, self), float2::copysign(one, self), zero);
  }

  #[inline(always)]
  fn mix(self, a: Self, b: Self) -> Self {
    return a + self * (b - a);
  }

  #[inline(always)]
  fn recip(self) -> Self {
    return 1.0 / self;
  }

  #[inline(always)]
  fn rsqrt(self) -> Self {
    return 1.0 / float2::sqrt(self);
  }

  #[inline(always)]
  fn fract(self) -> Self {
    return float2(self.0.fract(), self.1.fract());
  }

  #[inline(always)]
  fn step(self, edge: Self) -> Self {
    return simd::bitselect(float2::lt(self, edge), float2::broadcast(1.0), float2::broadcast(0.0));
  }

  #[inline(always)]
  fn smoothstep(self, edge0: Self, edge1: Self) -> Self {
    let t = simd::clamp((self - edge0) / (edge1 - edge0), float2::broadcast(0.0), float2::broadcast(1.0));

    return t * t * (3.0 - 2.0 * t);
  }
}

impl simd::Reduce for float2 {
  #[inline(always)]
  fn reduce_add(self) -> Self::Scalar {
    return self.0 + self.1;
  }

  #[inline(always)]
  fn reduce_min(self) -> Self::Scalar {
    return self.0.min(self.1);
  }

  #[inline(always)]
  fn reduce_max(self) -> Self::Scalar {
    return self.0.max(self.1);
  }
}

impl float2 {
  #[inline]
  pub fn bitcast<T>(x: T) -> float2 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn broadcast(x: f32) -> Self {
    return float2(x, x);
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
  pub fn copysign(x: float2, y: float2) -> float2 {
    return simd::bitselect(int2::broadcast(std::i32::MAX), y, x);
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
  pub fn dot(x: float2, y: float2) -> f32 {
    return simd::reduce_add(x * y);
  }

  #[inline]
  pub fn project(x: float2, y: float2) -> float2 {
    return simd::dot(x, y) / simd::dot(y, y) * y;
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
    return simd::reduce_add(simd::abs(x));
  }

  #[inline]
  pub fn norm_inf(x: float2) -> f32 {
    return simd::reduce_max(simd::abs(x));
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
    return x * simd::rsqrt(float2::broadcast(float2::length_squared(x)));
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
  pub fn to_char(x: float2) -> char2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_char_sat(x: float2) -> char2 {
    return float2::to_char(simd::clamp(x, float2::broadcast(std::i8::MIN as f32), float2::broadcast(std::i8::MAX as f32)));
  }

  #[inline]
  pub fn to_uchar(x: float2) -> uchar2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_uchar_sat(x: float2) -> uchar2 {
    return float2::to_uchar(simd::clamp(x, float2::broadcast(std::u8::MIN as f32), float2::broadcast(std::u8::MAX as f32)));
  }

  #[inline]
  pub fn to_short(x: float2) -> short2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_short_sat(x: float2) -> short2 {
    return float2::to_short(simd::clamp(x, float2::broadcast(std::i16::MIN as f32), float2::broadcast(std::i16::MAX as f32)));
  }

  #[inline]
  pub fn to_ushort(x: float2) -> ushort2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_ushort_sat(x: float2) -> ushort2 {
    return float2::to_ushort(simd::clamp(x, float2::broadcast(std::u16::MIN as f32), float2::broadcast(std::u16::MAX as f32)));
  }

  #[inline]
  pub fn to_int(x: float2) -> int2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_int_sat(x: float2) -> int2 {
    return float2::to_int(simd::clamp(x, float2::broadcast(std::i32::MIN as f32), float2::broadcast(std::i32::MAX as f32)));
  }

  #[inline]
  pub fn to_uint(x: float2) -> uint2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_uint_sat(x: float2) -> uint2 {
    return float2::to_uint(simd::clamp(x, float2::broadcast(std::u32::MIN as f32), float2::broadcast(std::u32::MAX as f32)));
  }

  #[inline]
  pub fn to_float(x: float2) -> float2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_long(x: float2) -> long2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_long_sat(x: float2) -> long2 {
    return float2::to_long(simd::clamp(x, float2::broadcast(std::i64::MIN as f32), float2::broadcast(std::i64::MAX as f32)));
  }

  #[inline]
  pub fn to_ulong(x: float2) -> ulong2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_ulong_sat(x: float2) -> ulong2 {
    return float2::to_ulong(simd::clamp(x, float2::broadcast(std::u64::MIN as f32), float2::broadcast(std::u64::MAX as f32)));
  }

  #[inline]
  pub fn to_double(x: float2) -> double2 {
    return unsafe { simd_cast(x) };
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
