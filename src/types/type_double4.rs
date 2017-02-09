use std;
use ::*;

#[repr(C)]
#[repr(simd)]
#[derive(Copy, Clone, Debug)]
pub struct double4(pub f64, pub f64, pub f64, pub f64);
pub type vector_double4 = double4;

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

impl std::ops::Index<u32> for double4 {
  type Output = f64;

  #[inline]
  fn index(&self, index: u32) -> &f64 {
    return unsafe { simd_extract(self, index) };
  }
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
    return long4::all(double4::eq(*self, *other));
  }

  #[inline]
  fn ne(&self, other: &Self) -> bool {
    return long4::all(double4::ne(*self, *other));
  }
}

impl Dot for double4 {
  type Output = f64;

  #[inline]
  fn dot(self, other: double4) -> f64 {
    return double4::reduce_add(self * other);
  }
}

impl double4 {
  #[inline]
  pub fn bitcast<T>(x: T) -> double4 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn broadcast(x: f64) -> double4 {
    return double4(x, x, x, x);
  }

  #[inline]
  pub fn extract(self, i: u32) -> f64 {
    return unsafe { simd_extract(self, i) };
  }

  #[inline]
  pub fn replace(self, i: u32, x: f64) -> double4 {
    return unsafe { simd_insert(self, i, x) };
  }

  #[inline]
  pub fn eq(x: double4, y: double4) -> long4 {
    return unsafe { simd_eq(x, y) };
  }

  #[inline]
  pub fn ne(x: double4, y: double4) -> long4 {
    return unsafe { simd_ne(x, y) };
  }

  #[inline]
  pub fn lt(x: double4, y: double4) -> long4 {
    return unsafe { simd_lt(x, y) };
  }

  #[inline]
  pub fn le(x: double4, y: double4) -> long4 {
    return unsafe { simd_le(x, y) };
  }

  #[inline]
  pub fn gt(x: double4, y: double4) -> long4 {
    return unsafe { simd_gt(x, y) };
  }

  #[inline]
  pub fn ge(x: double4, y: double4) -> long4 {
    return unsafe { simd_ge(x, y) };
  }

  #[inline]
  pub fn abs(x: double4) -> double4 {
    return double4(x.0.abs(), x.1.abs(), x.2.abs(), x.3.abs());
  }

  #[inline]
  pub fn max(x: double4, y: double4) -> double4 {
    return double4(x.0.max(y.0), x.1.max(y.1), x.2.max(y.2), x.3.max(y.3));
  }

  #[inline]
  pub fn min(x: double4, y: double4) -> double4 {
    return double4(x.0.min(y.0), x.1.min(y.1), x.2.min(y.2), x.3.min(y.3));
  }

  #[inline]
  pub fn clamp(x: double4, min: double4, max: double4) -> double4 {
    return double4::min(double4::max(x, min), max);
  }

  #[inline]
  pub fn sign(x: double4) -> double4 {
    let (zero, one) = (double4::broadcast(0.0), double4::broadcast(1.0));
    return double4::bitselect(double4::copysign(one, x), zero, double4::eq(x, zero) | double4::ne(x, x));
  }

  #[inline]
  pub fn mix(x: double4, y: double4, t: double4) -> double4 {
    return x + t * (y - x);
  }

  #[inline]
  pub fn recip(x: double4) -> double4 {
    return double4::broadcast(1.0) / x;
  }

  #[inline]
  pub fn rsqrt(x: double4) -> double4 {
    return double4::broadcast(1.0) / double4::sqrt(x);
  }

  #[inline]
  pub fn fract(x: double4) -> double4 {
    return double4(x.0.fract(), x.1.fract(), x.2.fract(), x.3.fract());
  }

  #[inline]
  pub fn step(edge: double4, x: double4) -> double4 {
    return double4::bitselect(double4::broadcast(1.0), double4::broadcast(0.0), double4::lt(x, edge));
  }

  #[inline]
  pub fn smoothstep(edge0: double4, edge1: double4, x: double4) -> double4 {
    let t = double4::clamp((x - edge0) / (edge1 - edge0), double4::broadcast(0.0), double4::broadcast(1.0));

    return t * t * (3.0 - 2.0 * t);
  }

  #[inline]
  pub fn reduce_add(x: double4) -> f64 {
    return double2::reduce_add(x.lo() + x.hi());
  }

  #[inline]
  pub fn reduce_min(x: double4) -> f64 {
    return double2::reduce_min(double2::min(x.lo(), x.hi()));
  }

  #[inline]
  pub fn reduce_max(x: double4) -> f64 {
    return double2::reduce_max(double2::max(x.lo(), x.hi()));
  }

  #[inline]
  pub fn copysign(x: double4, y: double4) -> double4 {
    return double4::bitselect(y, x, long4::broadcast(std::i64::MAX));
  }

  #[inline]
  pub fn sqrt(x: double4) -> double4 {
    return double4(x.0.sqrt(), x.1.sqrt(), x.2.sqrt(), x.3.sqrt());
  }

  #[inline]
  pub fn ceil(x: double4) -> double4 {
    return double4(x.0.ceil(), x.1.ceil(), x.2.ceil(), x.3.ceil());
  }

  #[inline]
  pub fn floor(x: double4) -> double4 {
    return double4(x.0.floor(), x.1.floor(), x.2.floor(), x.3.floor());
  }

  #[inline]
  pub fn trunc(x: double4) -> double4 {
    return double4(x.0.trunc(), x.1.trunc(), x.2.trunc(), x.3.trunc());
  }

  #[inline]
  pub fn sin(x: double4) -> double4 {
    return double4(x.0.sin(), x.1.sin(), x.2.sin(), x.3.sin());
  }

  #[inline]
  pub fn cos(x: double4) -> double4 {
    return double4(x.0.cos(), x.1.cos(), x.2.cos(), x.3.cos());
  }

  #[inline]
  pub fn select(x: double4, y: double4, z: long4) -> double4 {
    return double4::bitselect(x, y, z >> 63);
  }

  #[inline]
  pub fn bitselect(x: double4, y: double4, z: long4) -> double4 {
    return double4::bitcast(long4::bitselect(long4::bitcast(x), long4::bitcast(y), z));
  }

  #[inline]
  pub fn lo(self) -> double2 {
    return double2(self.0, self.1);
  }

  #[inline]
  pub fn hi(self) -> double2 {
    return double2(self.2, self.3);
  }
}
