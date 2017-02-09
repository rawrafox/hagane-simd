use std;
use ::*;

#[repr(C)]
#[repr(simd)]
#[derive(Copy, Clone, Debug)]
pub struct long2(pub i64, pub i64);
pub type vector_long2 = long2;

extern "platform-intrinsic" {
  fn simd_add<T>(x: T, y: T) -> T;
  fn simd_sub<T>(x: T, y: T) -> T;
  fn simd_mul<T>(x: T, y: T) -> T;
  fn simd_div<T>(x: T, y: T) -> T;

  fn simd_shl<T>(x: T, y: T) -> T;
  fn simd_shr<T>(x: T, y: T) -> T;

  fn simd_and<T>(x: T, y: T) -> T;
  fn simd_or<T>(x: T, y: T) -> T;
  fn simd_xor<T>(x: T, y: T) -> T;

  fn simd_eq<T, U>(x: T, y: T) -> U;
  fn simd_ne<T, U>(x: T, y: T) -> U;
  fn simd_lt<T, U>(x: T, y: T) -> U;
  fn simd_le<T, U>(x: T, y: T) -> U;
  fn simd_gt<T, U>(x: T, y: T) -> U;
  fn simd_ge<T, U>(x: T, y: T) -> U;

  fn simd_insert<T, E>(x: T, i: u32, e: E) -> T;
  fn simd_extract<T, E>(x: T, i: u32) -> E;
}

impl std::ops::Index<u32> for long2 {
  type Output = i64;

  #[inline]
  fn index(&self, index: u32) -> &i64 {
    return unsafe { simd_extract(self, index) };
  }
}

impl std::ops::Add for long2 {
  type Output = Self;

  #[inline]
  fn add(self, other: Self) -> Self {
    return unsafe { simd_add(self, other) };
  }
}

impl std::ops::Add<i64> for long2 {
  type Output = Self;

  #[inline]
  fn add(self, other: i64) -> Self {
    return unsafe { simd_add(self, long2::broadcast(other)) };
  }
}

impl std::ops::Add<long2> for i64 {
  type Output = long2;

  #[inline]
  fn add(self, other: long2) -> long2 {
    return unsafe { simd_add(long2::broadcast(self), other) };
  }
}

impl std::ops::Sub for long2 {
  type Output = Self;

  #[inline]
  fn sub(self, other: Self) -> Self {
    return unsafe { simd_sub(self, other) };
  }
}

impl std::ops::Sub<i64> for long2 {
  type Output = Self;

  #[inline]
  fn sub(self, other: i64) -> Self {
    return unsafe { simd_sub(self, long2::broadcast(other)) };
  }
}

impl std::ops::Sub<long2> for i64 {
  type Output = long2;

  #[inline]
  fn sub(self, other: long2) -> long2 {
    return unsafe { simd_sub(long2::broadcast(self), other) };
  }
}

impl std::ops::Mul for long2 {
  type Output = Self;

  #[inline]
  fn mul(self, other: Self) -> Self {
    return unsafe { simd_mul(self, other) };
  }
}

impl std::ops::Mul<i64> for long2 {
  type Output = Self;

  #[inline]
  fn mul(self, other: i64) -> Self {
    return unsafe { simd_mul(self, long2::broadcast(other)) };
  }
}

impl std::ops::Mul<long2> for i64 {
  type Output = long2;

  #[inline]
  fn mul(self, other: long2) -> long2 {
    return unsafe { simd_mul(long2::broadcast(self), other) };
  }
}

impl std::ops::Div for long2 {
  type Output = Self;

  #[inline]
  fn div(self, other: Self) -> Self {
    return unsafe { simd_div(self, other) };
  }
}

impl std::ops::Div<i64> for long2 {
  type Output = Self;

  #[inline]
  fn div(self, other: i64) -> Self {
    return unsafe { simd_div(self, long2::broadcast(other)) };
  }
}

impl std::ops::Div<long2> for i64 {
  type Output = long2;

  #[inline]
  fn div(self, other: long2) -> long2 {
    return unsafe { simd_div(long2::broadcast(self), other) };
  }
}

impl std::ops::BitAnd for long2 {
  type Output = Self;

  #[inline]
  fn bitand(self, other: Self) -> Self {
    return unsafe { simd_and(self, other) };
  }
}

impl std::ops::BitAnd<i64> for long2 {
  type Output = Self;

  #[inline]
  fn bitand(self, other: i64) -> Self {
    return unsafe { simd_and(self, long2::broadcast(other)) };
  }
}

impl std::ops::BitAnd<long2> for i64 {
  type Output = long2;

  #[inline]
  fn bitand(self, other: long2) -> long2 {
    return unsafe { simd_and(long2::broadcast(self), other) };
  }
}

impl std::ops::BitOr for long2 {
  type Output = Self;

  #[inline]
  fn bitor(self, other: Self) -> Self {
    return unsafe { simd_or(self, other) };
  }
}

impl std::ops::BitOr<i64> for long2 {
  type Output = Self;

  #[inline]
  fn bitor(self, other: i64) -> Self {
    return unsafe { simd_or(self, long2::broadcast(other)) };
  }
}

impl std::ops::BitOr<long2> for i64 {
  type Output = long2;

  #[inline]
  fn bitor(self, other: long2) -> long2 {
    return unsafe { simd_or(long2::broadcast(self), other) };
  }
}

impl std::ops::BitXor for long2 {
  type Output = Self;

  #[inline]
  fn bitxor(self, other: Self) -> Self {
    return unsafe { simd_xor(self, other) };
  }
}

impl std::ops::BitXor<i64> for long2 {
  type Output = Self;

  #[inline]
  fn bitxor(self, other: i64) -> Self {
    return unsafe { simd_xor(self, long2::broadcast(other)) };
  }
}

impl std::ops::BitXor<long2> for i64 {
  type Output = long2;

  #[inline]
  fn bitxor(self, other: long2) -> long2 {
    return unsafe { simd_xor(long2::broadcast(self), other) };
  }
}

impl std::ops::Shl<long2> for long2 {
  type Output = Self;

  #[inline]
  fn shl(self, other: Self) -> Self {
    return unsafe { simd_shl(self, other) };
  }
}

impl std::ops::Shl<i64> for long2 {
  type Output = Self;

  #[inline]
  fn shl(self, other: i64) -> Self {
    return unsafe { simd_shl(self, long2::broadcast(other)) };
  }
}

impl std::ops::Shl<long2> for i64 {
  type Output = long2;

  #[inline]
  fn shl(self, other: long2) -> long2 {
    return unsafe { simd_shl(long2::broadcast(self), other) };
  }
}

impl std::ops::Shr<long2> for long2 {
  type Output = Self;

  #[inline]
  fn shr(self, other: Self) -> Self {
    return unsafe { simd_shr(self, other) };
  }
}

impl std::ops::Shr<i64> for long2 {
  type Output = Self;

  #[inline]
  fn shr(self, other: i64) -> Self {
    return unsafe { simd_shr(self, long2::broadcast(other)) };
  }
}

impl std::ops::Shr<long2> for i64 {
  type Output = long2;

  #[inline]
  fn shr(self, other: long2) -> long2 {
    return unsafe { simd_shr(long2::broadcast(self), other) };
  }
}

impl std::ops::Not for long2 {
  type Output = Self;

  #[inline]
  fn not(self) -> Self {
    return self ^ long2::broadcast(-1);
  }
}

impl PartialEq for long2 {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    return long2::all(long2::eq(*self, *other));
  }

  #[inline]
  fn ne(&self, other: &Self) -> bool {
    return long2::all(long2::ne(*self, *other));
  }
}

impl Dot for long2 {
  type Output = i64;

  #[inline]
  fn dot(self, other: long2) -> i64 {
    return long2::reduce_add(self * other);
  }
}

impl long2 {
  #[inline]
  pub fn bitcast<T>(x: T) -> long2 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn broadcast(x: i64) -> long2 {
    return long2(x, x);
  }

  #[inline]
  pub fn extract(self, i: u32) -> i64 {
    return unsafe { simd_extract(self, i) };
  }

  #[inline]
  pub fn replace(self, i: u32, x: i64) -> long2 {
    return unsafe { simd_insert(self, i, x) };
  }

  #[inline]
  pub fn eq(x: long2, y: long2) -> long2 {
    return unsafe { simd_eq(x, y) };
  }

  #[inline]
  pub fn ne(x: long2, y: long2) -> long2 {
    return unsafe { simd_ne(x, y) };
  }

  #[inline]
  pub fn lt(x: long2, y: long2) -> long2 {
    return unsafe { simd_lt(x, y) };
  }

  #[inline]
  pub fn le(x: long2, y: long2) -> long2 {
    return unsafe { simd_le(x, y) };
  }

  #[inline]
  pub fn gt(x: long2, y: long2) -> long2 {
    return unsafe { simd_gt(x, y) };
  }

  #[inline]
  pub fn ge(x: long2, y: long2) -> long2 {
    return unsafe { simd_ge(x, y) };
  }

  #[inline]
  pub fn abs(x: long2) -> long2 {
    return long2(x.0.abs(), x.1.abs());
  }

  #[inline]
  pub fn max(x: long2, y: long2) -> long2 {
    return long2(std::cmp::max(x.0, y.0), std::cmp::max(x.1, y.1));
  }

  #[inline]
  pub fn min(x: long2, y: long2) -> long2 {
    return long2(std::cmp::min(x.0, y.0), std::cmp::min(x.1, y.1));
  }

  #[inline]
  pub fn clamp(x: long2, min: long2, max: long2) -> long2 {
    return long2::min(long2::max(x, min), max);
  }

  #[inline]
  pub fn reduce_add(x: long2) -> i64 {
    return x.0 + x.1;
  }

  #[inline]
  pub fn reduce_min(x: long2) -> i64 {
    return std::cmp::min(x.0, x.1);
  }

  #[inline]
  pub fn reduce_max(x: long2) -> i64 {
    return std::cmp::max(x.0, x.1);
  }

  #[inline]
  pub fn all(x: long2) -> bool {
    return (x.0 & x.1) & std::i64::MIN != 0;
  }

  #[inline]
  pub fn any(x: long2) -> bool {
    return (x.0 | x.1) & std::i64::MIN != 0;
  }

  #[inline]
  pub fn bitselect(x: long2, y: long2, z: long2) -> long2 {
    return (x & !z) | (y & z);
  }

  #[inline]
  pub fn lo(self) -> long1 {
    return self.0;
  }

  #[inline]
  pub fn hi(self) -> long1 {
    return self.1;
  }
}
