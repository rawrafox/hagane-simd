use std;
use ::*;

extern "platform-intrinsic" {
  fn simd_add<T>(x: T, y: T) -> T;
  fn simd_sub<T>(x: T, y: T) -> T;
  fn simd_mul<T>(x: T, y: T) -> T;
  fn simd_div<T>(x: T, y: T) -> T;

  fn simd_cast<T, U>(x: T) -> U;
}

impl std::ops::Add for double2 {
  type Output = Self;

  #[inline]
  fn add(self, other: Self) -> Self {
    return unsafe { simd_add(self, other) };
  }
}

impl std::ops::Add<f64> for double2 {
  type Output = Self;

  #[inline]
  fn add(self, other: f64) -> Self {
    return unsafe { simd_add(self, double2::broadcast(other)) };
  }
}

impl std::ops::Add<double2> for f64 {
  type Output = double2;

  #[inline]
  fn add(self, other: double2) -> double2 {
    return unsafe { simd_add(double2::broadcast(self), other) };
  }
}

impl std::ops::Sub for double2 {
  type Output = Self;

  #[inline]
  fn sub(self, other: Self) -> Self {
    return unsafe { simd_sub(self, other) };
  }
}

impl std::ops::Sub<f64> for double2 {
  type Output = Self;

  #[inline]
  fn sub(self, other: f64) -> Self {
    return unsafe { simd_sub(self, double2::broadcast(other)) };
  }
}

impl std::ops::Sub<double2> for f64 {
  type Output = double2;

  #[inline]
  fn sub(self, other: double2) -> double2 {
    return unsafe { simd_sub(double2::broadcast(self), other) };
  }
}

impl std::ops::Mul for double2 {
  type Output = Self;

  #[inline]
  fn mul(self, other: Self) -> Self {
    return unsafe { simd_mul(self, other) };
  }
}

impl std::ops::Mul<f64> for double2 {
  type Output = Self;

  #[inline]
  fn mul(self, other: f64) -> Self {
    return unsafe { simd_mul(self, double2::broadcast(other)) };
  }
}

impl std::ops::Mul<double2> for f64 {
  type Output = double2;

  #[inline]
  fn mul(self, other: double2) -> double2 {
    return unsafe { simd_mul(double2::broadcast(self), other) };
  }
}

impl std::ops::Div for double2 {
  type Output = Self;

  #[inline]
  fn div(self, other: Self) -> Self {
    return unsafe { simd_div(self, other) };
  }
}

impl std::ops::Div<f64> for double2 {
  type Output = Self;

  #[inline]
  fn div(self, other: f64) -> Self {
    return unsafe { simd_div(self, double2::broadcast(other)) };
  }
}

impl std::ops::Div<double2> for f64 {
  type Output = double2;

  #[inline]
  fn div(self, other: double2) -> double2 {
    return unsafe { simd_div(double2::broadcast(self), other) };
  }
}

impl PartialEq for double2 {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    return simd::all(simd::eq(*self, *other));
  }

  #[inline]
  fn ne(&self, other: &Self) -> bool {
    return simd::all(simd::ne(*self, *other));
  }
}

impl simd::Vector for double2 {
  type Scalar = f64;
  type Boolean = long2;

  #[inline(always)]
  fn abs(self) -> Self {
    return simd::bitselect(long2::broadcast(std::i64::MAX), double2::broadcast(0.0), self);
  }

  #[inline(always)]
  fn max(self, other: Self) -> Self {
    return double2(self.0.max(other.0), self.1.max(other.1));
  }

  #[inline(always)]
  fn min(self, other: Self) -> Self {
    return double2(self.0.min(other.0), self.1.min(other.1));
  }
}

impl simd::Cross for double2 {
  type CrossProduct = double3;

  #[inline(always)]
  fn cross(self, other: Self) -> Self::CrossProduct {
    return double3(0.0, 0.0, self.0 * other.1 - self.1 * other.0);
  }
}

impl simd::Dot for double2 {
  type DotProduct = f64;
  #[inline(always)]
  fn dot(self, other: Self) -> Self::DotProduct {
    return simd::reduce_add(self * other);
  }
}

impl simd::Float for double2 {
  #[inline(always)]
  fn copysign(self, magnitude: Self) -> Self {
    return simd::bitselect(long2::broadcast(std::i64::MAX), magnitude, self);
  }

  #[inline(always)]
  fn sign(self) -> Self {
    let (zero, one) = (double2::broadcast(0.0), double2::broadcast(1.0));

    return simd::bitselect(simd::eq(self, zero) | simd::ne(self, self), one.copysign(self), zero);
  }

  #[inline(always)]
  fn sqrt(self) -> Self {
    return double2(self.0.sqrt(), self.1.sqrt());
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
    return double2(self.0.fract(), self.1.fract());
  }

  #[inline(always)]
  fn ceil(self) -> Self {
    return double2(self.0.ceil(), self.1.ceil());
  }

  #[inline(always)]
  fn floor(self) -> Self {
    return double2(self.0.floor(), self.1.floor());
  }

  #[inline(always)]
  fn trunc(self) -> Self {
    return double2(self.0.trunc(), self.1.trunc());
  }

  #[inline(always)]
  fn mix(self, a: Self, b: Self) -> Self {
    return a + self * (b - a);
  }

  #[inline(always)]
  fn step(self, edge: Self) -> Self {
    return simd::bitselect(simd::lt(self, edge), double2::broadcast(1.0), double2::broadcast(0.0));
  }

  #[inline(always)]
  fn smoothstep(self, edge0: Self, edge1: Self) -> Self {
    let t = simd::clamp((self - edge0) / (edge1 - edge0), double2::broadcast(0.0), double2::broadcast(1.0));

    return t * t * (3.0 - 2.0 * t);
  }

  #[inline(always)]
  fn sin(self) -> Self {
    return double2(self.0.sin(), self.1.sin());
  }

  #[inline(always)]
  fn cos(self) -> Self {
    return double2(self.0.cos(), self.1.cos());
  }
}

impl simd::Reduce for double2 {
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

impl double2 {
  #[inline]
  pub fn bitcast<T>(x: T) -> double2 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn broadcast(x: f64) -> Self {
    return double2(x, x);
  }

  #[inline]
  pub fn madd(x: double2, y: double2, z: double2) -> double2 {
    return x * y + z;
  }

  #[inline]
  pub fn project(x: double2, y: double2) -> double2 {
    return simd::dot(x, y) / simd::dot(y, y) * y;
  }

  #[inline]
  pub fn length(x: double2) -> f64 {
    return double2::length_squared(x).sqrt();
  }

  #[inline]
  pub fn length_squared(x: double2) -> f64 {
    return simd::dot(x, x);
  }

  #[inline]
  pub fn norm_one(x: double2) -> f64 {
    return simd::reduce_add(simd::abs(x));
  }

  #[inline]
  pub fn norm_inf(x: double2) -> f64 {
    return simd::reduce_max(simd::abs(x));
  }

  #[inline]
  pub fn distance(x: double2, y: double2) -> f64 {
    return double2::length(x - y);
  }

  #[inline]
  pub fn distance_squared(x: double2, y: double2) -> f64 {
    return double2::length_squared(x - y);
  }

  #[inline]
  pub fn normalize(x: double2) -> double2 {
    return x * simd::rsqrt(double2::broadcast(double2::length_squared(x)));
  }

  #[inline]
  pub fn reflect(x: double2, n: double2) -> double2 {
    return x - 2.0 * simd::dot(x, n) * n;
  }

  #[inline]
  pub fn refract(x: double2, n: double2, eta: f64) -> double2 {
    let dp = simd::dot(x, n);
    let k = 1.0 - eta * eta * (1.0 - dp * dp);
    return if k >= 0.0 { eta * x - (eta * dp + k.sqrt()) } else { double2::broadcast(0.0) };
  }

  #[inline]
  pub fn to_char(x: double2) -> char2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_char_sat(x: double2) -> char2 {
    return double2::to_char(simd::clamp(x, double2::broadcast(std::i8::MIN as f64), double2::broadcast(std::i8::MAX as f64)));
  }

  #[inline]
  pub fn to_uchar(x: double2) -> uchar2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_uchar_sat(x: double2) -> uchar2 {
    return double2::to_uchar(simd::clamp(x, double2::broadcast(std::u8::MIN as f64), double2::broadcast(std::u8::MAX as f64)));
  }

  #[inline]
  pub fn to_short(x: double2) -> short2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_short_sat(x: double2) -> short2 {
    return double2::to_short(simd::clamp(x, double2::broadcast(std::i16::MIN as f64), double2::broadcast(std::i16::MAX as f64)));
  }

  #[inline]
  pub fn to_ushort(x: double2) -> ushort2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_ushort_sat(x: double2) -> ushort2 {
    return double2::to_ushort(simd::clamp(x, double2::broadcast(std::u16::MIN as f64), double2::broadcast(std::u16::MAX as f64)));
  }

  #[inline]
  pub fn to_int(x: double2) -> int2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_int_sat(x: double2) -> int2 {
    return double2::to_int(simd::clamp(x, double2::broadcast(std::i32::MIN as f64), double2::broadcast(std::i32::MAX as f64)));
  }

  #[inline]
  pub fn to_uint(x: double2) -> uint2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_uint_sat(x: double2) -> uint2 {
    return double2::to_uint(simd::clamp(x, double2::broadcast(std::u32::MIN as f64), double2::broadcast(std::u32::MAX as f64)));
  }

  #[inline]
  pub fn to_float(x: double2) -> float2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_long(x: double2) -> long2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_long_sat(x: double2) -> long2 {
    return double2::to_long(simd::clamp(x, double2::broadcast(std::i64::MIN as f64), double2::broadcast(std::i64::MAX as f64)));
  }

  #[inline]
  pub fn to_ulong(x: double2) -> ulong2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_ulong_sat(x: double2) -> ulong2 {
    return double2::to_ulong(simd::clamp(x, double2::broadcast(std::u64::MIN as f64), double2::broadcast(std::u64::MAX as f64)));
  }

  #[inline]
  pub fn to_double(x: double2) -> double2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn lo(self) -> f64 {
    return self.0;
  }

  #[inline]
  pub fn hi(self) -> f64 {
    return self.1;
  }

  #[inline]
  pub fn odd(self) -> f64 {
    return self.1;
  }

  #[inline]
  pub fn even(self) -> f64 {
    return self.0;
  }
}
