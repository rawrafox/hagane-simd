use std;
use ::*;
use ::simd::*;

extern "platform-intrinsic" {
  fn simd_add<T>(x: T, y: T) -> T;
  fn simd_sub<T>(x: T, y: T) -> T;
  fn simd_mul<T>(x: T, y: T) -> T;
  fn simd_div<T>(x: T, y: T) -> T;
}

impl std::ops::Add for double4 {
  type Output = Self;

  #[inline]
  fn add(self, other: Self) -> Self {
    return unsafe { simd_add(self, other) };
  }
}

impl std::ops::Add<f64> for double4 {
  type Output = Self;

  #[inline]
  fn add(self, other: f64) -> Self {
    return unsafe { simd_add(self, double4::broadcast(other)) };
  }
}

impl std::ops::Add<double4> for f64 {
  type Output = double4;

  #[inline]
  fn add(self, other: double4) -> double4 {
    return unsafe { simd_add(double4::broadcast(self), other) };
  }
}

impl std::ops::Sub for double4 {
  type Output = Self;

  #[inline]
  fn sub(self, other: Self) -> Self {
    return unsafe { simd_sub(self, other) };
  }
}

impl std::ops::Sub<f64> for double4 {
  type Output = Self;

  #[inline]
  fn sub(self, other: f64) -> Self {
    return unsafe { simd_sub(self, double4::broadcast(other)) };
  }
}

impl std::ops::Sub<double4> for f64 {
  type Output = double4;

  #[inline]
  fn sub(self, other: double4) -> double4 {
    return unsafe { simd_sub(double4::broadcast(self), other) };
  }
}

impl std::ops::Mul for double4 {
  type Output = Self;

  #[inline]
  fn mul(self, other: Self) -> Self {
    return unsafe { simd_mul(self, other) };
  }
}

impl std::ops::Mul<f64> for double4 {
  type Output = Self;

  #[inline]
  fn mul(self, other: f64) -> Self {
    return unsafe { simd_mul(self, double4::broadcast(other)) };
  }
}

impl std::ops::Mul<double4> for f64 {
  type Output = double4;

  #[inline]
  fn mul(self, other: double4) -> double4 {
    return unsafe { simd_mul(double4::broadcast(self), other) };
  }
}

impl std::ops::Div for double4 {
  type Output = Self;

  #[inline]
  fn div(self, other: Self) -> Self {
    return unsafe { simd_div(self, other) };
  }
}

impl std::ops::Div<f64> for double4 {
  type Output = Self;

  #[inline]
  fn div(self, other: f64) -> Self {
    return unsafe { simd_div(self, double4::broadcast(other)) };
  }
}

impl std::ops::Div<double4> for f64 {
  type Output = double4;

  #[inline]
  fn div(self, other: double4) -> double4 {
    return unsafe { simd_div(double4::broadcast(self), other) };
  }
}

impl PartialEq for double4 {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    return simd::eq(*self, *other).all();
  }

  #[inline]
  fn ne(&self, other: &Self) -> bool {
    return simd::ne(*self, *other).all();
  }
}

impl simd::Vector for double4 {
  type Scalar = f64;
  type Boolean = long4;

  type CharVector = char4;
  type ShortVector = short4;
  type IntVector = int4;
  type LongVector = long4;

  type UCharVector = uchar4;
  type UShortVector = ushort4;
  type UIntVector = uint4;
  type ULongVector = ulong4;

  type FloatVector = float4;
  type DoubleVector = double4;

  #[inline(always)]
  fn abs(self) -> Self {
    return simd::bitselect(long4::broadcast(std::i64::MAX), double4::broadcast(0.0), self);
  }

  #[inline(always)]
  fn max(self, other: Self) -> Self {
    return double4(self.0.max(other.0), self.1.max(other.1), self.2.max(other.2), self.3.max(other.3));
  }

  #[inline(always)]
  fn min(self, other: Self) -> Self {
    return double4(self.0.min(other.0), self.1.min(other.1), self.2.min(other.2), self.3.min(other.3));
  }

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

  #[inline(always)]
  fn to_char_sat(self) -> char4 {
    return double4::to_char(simd::clamp(self, double4::broadcast(std::i8::MIN as f64), double4::broadcast(std::i8::MAX as f64)));
  }

  #[inline(always)]
  fn to_uchar_sat(self) -> uchar4 {
    return double4::to_uchar(simd::clamp(self, double4::broadcast(std::u8::MIN as f64), double4::broadcast(std::u8::MAX as f64)));
  }

  #[inline(always)]
  fn to_short_sat(self) -> short4 {
    return double4::to_short(simd::clamp(self, double4::broadcast(std::i16::MIN as f64), double4::broadcast(std::i16::MAX as f64)));
  }

  #[inline(always)]
  fn to_ushort_sat(self) -> ushort4 {
    return double4::to_ushort(simd::clamp(self, double4::broadcast(std::u16::MIN as f64), double4::broadcast(std::u16::MAX as f64)));
  }

  #[inline(always)]
  fn to_int_sat(self) -> int4 {
    return double4::to_int(simd::clamp(self, double4::broadcast(std::i32::MIN as f64), double4::broadcast(std::i32::MAX as f64)));
  }

  #[inline(always)]
  fn to_uint_sat(self) -> uint4 {
    return double4::to_uint(simd::clamp(self, double4::broadcast(std::u32::MIN as f64), double4::broadcast(std::u32::MAX as f64)));
  }

  #[inline(always)]
  fn to_long_sat(self) -> long4 {
    return double4::to_long(simd::clamp(self, double4::broadcast(std::i64::MIN as f64), double4::broadcast(std::i64::MAX as f64)));
  }

  #[inline(always)]
  fn to_ulong_sat(self) -> ulong4 {
    return double4::to_ulong(simd::clamp(self, double4::broadcast(std::u64::MIN as f64), double4::broadcast(std::u64::MAX as f64)));
  }
}

impl simd::Dot for double4 {
  type DotProduct = f64;
  #[inline(always)]
  fn dot(self, other: Self) -> Self::DotProduct {
    return simd::reduce_add(self * other);
  }
}

impl simd::Float for double4 {
  #[inline(always)]
  fn copysign(self, magnitude: Self) -> Self {
    return simd::bitselect(long4::broadcast(std::i64::MAX), magnitude, self);
  }

  #[inline(always)]
  fn sign(self) -> Self {
    let (zero, one) = (double4::broadcast(0.0), double4::broadcast(1.0));

    return simd::bitselect(simd::eq(self, zero) | simd::ne(self, self), one.copysign(self), zero);
  }

  #[inline(always)]
  fn sqrt(self) -> Self {
    return double4(self.0.sqrt(), self.1.sqrt(), self.2.sqrt(), self.3.sqrt());
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
    return double4(self.0.fract(), self.1.fract(), self.2.fract(), self.3.fract());
  }

  #[inline(always)]
  fn ceil(self) -> Self {
    return double4(self.0.ceil(), self.1.ceil(), self.2.ceil(), self.3.ceil());
  }

  #[inline(always)]
  fn floor(self) -> Self {
    return double4(self.0.floor(), self.1.floor(), self.2.floor(), self.3.floor());
  }

  #[inline(always)]
  fn trunc(self) -> Self {
    return double4(self.0.trunc(), self.1.trunc(), self.2.trunc(), self.3.trunc());
  }

  #[inline(always)]
  fn mix(self, a: Self, b: Self) -> Self {
    return a + self * (b - a);
  }

  #[inline(always)]
  fn step(self, edge: Self) -> Self {
    return simd::bitselect(simd::lt(self, edge), double4::broadcast(1.0), double4::broadcast(0.0));
  }

  #[inline(always)]
  fn smoothstep(self, edge0: Self, edge1: Self) -> Self {
    let t = simd::clamp((self - edge0) / (edge1 - edge0), double4::broadcast(0.0), double4::broadcast(1.0));

    return t * t * (3.0 - 2.0 * t);
  }

  #[inline(always)]
  fn sin(self) -> Self {
    return double4(self.0.sin(), self.1.sin(), self.2.sin(), self.3.sin());
  }

  #[inline(always)]
  fn cos(self) -> Self {
    return double4(self.0.cos(), self.1.cos(), self.2.cos(), self.3.cos());
  }
}

impl simd::Geometry for double4 {
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
    return self * simd::rsqrt(double4::broadcast(self.length_squared()));
  }

  #[inline(always)]
  fn reflect(self, n: Self) -> Self {
    return self - 2.0 * self.dot(n) * n;
  }

  #[inline(always)]
  fn refract(self, n: Self, eta: Self::Scalar) -> Self {
    let dp = self.dot(n);
    let k = 1.0 - eta * eta * (1.0 - dp * dp);

    return if k >= 0.0 { eta * self - (eta * dp + k.sqrt()) } else { double4::broadcast(0.0) };
  }
}

impl double4 {
  #[inline]
  pub fn bitcast<T>(x: T) -> double4 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn broadcast(x: f64) -> Self {
    return double4(x, x, x, x);
  }

  #[inline]
  pub fn lo(self) -> double2 {
    return double2(self.0, self.1);
  }

  #[inline]
  pub fn hi(self) -> double2 {
    return double2(self.2, self.3);
  }

  #[inline]
  pub fn odd(self) -> double2 {
    return double2(self.1, self.3);
  }

  #[inline]
  pub fn even(self) -> double2 {
    return double2(self.0, self.2);
  }
}
