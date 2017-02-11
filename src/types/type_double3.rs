use std;
use ::*;

#[repr(C)]
#[repr(simd)]
#[derive(Copy, Clone, Debug)]
pub struct double3(pub f64, pub f64, pub f64);

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

impl std::ops::Index<u32> for double3 {
  type Output = f64;

  #[inline]
  fn index(&self, index: u32) -> &f64 {
    return unsafe { simd_extract(self, index) };
  }
}

impl std::ops::Add for double3 {
  type Output = Self;

  #[inline]
  fn add(self, other: Self) -> Self {
    return unsafe { simd_add(self, other) };
  }
}

impl std::ops::Add<f64> for double3 {
  type Output = Self;

  #[inline]
  fn add(self, other: f64) -> Self {
    return unsafe { simd_add(self, double3::broadcast(other)) };
  }
}

impl std::ops::Add<double3> for f64 {
  type Output = double3;

  #[inline]
  fn add(self, other: double3) -> double3 {
    return unsafe { simd_add(double3::broadcast(self), other) };
  }
}

impl std::ops::Sub for double3 {
  type Output = Self;

  #[inline]
  fn sub(self, other: Self) -> Self {
    return unsafe { simd_sub(self, other) };
  }
}

impl std::ops::Sub<f64> for double3 {
  type Output = Self;

  #[inline]
  fn sub(self, other: f64) -> Self {
    return unsafe { simd_sub(self, double3::broadcast(other)) };
  }
}

impl std::ops::Sub<double3> for f64 {
  type Output = double3;

  #[inline]
  fn sub(self, other: double3) -> double3 {
    return unsafe { simd_sub(double3::broadcast(self), other) };
  }
}

impl std::ops::Mul for double3 {
  type Output = Self;

  #[inline]
  fn mul(self, other: Self) -> Self {
    return unsafe { simd_mul(self, other) };
  }
}

impl std::ops::Mul<f64> for double3 {
  type Output = Self;

  #[inline]
  fn mul(self, other: f64) -> Self {
    return unsafe { simd_mul(self, double3::broadcast(other)) };
  }
}

impl std::ops::Mul<double3> for f64 {
  type Output = double3;

  #[inline]
  fn mul(self, other: double3) -> double3 {
    return unsafe { simd_mul(double3::broadcast(self), other) };
  }
}

impl std::ops::Div for double3 {
  type Output = Self;

  #[inline]
  fn div(self, other: Self) -> Self {
    return unsafe { simd_div(self, other) };
  }
}

impl std::ops::Div<f64> for double3 {
  type Output = Self;

  #[inline]
  fn div(self, other: f64) -> Self {
    return unsafe { simd_div(self, double3::broadcast(other)) };
  }
}

impl std::ops::Div<double3> for f64 {
  type Output = double3;

  #[inline]
  fn div(self, other: double3) -> double3 {
    return unsafe { simd_div(double3::broadcast(self), other) };
  }
}

impl PartialEq for double3 {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    return simd::all(double3::eq(*self, *other));
  }

  #[inline]
  fn ne(&self, other: &Self) -> bool {
    return simd::all(double3::ne(*self, *other));
  }
}

impl simd::Vector for double3 {
  type Scalar = f64;
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
    return simd::bitselect(long3::broadcast(std::i64::MAX), double3::broadcast(0.0), self);
  }

  #[inline(always)]
  fn max(self, other: Self) -> Self {
    return double3(self.0.max(other.0), self.1.max(other.1), self.2.max(other.2));
  }

  #[inline(always)]
  fn min(self, other: Self) -> Self {
    return double3(self.0.min(other.0), self.1.min(other.1), self.2.min(other.2));
  }
}

impl simd::Dot for double3 {
  type Output = f64;

  #[inline(always)]
  fn dot(self, other: Self) -> Self::Output {
    return simd::reduce_add(self * other);
  }
}

impl simd::Float for double3 {
  #[inline(always)]
  fn sign(self) -> Self {
    let (zero, one) = (double3::broadcast(0.0), double3::broadcast(1.0));

    return simd::bitselect(double3::eq(self, zero) | double3::ne(self, self), double3::copysign(one, self), zero);
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
    return 1.0 / double3::sqrt(self);
  }

  #[inline(always)]
  fn fract(self) -> Self {
    return double3(self.0.fract(), self.1.fract(), self.2.fract());
  }

  #[inline(always)]
  fn step(self, edge: Self) -> Self {
    return simd::bitselect(double3::lt(self, edge), double3::broadcast(1.0), double3::broadcast(0.0));
  }

  #[inline(always)]
  fn smoothstep(self, edge0: Self, edge1: Self) -> Self {
    let t = simd::clamp((self - edge0) / (edge1 - edge0), double3::broadcast(0.0), double3::broadcast(1.0));

    return t * t * (3.0 - 2.0 * t);
  }
}

impl simd::Reduce for double3 {
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

impl double3 {
  #[inline]
  pub fn bitcast<T>(x: T) -> double3 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn broadcast(x: f64) -> Self {
    return double3(x, x, x);
  }

  #[inline]
  pub fn eq(x: double3, y: double3) -> long3 {
    return unsafe { simd_eq(x, y) };
  }

  #[inline]
  pub fn ne(x: double3, y: double3) -> long3 {
    return unsafe { simd_ne(x, y) };
  }

  #[inline]
  pub fn lt(x: double3, y: double3) -> long3 {
    return unsafe { simd_lt(x, y) };
  }

  #[inline]
  pub fn le(x: double3, y: double3) -> long3 {
    return unsafe { simd_le(x, y) };
  }

  #[inline]
  pub fn gt(x: double3, y: double3) -> long3 {
    return unsafe { simd_gt(x, y) };
  }

  #[inline]
  pub fn ge(x: double3, y: double3) -> long3 {
    return unsafe { simd_ge(x, y) };
  }

  #[inline]
  pub fn madd(x: double3, y: double3, z: double3) -> double3 {
    return x * y + z;
  }

  #[inline]
  pub fn copysign(x: double3, y: double3) -> double3 {
    return simd::bitselect(long3::broadcast(std::i64::MAX), y, x);
  }

  #[inline]
  pub fn sqrt(x: double3) -> double3 {
    return double3(x.0.sqrt(), x.1.sqrt(), x.2.sqrt());
  }

  #[inline]
  pub fn ceil(x: double3) -> double3 {
    return double3(x.0.ceil(), x.1.ceil(), x.2.ceil());
  }

  #[inline]
  pub fn floor(x: double3) -> double3 {
    return double3(x.0.floor(), x.1.floor(), x.2.floor());
  }

  #[inline]
  pub fn trunc(x: double3) -> double3 {
    return double3(x.0.trunc(), x.1.trunc(), x.2.trunc());
  }

  #[inline]
  pub fn sin(x: double3) -> double3 {
    return double3(x.0.sin(), x.1.sin(), x.2.sin());
  }

  #[inline]
  pub fn cos(x: double3) -> double3 {
    return double3(x.0.cos(), x.1.cos(), x.2.cos());
  }

  #[inline]
  pub fn dot(x: double3, y: double3) -> f64 {
    return simd::reduce_add(x * y);
  }

  #[inline]
  pub fn project(x: double3, y: double3) -> double3 {
    return simd::dot(x, y) / simd::dot(y, y) * y;
  }

  #[inline]
  pub fn length(x: double3) -> f64 {
    return double3::length_squared(x).sqrt();
  }

  #[inline]
  pub fn length_squared(x: double3) -> f64 {
    return double3::dot(x, x);
  }

  #[inline]
  pub fn norm_one(x: double3) -> f64 {
    return simd::reduce_add(simd::abs(x));
  }

  #[inline]
  pub fn norm_inf(x: double3) -> f64 {
    return simd::reduce_max(simd::abs(x));
  }

  #[inline]
  pub fn distance(x: double3, y: double3) -> f64 {
    return double3::length(x - y);
  }

  #[inline]
  pub fn distance_squared(x: double3, y: double3) -> f64 {
    return double3::length_squared(x - y);
  }

  #[inline]
  pub fn normalize(x: double3) -> double3 {
    return x * simd::rsqrt(double3::broadcast(double3::length_squared(x)));
  }

  #[inline]
  pub fn cross(x: double3, y: double3) -> double3 {
    let a = x * double3(y.2, y.1, y.0) - double3(x.2, x.1, x.0) * y;
    return double3(a.2, a.1, a.0);
  }

  #[inline]
  pub fn reflect(x: double3, n: double3) -> double3 {
    return x - 2.0 * double3::dot(x, n) * n;
  }

  #[inline]
  pub fn refract(x: double3, n: double3, eta: f64) -> double3 {
    let dp = double3::dot(x, n);
    let k = 1.0 - eta * eta * (1.0 - dp * dp);
    return if k >= 0.0 { eta * x - (eta * dp + k.sqrt()) } else { double3::broadcast(0.0) };
  }

  #[inline]
  pub fn to_char(x: double3) -> char3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_char_sat(x: double3) -> char3 {
    return double3::to_char(simd::clamp(x, double3::broadcast(std::i8::MIN as f64), double3::broadcast(std::i8::MAX as f64)));
  }

  #[inline]
  pub fn to_uchar(x: double3) -> uchar3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_uchar_sat(x: double3) -> uchar3 {
    return double3::to_uchar(simd::clamp(x, double3::broadcast(std::u8::MIN as f64), double3::broadcast(std::u8::MAX as f64)));
  }

  #[inline]
  pub fn to_short(x: double3) -> short3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_short_sat(x: double3) -> short3 {
    return double3::to_short(simd::clamp(x, double3::broadcast(std::i16::MIN as f64), double3::broadcast(std::i16::MAX as f64)));
  }

  #[inline]
  pub fn to_ushort(x: double3) -> ushort3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_ushort_sat(x: double3) -> ushort3 {
    return double3::to_ushort(simd::clamp(x, double3::broadcast(std::u16::MIN as f64), double3::broadcast(std::u16::MAX as f64)));
  }

  #[inline]
  pub fn to_int(x: double3) -> int3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_int_sat(x: double3) -> int3 {
    return double3::to_int(simd::clamp(x, double3::broadcast(std::i32::MIN as f64), double3::broadcast(std::i32::MAX as f64)));
  }

  #[inline]
  pub fn to_uint(x: double3) -> uint3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_uint_sat(x: double3) -> uint3 {
    return double3::to_uint(simd::clamp(x, double3::broadcast(std::u32::MIN as f64), double3::broadcast(std::u32::MAX as f64)));
  }

  #[inline]
  pub fn to_float(x: double3) -> float3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_long(x: double3) -> long3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_long_sat(x: double3) -> long3 {
    return double3::to_long(simd::clamp(x, double3::broadcast(std::i64::MIN as f64), double3::broadcast(std::i64::MAX as f64)));
  }

  #[inline]
  pub fn to_ulong(x: double3) -> ulong3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_ulong_sat(x: double3) -> ulong3 {
    return double3::to_ulong(simd::clamp(x, double3::broadcast(std::u64::MIN as f64), double3::broadcast(std::u64::MAX as f64)));
  }

  #[inline]
  pub fn to_double(x: double3) -> double3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn lo(self) -> double2 {
    return double2(self.0, self.1);
  }

  #[inline]
  pub fn hi(self) -> double2 {
    return double2(self.2, 0.0);
  }

  #[inline]
  pub fn odd(self) -> double2 {
    return double2(self.1, 0.0);
  }

  #[inline]
  pub fn even(self) -> double2 {
    return double2(self.0, self.2);
  }
}
