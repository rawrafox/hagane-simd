use std;
use ::*;

#[repr(C)]
#[repr(simd)]
#[derive(Copy, Clone, Debug)]
pub struct double2(pub f64, pub f64);
pub type vector_double2 = double2;

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

impl std::ops::Index<u32> for double2 {
  type Output = f64;

  #[inline]
  fn index(&self, index: u32) -> &f64 {
    return unsafe { simd_extract(self, index) };
  }
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
    return long2::all(double2::eq(*self, *other));
  }

  #[inline]
  fn ne(&self, other: &Self) -> bool {
    return long2::all(double2::ne(*self, *other));
  }
}

impl Dot for double2 {
  type Output = f64;

  #[inline]
  fn dot(self, other: double2) -> f64 {
    return double2::reduce_add(self * other);
  }
}

impl double2 {
  #[inline]
  pub fn bitcast<T>(x: T) -> double2 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn broadcast(x: f64) -> double2 {
    return double2(x, x);
  }

  #[inline]
  pub fn extract(self, i: u32) -> f64 {
    return unsafe { simd_extract(self, i) };
  }

  #[inline]
  pub fn replace(self, i: u32, x: f64) -> double2 {
    return unsafe { simd_insert(self, i, x) };
  }

  #[inline]
  pub fn eq(x: double2, y: double2) -> long2 {
    return unsafe { simd_eq(x, y) };
  }

  #[inline]
  pub fn ne(x: double2, y: double2) -> long2 {
    return unsafe { simd_ne(x, y) };
  }

  #[inline]
  pub fn lt(x: double2, y: double2) -> long2 {
    return unsafe { simd_lt(x, y) };
  }

  #[inline]
  pub fn le(x: double2, y: double2) -> long2 {
    return unsafe { simd_le(x, y) };
  }

  #[inline]
  pub fn gt(x: double2, y: double2) -> long2 {
    return unsafe { simd_gt(x, y) };
  }

  #[inline]
  pub fn ge(x: double2, y: double2) -> long2 {
    return unsafe { simd_ge(x, y) };
  }

  #[inline]
  pub fn abs(x: double2) -> double2 {
    return double2(x.0.abs(), x.1.abs());
  }

  #[inline]
  pub fn max(x: double2, y: double2) -> double2 {
    return double2(x.0.max(y.0), x.1.max(y.1));
  }

  #[inline]
  pub fn min(x: double2, y: double2) -> double2 {
    return double2(x.0.min(y.0), x.1.min(y.1));
  }

  #[inline]
  pub fn clamp(x: double2, min: double2, max: double2) -> double2 {
    return double2::min(double2::max(x, min), max);
  }

  #[inline]
  pub fn sign(x: double2) -> double2 {
    let (zero, one) = (double2::broadcast(0.0), double2::broadcast(1.0));
    return double2::bitselect(double2::copysign(one, x), zero, double2::eq(x, zero) | double2::ne(x, x));
  }

  #[inline]
  pub fn mix(x: double2, y: double2, t: double2) -> double2 {
    return x + t * (y - x);
  }

  #[inline]
  pub fn recip(x: double2) -> double2 {
    return 1.0 / x;
  }

  #[inline]
  pub fn rsqrt(x: double2) -> double2 {
    return 1.0 / double2::sqrt(x);
  }

  #[inline]
  pub fn fract(x: double2) -> double2 {
    return double2(x.0.fract(), x.1.fract());
  }

  #[inline]
  pub fn step(edge: double2, x: double2) -> double2 {
    return double2::bitselect(double2::broadcast(1.0), double2::broadcast(0.0), double2::lt(x, edge));
  }

  #[inline]
  pub fn smoothstep(edge0: double2, edge1: double2, x: double2) -> double2 {
    let t = double2::clamp((x - edge0) / (edge1 - edge0), double2::broadcast(0.0), double2::broadcast(1.0));

    return t * t * (3.0 - 2.0 * t);
  }

  #[inline]
  pub fn reduce_add(x: double2) -> f64 {
    return x.0 + x.1;
  }

  #[inline]
  pub fn reduce_min(x: double2) -> f64 {
    return x.0.min(x.1);
  }

  #[inline]
  pub fn reduce_max(x: double2) -> f64 {
    return x.0.max(x.1);
  }

  #[inline]
  pub fn copysign(x: double2, y: double2) -> double2 {
    return double2::bitselect(y, x, long2::broadcast(std::i64::MAX));
  }

  #[inline]
  pub fn sqrt(x: double2) -> double2 {
    return double2(x.0.sqrt(), x.1.sqrt());
  }

  #[inline]
  pub fn ceil(x: double2) -> double2 {
    return double2(x.0.ceil(), x.1.ceil());
  }

  #[inline]
  pub fn floor(x: double2) -> double2 {
    return double2(x.0.floor(), x.1.floor());
  }

  #[inline]
  pub fn trunc(x: double2) -> double2 {
    return double2(x.0.trunc(), x.1.trunc());
  }

  #[inline]
  pub fn sin(x: double2) -> double2 {
    return double2(x.0.sin(), x.1.sin());
  }

  #[inline]
  pub fn cos(x: double2) -> double2 {
    return double2(x.0.cos(), x.1.cos());
  }

  #[inline]
  pub fn select(x: double2, y: double2, z: long2) -> double2 {
    return double2::bitselect(x, y, z >> 63);
  }

  #[inline]
  pub fn bitselect(x: double2, y: double2, z: long2) -> double2 {
    return double2::bitcast(long2::bitselect(long2::bitcast(x), long2::bitcast(y), z));
  }

  #[inline]
  pub fn lo(self) -> double1 {
    return self.0;
  }

  #[inline]
  pub fn hi(self) -> double1 {
    return self.1;
  }
}
