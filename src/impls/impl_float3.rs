use std;
use ::*;

extern "platform-intrinsic" {
  fn simd_add<T>(x: T, y: T) -> T;
  fn simd_sub<T>(x: T, y: T) -> T;
  fn simd_mul<T>(x: T, y: T) -> T;
  fn simd_div<T>(x: T, y: T) -> T;

  fn simd_cast<T, U>(x: T) -> U;
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
    return simd::all(simd::eq(*self, *other));
  }

  #[inline]
  fn ne(&self, other: &Self) -> bool {
    return simd::all(simd::ne(*self, *other));
  }
}

impl simd::Vector for float3 {
  type Scalar = f32;
  type Boolean = int3;

  #[inline(always)]
  fn abs(self) -> Self {
    return simd::bitselect(int3::broadcast(std::i32::MAX), float3::broadcast(0.0), self);
  }

  #[inline(always)]
  fn max(self, other: Self) -> Self {
    return float3(self.0.max(other.0), self.1.max(other.1), self.2.max(other.2));
  }

  #[inline(always)]
  fn min(self, other: Self) -> Self {
    return float3(self.0.min(other.0), self.1.min(other.1), self.2.min(other.2));
  }
}

impl simd::Dot for float3 {
  type DotProduct = f32;

  #[inline(always)]
  fn dot(self, other: Self) -> Self::DotProduct {
    return simd::reduce_add(self * other);
  }
}

impl simd::Float for float3 {
  #[inline(always)]
  fn copysign(self, magnitude: Self) -> Self {
    return simd::bitselect(int3::broadcast(std::i32::MAX), magnitude, self);
  }

  #[inline(always)]
  fn sign(self) -> Self {
    let (zero, one) = (float3::broadcast(0.0), float3::broadcast(1.0));

    return simd::bitselect(simd::eq(self, zero) | simd::ne(self, self), one.copysign(self), zero);
  }

  #[inline(always)]
  fn sqrt(self) -> Self {
    return float3(self.0.sqrt(), self.1.sqrt(), self.2.sqrt());
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
    return float3(self.0.fract(), self.1.fract(), self.2.fract());
  }

  #[inline(always)]
  fn ceil(self) -> Self {
    return float3(self.0.ceil(), self.1.ceil(), self.2.ceil());
  }

  #[inline(always)]
  fn floor(self) -> Self {
    return float3(self.0.floor(), self.1.floor(), self.2.floor());
  }

  #[inline(always)]
  fn trunc(self) -> Self {
    return float3(self.0.trunc(), self.1.trunc(), self.2.trunc());
  }

  #[inline(always)]
  fn mix(self, a: Self, b: Self) -> Self {
    return a + self * (b - a);
  }

  #[inline(always)]
  fn step(self, edge: Self) -> Self {
    return simd::bitselect(simd::lt(self, edge), float3::broadcast(1.0), float3::broadcast(0.0));
  }

  #[inline(always)]
  fn smoothstep(self, edge0: Self, edge1: Self) -> Self {
    let t = simd::clamp((self - edge0) / (edge1 - edge0), float3::broadcast(0.0), float3::broadcast(1.0));

    return t * t * (3.0 - 2.0 * t);
  }

  #[inline(always)]
  fn sin(self) -> Self {
    return float3(self.0.sin(), self.1.sin(), self.2.sin());
  }

  #[inline(always)]
  fn cos(self) -> Self {
    return float3(self.0.cos(), self.1.cos(), self.2.cos());
  }
}

impl simd::Reduce for float3 {
  #[inline(always)]
  fn reduce_add(self) -> Self::Scalar {
    return self.0 + self.1 + self.2;
  }

  #[inline(always)]
  fn reduce_min(self) -> Self::Scalar {
    return self.2.min(simd::reduce_min(self.lo()));
  }

  #[inline(always)]
  fn reduce_max(self) -> Self::Scalar {
    return self.2.max(simd::reduce_max(self.lo()));
  }
}

impl float3 {
  #[inline]
  pub fn bitcast<T>(x: T) -> float3 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn broadcast(x: f32) -> Self {
    return float3(x, x, x);
  }

  #[inline]
  pub fn madd(x: float3, y: float3, z: float3) -> float3 {
    return x * y + z;
  }

  #[inline]
  pub fn dot(x: float3, y: float3) -> f32 {
    return simd::reduce_add(x * y);
  }

  #[inline]
  pub fn project(x: float3, y: float3) -> float3 {
    return simd::dot(x, y) / simd::dot(y, y) * y;
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
    return simd::reduce_add(simd::abs(x));
  }

  #[inline]
  pub fn norm_inf(x: float3) -> f32 {
    return simd::reduce_max(simd::abs(x));
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
    return x * simd::rsqrt(float3::broadcast(float3::length_squared(x)));
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
    return float3::to_char(simd::clamp(x, float3::broadcast(std::i8::MIN as f32), float3::broadcast(std::i8::MAX as f32)));
  }

  #[inline]
  pub fn to_uchar(x: float3) -> uchar3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_uchar_sat(x: float3) -> uchar3 {
    return float3::to_uchar(simd::clamp(x, float3::broadcast(std::u8::MIN as f32), float3::broadcast(std::u8::MAX as f32)));
  }

  #[inline]
  pub fn to_short(x: float3) -> short3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_short_sat(x: float3) -> short3 {
    return float3::to_short(simd::clamp(x, float3::broadcast(std::i16::MIN as f32), float3::broadcast(std::i16::MAX as f32)));
  }

  #[inline]
  pub fn to_ushort(x: float3) -> ushort3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_ushort_sat(x: float3) -> ushort3 {
    return float3::to_ushort(simd::clamp(x, float3::broadcast(std::u16::MIN as f32), float3::broadcast(std::u16::MAX as f32)));
  }

  #[inline]
  pub fn to_int(x: float3) -> int3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_int_sat(x: float3) -> int3 {
    return float3::to_int(simd::clamp(x, float3::broadcast(std::i32::MIN as f32), float3::broadcast(std::i32::MAX as f32)));
  }

  #[inline]
  pub fn to_uint(x: float3) -> uint3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_uint_sat(x: float3) -> uint3 {
    return float3::to_uint(simd::clamp(x, float3::broadcast(std::u32::MIN as f32), float3::broadcast(std::u32::MAX as f32)));
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
    return float3::to_long(simd::clamp(x, float3::broadcast(std::i64::MIN as f32), float3::broadcast(std::i64::MAX as f32)));
  }

  #[inline]
  pub fn to_ulong(x: float3) -> ulong3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_ulong_sat(x: float3) -> ulong3 {
    return float3::to_ulong(simd::clamp(x, float3::broadcast(std::u64::MIN as f32), float3::broadcast(std::u64::MAX as f32)));
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
