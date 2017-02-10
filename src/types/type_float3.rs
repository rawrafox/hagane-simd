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

  fn simd_cast<T, U>(x: T) -> U;

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
  pub fn madd(x: float3, y: float3, z: float3) -> float3 {
    return x * y + z;
  }

  #[inline]
  pub fn abs(x: float3) -> float3 {
    return float3::bitselect(float3::broadcast(0.0), x, int3::broadcast(std::i32::MAX));
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
    let (zero, one) = (float3::broadcast(0.0), float3::broadcast(1.0));
    return float3::bitselect(float3::copysign(one, x), zero, float3::eq(x, zero) | float3::ne(x, x));
  }

  #[inline]
  pub fn mix(x: float3, y: float3, t: float3) -> float3 {
    return x + t * (y - x);
  }

  #[inline]
  pub fn recip(x: float3) -> float3 {
    return 1.0 / x;
  }

  #[inline]
  pub fn rsqrt(x: float3) -> float3 {
    return 1.0 / float3::sqrt(x);
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
  pub fn dot(x: float3, y: float3) -> f32 {
    return float3::reduce_add(x * y);
  }

  #[inline]
  pub fn project(x: float3, y: float3) -> float3 {
    return float3::dot(x, y) / float3::dot(y, y) * y;
  }

  #[inline]
  pub fn length(x: float3) -> f32 {
    return float3::length_squared(x).sqrt();
  }

  #[inline]
  pub fn length_squared(x: float3) -> f32 {
    return float3::dot(x, x);
  }

  #[inline]
  pub fn norm_one(x: float3) -> f32 {
    return float3::reduce_add(float3::abs(x));
  }

  #[inline]
  pub fn norm_inf(x: float3) -> f32 {
    return float3::reduce_max(float3::abs(x));
  }

  #[inline]
  pub fn distance(x: float3, y: float3) -> f32 {
    return float3::length(x - y);
  }

  #[inline]
  pub fn distance_squared(x: float3, y: float3) -> f32 {
    return float3::length_squared(x - y);
  }

  #[inline]
  pub fn normalize(x: float3) -> float3 {
    return x * float3::rsqrt(float3::broadcast(float3::length_squared(x)));
  }

  #[inline]
  pub fn cross(x: float3, y: float3) -> float3 {
    let a = x * float3(y.2, y.1, y.0) - float3(x.2, x.1, x.0) * y;
    return float3(a.2, a.1, a.0);
  }

  #[inline]
  pub fn reflect(x: float3, n: float3) -> float3 {
    return x - 2.0 * float3::dot(x, n) * n;
  }

  #[inline]
  pub fn refract(x: float3, n: float3, eta: f32) -> float3 {
    let dp = float3::dot(x, n);
    let k = 1.0 - eta * eta * (1.0 - dp * dp);
    return if k >= 0.0 { eta * x - (eta * dp + k.sqrt()) } else { float3::broadcast(0.0) };
  }

  #[inline]
  pub fn to_char(x: float3) -> char3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_char_sat(x: float3) -> char3 {
    return float3::to_char(float3::clamp(x, float3::broadcast(std::i8::MIN as f32), float3::broadcast(std::i8::MAX as f32)));
  }

  #[inline]
  pub fn to_uchar(x: float3) -> uchar3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_uchar_sat(x: float3) -> uchar3 {
    return float3::to_uchar(float3::clamp(x, float3::broadcast(std::u8::MIN as f32), float3::broadcast(std::u8::MAX as f32)));
  }

  #[inline]
  pub fn to_short(x: float3) -> short3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_short_sat(x: float3) -> short3 {
    return float3::to_short(float3::clamp(x, float3::broadcast(std::i16::MIN as f32), float3::broadcast(std::i16::MAX as f32)));
  }

  #[inline]
  pub fn to_ushort(x: float3) -> ushort3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_ushort_sat(x: float3) -> ushort3 {
    return float3::to_ushort(float3::clamp(x, float3::broadcast(std::u16::MIN as f32), float3::broadcast(std::u16::MAX as f32)));
  }

  #[inline]
  pub fn to_int(x: float3) -> int3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_int_sat(x: float3) -> int3 {
    return float3::to_int(float3::clamp(x, float3::broadcast(std::i32::MIN as f32), float3::broadcast(std::i32::MAX as f32)));
  }

  #[inline]
  pub fn to_uint(x: float3) -> uint3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_uint_sat(x: float3) -> uint3 {
    return float3::to_uint(float3::clamp(x, float3::broadcast(std::u32::MIN as f32), float3::broadcast(std::u32::MAX as f32)));
  }

  #[inline]
  pub fn to_float(x: float3) -> float3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_long(x: float3) -> long3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_long_sat(x: float3) -> long3 {
    return float3::to_long(float3::clamp(x, float3::broadcast(std::i64::MIN as f32), float3::broadcast(std::i64::MAX as f32)));
  }

  #[inline]
  pub fn to_ulong(x: float3) -> ulong3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_ulong_sat(x: float3) -> ulong3 {
    return float3::to_ulong(float3::clamp(x, float3::broadcast(std::u64::MIN as f32), float3::broadcast(std::u64::MAX as f32)));
  }

  #[inline]
  pub fn to_double(x: float3) -> double3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn lo(self) -> float2 {
    return float2(self.0, self.1);
  }

  #[inline]
  pub fn hi(self) -> float2 {
    return float2(self.2, 0.0);
  }

  #[inline]
  pub fn odd(self) -> float2 {
    return float2(self.1, 0.0);
  }

  #[inline]
  pub fn even(self) -> float2 {
    return float2(self.0, self.2);
  }
}
