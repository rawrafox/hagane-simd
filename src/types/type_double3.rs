use std;
use ::*;

#[repr(C)]
#[repr(simd)]
#[derive(Copy, Clone, Debug)]
pub struct double3(pub f64, pub f64, pub f64);
pub type vector_double3 = double3;

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
    return long3::all(double3::eq(*self, *other));
  }

  #[inline]
  fn ne(&self, other: &Self) -> bool {
    return long3::all(double3::ne(*self, *other));
  }
}

impl Dot for double3 {
  type Output = f64;

  #[inline]
  fn dot(self, other: double3) -> f64 {
    return double3::reduce_add(self * other);
  }
}

impl double3 {
  #[inline]
  pub fn bitcast<T>(x: T) -> double3 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn broadcast(x: f64) -> double3 {
    return double3(x, x, x);
  }

  #[inline]
  pub fn extract(self, i: u32) -> f64 {
    return unsafe { simd_extract(self, i) };
  }

  #[inline]
  pub fn replace(self, i: u32, x: f64) -> double3 {
    return unsafe { simd_insert(self, i, x) };
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
  pub fn abs(x: double3) -> double3 {
    return double3(x.0.abs(), x.1.abs(), x.2.abs());
  }

  #[inline]
  pub fn max(x: double3, y: double3) -> double3 {
    return double3(x.0.max(y.0), x.1.max(y.1), x.2.max(y.2));
  }

  #[inline]
  pub fn min(x: double3, y: double3) -> double3 {
    return double3(x.0.min(y.0), x.1.min(y.1), x.2.min(y.2));
  }

  #[inline]
  pub fn clamp(x: double3, min: double3, max: double3) -> double3 {
    return double3::min(double3::max(x, min), max);
  }

  #[inline]
  pub fn sign(x: double3) -> double3 {
    let (zero, one) = (double3::broadcast(0.0), double3::broadcast(1.0));
    return double3::bitselect(double3::copysign(one, x), zero, double3::eq(x, zero) | double3::ne(x, x));
  }

  #[inline]
  pub fn mix(x: double3, y: double3, t: double3) -> double3 {
    return x + t * (y - x);
  }

  #[inline]
  pub fn recip(x: double3) -> double3 {
    return 1.0 / x;
  }

  #[inline]
  pub fn rsqrt(x: double3) -> double3 {
    return 1.0 / double3::sqrt(x);
  }

  #[inline]
  pub fn fract(x: double3) -> double3 {
    return double3(x.0.fract(), x.1.fract(), x.2.fract());
  }

  #[inline]
  pub fn step(edge: double3, x: double3) -> double3 {
    return double3::bitselect(double3::broadcast(1.0), double3::broadcast(0.0), double3::lt(x, edge));
  }

  #[inline]
  pub fn smoothstep(edge0: double3, edge1: double3, x: double3) -> double3 {
    let t = double3::clamp((x - edge0) / (edge1 - edge0), double3::broadcast(0.0), double3::broadcast(1.0));

    return t * t * (3.0 - 2.0 * t);
  }

  #[inline]
  pub fn reduce_add(x: double3) -> f64 {
    return x.0 + x.1 + x.2;
  }

  #[inline]
  pub fn reduce_min(x: double3) -> f64 {
    return x.2.min(double2::reduce_min(x.lo()));
  }

  #[inline]
  pub fn reduce_max(x: double3) -> f64 {
    return x.2.max(double2::reduce_max(x.lo()));
  }

  #[inline]
  pub fn copysign(x: double3, y: double3) -> double3 {
    return double3::bitselect(y, x, long3::broadcast(std::i64::MAX));
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
  pub fn select(x: double3, y: double3, z: long3) -> double3 {
    return double3::bitselect(x, y, z >> 63);
  }

  #[inline]
  pub fn bitselect(x: double3, y: double3, z: long3) -> double3 {
    return double3::bitcast(long3::bitselect(long3::bitcast(x), long3::bitcast(y), z));
  }

  #[inline]
  pub fn lo(self) -> double2 {
    return double2(self.0, self.1);
  }

  #[inline]
  pub fn hi(self) -> double2 {
    return double2(self.2, 0.0);
  }
}
