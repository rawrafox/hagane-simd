use std;
use ::*;

#[repr(C)]
#[repr(simd)]
#[derive(Copy, Clone, Debug)]
pub struct long3(pub i64, pub i64, pub i64);
pub type vector_long3 = long3;

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

impl std::ops::Index<u32> for long3 {
  type Output = i64;

  #[inline]
  fn index(&self, index: u32) -> &i64 {
    return unsafe { simd_extract(self, index) };
  }
}

impl std::ops::Add for long3 {
  type Output = Self;

  #[inline]
  fn add(self, other: Self) -> Self {
    return unsafe { simd_add(self, other) };
  }
}

impl std::ops::Add<i64> for long3 {
  type Output = Self;

  #[inline]
  fn add(self, other: i64) -> Self {
    return unsafe { simd_add(self, long3::broadcast(other)) };
  }
}

impl std::ops::Add<long3> for i64 {
  type Output = long3;

  #[inline]
  fn add(self, other: long3) -> long3 {
    return unsafe { simd_add(long3::broadcast(self), other) };
  }
}

impl std::ops::Sub for long3 {
  type Output = Self;

  #[inline]
  fn sub(self, other: Self) -> Self {
    return unsafe { simd_sub(self, other) };
  }
}

impl std::ops::Sub<i64> for long3 {
  type Output = Self;

  #[inline]
  fn sub(self, other: i64) -> Self {
    return unsafe { simd_sub(self, long3::broadcast(other)) };
  }
}

impl std::ops::Sub<long3> for i64 {
  type Output = long3;

  #[inline]
  fn sub(self, other: long3) -> long3 {
    return unsafe { simd_sub(long3::broadcast(self), other) };
  }
}

impl std::ops::Mul for long3 {
  type Output = Self;

  #[inline]
  fn mul(self, other: Self) -> Self {
    return unsafe { simd_mul(self, other) };
  }
}

impl std::ops::Mul<i64> for long3 {
  type Output = Self;

  #[inline]
  fn mul(self, other: i64) -> Self {
    return unsafe { simd_mul(self, long3::broadcast(other)) };
  }
}

impl std::ops::Mul<long3> for i64 {
  type Output = long3;

  #[inline]
  fn mul(self, other: long3) -> long3 {
    return unsafe { simd_mul(long3::broadcast(self), other) };
  }
}

impl std::ops::Div for long3 {
  type Output = Self;

  #[inline]
  fn div(self, other: Self) -> Self {
    return unsafe { simd_div(self, other) };
  }
}

impl std::ops::Div<i64> for long3 {
  type Output = Self;

  #[inline]
  fn div(self, other: i64) -> Self {
    return unsafe { simd_div(self, long3::broadcast(other)) };
  }
}

impl std::ops::Div<long3> for i64 {
  type Output = long3;

  #[inline]
  fn div(self, other: long3) -> long3 {
    return unsafe { simd_div(long3::broadcast(self), other) };
  }
}

impl std::ops::BitAnd for long3 {
  type Output = Self;

  #[inline]
  fn bitand(self, other: Self) -> Self {
    return unsafe { simd_and(self, other) };
  }
}

impl std::ops::BitAnd<i64> for long3 {
  type Output = Self;

  #[inline]
  fn bitand(self, other: i64) -> Self {
    return unsafe { simd_and(self, long3::broadcast(other)) };
  }
}

impl std::ops::BitAnd<long3> for i64 {
  type Output = long3;

  #[inline]
  fn bitand(self, other: long3) -> long3 {
    return unsafe { simd_and(long3::broadcast(self), other) };
  }
}

impl std::ops::BitOr for long3 {
  type Output = Self;

  #[inline]
  fn bitor(self, other: Self) -> Self {
    return unsafe { simd_or(self, other) };
  }
}

impl std::ops::BitOr<i64> for long3 {
  type Output = Self;

  #[inline]
  fn bitor(self, other: i64) -> Self {
    return unsafe { simd_or(self, long3::broadcast(other)) };
  }
}

impl std::ops::BitOr<long3> for i64 {
  type Output = long3;

  #[inline]
  fn bitor(self, other: long3) -> long3 {
    return unsafe { simd_or(long3::broadcast(self), other) };
  }
}

impl std::ops::BitXor for long3 {
  type Output = Self;

  #[inline]
  fn bitxor(self, other: Self) -> Self {
    return unsafe { simd_xor(self, other) };
  }
}

impl std::ops::BitXor<i64> for long3 {
  type Output = Self;

  #[inline]
  fn bitxor(self, other: i64) -> Self {
    return unsafe { simd_xor(self, long3::broadcast(other)) };
  }
}

impl std::ops::BitXor<long3> for i64 {
  type Output = long3;

  #[inline]
  fn bitxor(self, other: long3) -> long3 {
    return unsafe { simd_xor(long3::broadcast(self), other) };
  }
}

impl std::ops::Shl<long3> for long3 {
  type Output = Self;

  #[inline]
  fn shl(self, other: Self) -> Self {
    return unsafe { simd_shl(self, other) };
  }
}

impl std::ops::Shl<i64> for long3 {
  type Output = Self;

  #[inline]
  fn shl(self, other: i64) -> Self {
    return unsafe { simd_shl(self, long3::broadcast(other)) };
  }
}

impl std::ops::Shl<long3> for i64 {
  type Output = long3;

  #[inline]
  fn shl(self, other: long3) -> long3 {
    return unsafe { simd_shl(long3::broadcast(self), other) };
  }
}

impl std::ops::Shr<long3> for long3 {
  type Output = Self;

  #[inline]
  fn shr(self, other: Self) -> Self {
    return unsafe { simd_shr(self, other) };
  }
}

impl std::ops::Shr<i64> for long3 {
  type Output = Self;

  #[inline]
  fn shr(self, other: i64) -> Self {
    return unsafe { simd_shr(self, long3::broadcast(other)) };
  }
}

impl std::ops::Shr<long3> for i64 {
  type Output = long3;

  #[inline]
  fn shr(self, other: long3) -> long3 {
    return unsafe { simd_shr(long3::broadcast(self), other) };
  }
}

impl std::ops::Not for long3 {
  type Output = Self;

  #[inline]
  fn not(self) -> Self {
    return self ^ long3::broadcast(-1);
  }
}

impl PartialEq for long3 {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    return long3::all(long3::eq(*self, *other));
  }

  #[inline]
  fn ne(&self, other: &Self) -> bool {
    return long3::all(long3::ne(*self, *other));
  }
}

impl Dot for long3 {
  type Output = i64;

  #[inline]
  fn dot(self, other: long3) -> i64 {
    return long3::reduce_add(self * other);
  }
}

impl long3 {
  #[inline]
  pub fn bitcast<T>(x: T) -> long3 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn broadcast(x: i64) -> long3 {
    return long3(x, x, x);
  }

  #[inline]
  pub fn extract(self, i: u32) -> i64 {
    return unsafe { simd_extract(self, i) };
  }

  #[inline]
  pub fn replace(self, i: u32, x: i64) -> long3 {
    return unsafe { simd_insert(self, i, x) };
  }

  #[inline]
  pub fn eq(x: long3, y: long3) -> long3 {
    return unsafe { simd_eq(x, y) };
  }

  #[inline]
  pub fn ne(x: long3, y: long3) -> long3 {
    return unsafe { simd_ne(x, y) };
  }

  #[inline]
  pub fn lt(x: long3, y: long3) -> long3 {
    return unsafe { simd_lt(x, y) };
  }

  #[inline]
  pub fn le(x: long3, y: long3) -> long3 {
    return unsafe { simd_le(x, y) };
  }

  #[inline]
  pub fn gt(x: long3, y: long3) -> long3 {
    return unsafe { simd_gt(x, y) };
  }

  #[inline]
  pub fn ge(x: long3, y: long3) -> long3 {
    return unsafe { simd_ge(x, y) };
  }

  #[inline]
  pub fn abs(x: long3) -> long3 {
    let mask = x >> 63;
    return (x ^ mask) - mask;
  }

  #[inline]
  pub fn max(x: long3, y: long3) -> long3 {
    return long3::bitselect(x, y, long3::gt(y, x));
  }

  #[inline]
  pub fn min(x: long3, y: long3) -> long3 {
    return long3::bitselect(x, y, long3::lt(y, x));
  }

  #[inline]
  pub fn clamp(x: long3, min: long3, max: long3) -> long3 {
    return long3::min(long3::max(x, min), max);
  }

  #[inline]
  pub fn reduce_add(x: long3) -> i64 {
    return x.0 + x.1 + x.2;
  }

  #[inline]
  pub fn reduce_min(x: long3) -> i64 {
    return std::cmp::min(long2::reduce_min(x.lo()), x.2);
  }

  #[inline]
  pub fn reduce_max(x: long3) -> i64 {
    return std::cmp::max(long2::reduce_max(x.lo()), x.2);
  }

  #[inline]
  pub fn all(x: long3) -> bool {
    return (x.0 & x.1 & x.2) & std::i64::MIN != 0;
  }

  #[inline]
  pub fn any(x: long3) -> bool {
    return (x.0 | x.1 | x.2) & std::i64::MIN != 0;
  }

  #[inline]
  pub fn bitselect(x: long3, y: long3, z: long3) -> long3 {
    return (x & !z) | (y & z);
  }

  #[inline]
  pub fn lo(self) -> long2 {
    return long2(self.0, self.1);
  }

  #[inline]
  pub fn hi(self) -> long2 {
    return long2(self.2, 0);
  }
}
