use std;
use ::*;
use ::simd::*;

extern "platform-intrinsic" {
  fn simd_add<T>(x: T, y: T) -> T;
  fn simd_sub<T>(x: T, y: T) -> T;
  fn simd_mul<T>(x: T, y: T) -> T;
  fn simd_div<T>(x: T, y: T) -> T;
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
    return simd::eq(*self, *other).all();
  }

  #[inline]
  fn ne(&self, other: &Self) -> bool {
    return simd::ne(*self, *other).all();
  }
}

impl simd::Vector for float2 {
  type Scalar = f32;
  type Boolean = int2;

  type CharVector = char2;
  type ShortVector = short2;
  type IntVector = int2;
  type LongVector = long2;

  type UCharVector = uchar2;
  type UShortVector = ushort2;
  type UIntVector = uint2;
  type ULongVector = ulong2;

  type FloatVector = float2;
  type DoubleVector = double2;

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

  #[inline(always)]
  fn to_char_sat(self) -> char2 {
    return float2::to_char(simd::clamp(self, float2::broadcast(std::i8::MIN as f32), float2::broadcast(std::i8::MAX as f32)));
  }

  #[inline(always)]
  fn to_uchar_sat(self) -> uchar2 {
    return float2::to_uchar(simd::clamp(self, float2::broadcast(std::u8::MIN as f32), float2::broadcast(std::u8::MAX as f32)));
  }

  #[inline(always)]
  fn to_short_sat(self) -> short2 {
    return float2::to_short(simd::clamp(self, float2::broadcast(std::i16::MIN as f32), float2::broadcast(std::i16::MAX as f32)));
  }

  #[inline(always)]
  fn to_ushort_sat(self) -> ushort2 {
    return float2::to_ushort(simd::clamp(self, float2::broadcast(std::u16::MIN as f32), float2::broadcast(std::u16::MAX as f32)));
  }

  #[inline(always)]
  fn to_int_sat(self) -> int2 {
    return float2::to_int(simd::clamp(self, float2::broadcast(std::i32::MIN as f32), float2::broadcast(std::i32::MAX as f32)));
  }

  #[inline(always)]
  fn to_uint_sat(self) -> uint2 {
    return float2::to_uint(simd::clamp(self, float2::broadcast(std::u32::MIN as f32), float2::broadcast(std::u32::MAX as f32)));
  }

  #[inline(always)]
  fn to_long_sat(self) -> long2 {
    return float2::to_long(simd::clamp(self, float2::broadcast(std::i64::MIN as f32), float2::broadcast(std::i64::MAX as f32)));
  }

  #[inline(always)]
  fn to_ulong_sat(self) -> ulong2 {
    return float2::to_ulong(simd::clamp(self, float2::broadcast(std::u64::MIN as f32), float2::broadcast(std::u64::MAX as f32)));
  }
}

impl simd::Cross for float2 {
  type CrossProduct = float3;

  #[inline(always)]
  fn cross(self, other: Self) -> Self::CrossProduct {
    return float3(0.0, 0.0, self.0 * other.1 - self.1 * other.0);
  }
}

impl simd::Dot for float2 {
  type DotProduct = f32;
  #[inline(always)]
  fn dot(self, other: Self) -> Self::DotProduct {
    return simd::reduce_add(self * other);
  }
}

impl simd::Float for float2 {
  #[inline(always)]
  fn copysign(self, magnitude: Self) -> Self {
    return simd::bitselect(int2::broadcast(std::i32::MAX), magnitude, self);
  }

  #[inline(always)]
  fn sign(self) -> Self {
    let (zero, one) = (float2::broadcast(0.0), float2::broadcast(1.0));

    return simd::bitselect(simd::eq(self, zero) | simd::ne(self, self), one.copysign(self), zero);
  }

  #[inline(always)]
  fn sqrt(self) -> Self {
    return float2(self.0.sqrt(), self.1.sqrt());
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
    return float2(self.0.fract(), self.1.fract());
  }

  #[inline(always)]
  fn ceil(self) -> Self {
    return float2(self.0.ceil(), self.1.ceil());
  }

  #[inline(always)]
  fn floor(self) -> Self {
    return float2(self.0.floor(), self.1.floor());
  }

  #[inline(always)]
  fn trunc(self) -> Self {
    return float2(self.0.trunc(), self.1.trunc());
  }

  #[inline(always)]
  fn mix(self, a: Self, b: Self) -> Self {
    return a + self * (b - a);
  }

  #[inline(always)]
  fn step(self, edge: Self) -> Self {
    return simd::bitselect(simd::lt(self, edge), float2::broadcast(1.0), float2::broadcast(0.0));
  }

  #[inline(always)]
  fn smoothstep(self, edge0: Self, edge1: Self) -> Self {
    let t = simd::clamp((self - edge0) / (edge1 - edge0), float2::broadcast(0.0), float2::broadcast(1.0));

    return t * t * (3.0 - 2.0 * t);
  }

  #[inline(always)]
  fn sin(self) -> Self {
    return float2(self.0.sin(), self.1.sin());
  }

  #[inline(always)]
  fn cos(self) -> Self {
    return float2(self.0.cos(), self.1.cos());
  }
}

impl simd::Geometry for float2 {
  #[inline(always)]
  fn project(self, onto: Self) -> Self {
    return (self.dot(onto) / onto.dot(onto)) * onto;
  }

  #[inline(always)]
  fn length(self) -> Self::Scalar {
    return self.length_squared().sqrt();
  }

  #[inline(always)]
  fn length_squared(self) -> Self::Scalar {
    return self.dot(self);
  }

  #[inline(always)]
  fn norm_one(self) -> Self::Scalar {
    return self.abs().reduce_add();
  }

  #[inline(always)]
  fn norm_inf(self) -> Self::Scalar {
    return self.abs().reduce_max();
  }

  #[inline(always)]
  fn distance(self, other: Self) -> Self::Scalar {
    return (self - other).length();
  }

  #[inline(always)]
  fn distance_squared(self, other: Self) -> Self::Scalar {
    return (self - other).length_squared();
  }

  #[inline(always)]
  fn normalize(self) -> Self {
    return self * simd::rsqrt(float2::broadcast(self.length_squared()));
  }

  #[inline(always)]
  fn reflect(self, n: Self) -> Self {
    return self - 2.0 * self.dot(n) * n;
  }

  #[inline(always)]
  fn refract(self, n: Self, eta: Self::Scalar) -> Self {
    let dp = self.dot(n);
    let k = 1.0 - eta * eta * (1.0 - dp * dp);

    return if k >= 0.0 { eta * self - (eta * dp + k.sqrt()) } else { float2::broadcast(0.0) };
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
  pub fn madd(x: float2, y: float2, z: float2) -> float2 {
    return x * y + z;
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
