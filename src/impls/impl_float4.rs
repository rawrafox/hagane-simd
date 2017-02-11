use std;
use ::*;

extern "platform-intrinsic" {
  fn simd_add<T>(x: T, y: T) -> T;
  fn simd_sub<T>(x: T, y: T) -> T;
  fn simd_mul<T>(x: T, y: T) -> T;
  fn simd_div<T>(x: T, y: T) -> T;

  fn simd_cast<T, U>(x: T) -> U;
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
    return simd::all(simd::eq(*self, *other));
  }

  #[inline]
  fn ne(&self, other: &Self) -> bool {
    return simd::all(simd::ne(*self, *other));
  }
}

impl simd::Vector for float4 {
  type Scalar = f32;
  type Boolean = int4;

  #[inline(always)]
  fn abs(self) -> Self {
    return simd::bitselect(int4::broadcast(std::i32::MAX), float4::broadcast(0.0), self);
  }

  #[inline(always)]
  fn max(self, other: Self) -> Self {
    return float4(self.0.max(other.0), self.1.max(other.1), self.2.max(other.2), self.3.max(other.3));
  }

  #[inline(always)]
  fn min(self, other: Self) -> Self {
    return float4(self.0.min(other.0), self.1.min(other.1), self.2.min(other.2), self.3.min(other.3));
  }
}

impl simd::Dot for float4 {
  type DotProduct = f32;
  #[inline(always)]
  fn dot(self, other: Self) -> Self::DotProduct {
    return simd::reduce_add(self * other);
  }
}

impl simd::Float for float4 {
  #[inline(always)]
  fn copysign(self, magnitude: Self) -> Self {
    return simd::bitselect(int4::broadcast(std::i32::MAX), magnitude, self);
  }

  #[inline(always)]
  fn sign(self) -> Self {
    let (zero, one) = (float4::broadcast(0.0), float4::broadcast(1.0));

    return simd::bitselect(simd::eq(self, zero) | simd::ne(self, self), one.copysign(self), zero);
  }

  #[inline(always)]
  fn sqrt(self) -> Self {
    return float4(self.0.sqrt(), self.1.sqrt(), self.2.sqrt(), self.3.sqrt());
  }

  #[inline(always)]
  fn recip(self) -> Self {
    return 1.0 / self;
  }

  #[inline(always)]
  fn rsqrt(self) -> Self {
    return self.sqrt().recip();
  }

  #[inline(always)]
  fn fract(self) -> Self {
    return float4(self.0.fract(), self.1.fract(), self.2.fract(), self.3.fract());
  }

  #[inline(always)]
  fn ceil(self) -> Self {
    return float4(self.0.ceil(), self.1.ceil(), self.2.ceil(), self.3.ceil());
  }

  #[inline(always)]
  fn floor(self) -> Self {
    return float4(self.0.floor(), self.1.floor(), self.2.floor(), self.3.floor());
  }

  #[inline(always)]
  fn trunc(self) -> Self {
    return float4(self.0.trunc(), self.1.trunc(), self.2.trunc(), self.3.trunc());
  }

  #[inline(always)]
  fn mix(self, a: Self, b: Self) -> Self {
    return a + self * (b - a);
  }

  #[inline(always)]
  fn step(self, edge: Self) -> Self {
    return simd::bitselect(simd::lt(self, edge), float4::broadcast(1.0), float4::broadcast(0.0));
  }

  #[inline(always)]
  fn smoothstep(self, edge0: Self, edge1: Self) -> Self {
    let t = simd::clamp((self - edge0) / (edge1 - edge0), float4::broadcast(0.0), float4::broadcast(1.0));

    return t * t * (3.0 - 2.0 * t);
  }

  #[inline(always)]
  fn sin(self) -> Self {
    return float4(self.0.sin(), self.1.sin(), self.2.sin(), self.3.sin());
  }

  #[inline(always)]
  fn cos(self) -> Self {
    return float4(self.0.cos(), self.1.cos(), self.2.cos(), self.3.cos());
  }
}

impl simd::Reduce for float4 {
  #[inline(always)]
  fn reduce_add(self) -> Self::Scalar {
    return simd::reduce_add(self.lo() + self.hi());
  }

  #[inline(always)]
  fn reduce_min(self) -> Self::Scalar {
    return simd::reduce_min(simd::min(self.lo(), self.hi()));
  }

  #[inline(always)]
  fn reduce_max(self) -> Self::Scalar {
    return simd::reduce_max(simd::max(self.lo(), self.hi()));
  }
}

impl float4 {
  #[inline]
  pub fn bitcast<T>(x: T) -> float4 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn broadcast(x: f32) -> Self {
    return float4(x, x, x, x);
  }

  #[inline]
  pub fn madd(x: float4, y: float4, z: float4) -> float4 {
    return x * y + z;
  }

  #[inline]
  pub fn project(x: float4, y: float4) -> float4 {
    return simd::dot(x, y) / simd::dot(y, y) * y;
  }

  #[inline]
  pub fn length(x: float4) -> f32 {
    return float4::length_squared(x).sqrt();
  }

  #[inline]
  pub fn length_squared(x: float4) -> f32 {
    return simd::dot(x, x);
  }

  #[inline]
  pub fn norm_one(x: float4) -> f32 {
    return simd::reduce_add(simd::abs(x));
  }

  #[inline]
  pub fn norm_inf(x: float4) -> f32 {
    return simd::reduce_max(simd::abs(x));
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
    return x * simd::rsqrt(float4::broadcast(float4::length_squared(x)));
  }

  #[inline]
  pub fn reflect(x: float4, n: float4) -> float4 {
    return x - 2.0 * simd::dot(x, n) * n;
  }

  #[inline]
  pub fn refract(x: float4, n: float4, eta: f32) -> float4 {
    let dp = simd::dot(x, n);
    let k = 1.0 - eta * eta * (1.0 - dp * dp);
    return if k >= 0.0 { eta * x - (eta * dp + k.sqrt()) } else { float4::broadcast(0.0) };
  }

  #[inline]
  pub fn to_char(x: float4) -> char4 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_char_sat(x: float4) -> char4 {
    return float4::to_char(simd::clamp(x, float4::broadcast(std::i8::MIN as f32), float4::broadcast(std::i8::MAX as f32)));
  }

  #[inline]
  pub fn to_uchar(x: float4) -> uchar4 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_uchar_sat(x: float4) -> uchar4 {
    return float4::to_uchar(simd::clamp(x, float4::broadcast(std::u8::MIN as f32), float4::broadcast(std::u8::MAX as f32)));
  }

  #[inline]
  pub fn to_short(x: float4) -> short4 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_short_sat(x: float4) -> short4 {
    return float4::to_short(simd::clamp(x, float4::broadcast(std::i16::MIN as f32), float4::broadcast(std::i16::MAX as f32)));
  }

  #[inline]
  pub fn to_ushort(x: float4) -> ushort4 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_ushort_sat(x: float4) -> ushort4 {
    return float4::to_ushort(simd::clamp(x, float4::broadcast(std::u16::MIN as f32), float4::broadcast(std::u16::MAX as f32)));
  }

  #[inline]
  pub fn to_int(x: float4) -> int4 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_int_sat(x: float4) -> int4 {
    return float4::to_int(simd::clamp(x, float4::broadcast(std::i32::MIN as f32), float4::broadcast(std::i32::MAX as f32)));
  }

  #[inline]
  pub fn to_uint(x: float4) -> uint4 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_uint_sat(x: float4) -> uint4 {
    return float4::to_uint(simd::clamp(x, float4::broadcast(std::u32::MIN as f32), float4::broadcast(std::u32::MAX as f32)));
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
    return float4::to_long(simd::clamp(x, float4::broadcast(std::i64::MIN as f32), float4::broadcast(std::i64::MAX as f32)));
  }

  #[inline]
  pub fn to_ulong(x: float4) -> ulong4 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_ulong_sat(x: float4) -> ulong4 {
    return float4::to_ulong(simd::clamp(x, float4::broadcast(std::u64::MIN as f32), float4::broadcast(std::u64::MAX as f32)));
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
