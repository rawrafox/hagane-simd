use std;
use ::*;

#[repr(C)]
#[repr(simd)]
#[derive(Copy, Clone, Debug)]
pub struct long4(pub i64, pub i64, pub i64, pub i64);
pub type vector_long4 = long4;

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

impl std::ops::Index<u32> for long4 {
  type Output = i64;

  #[inline]
  fn index(&self, index: u32) -> &i64 {
    return unsafe { simd_extract(self, index) };
  }
}

impl std::ops::Add for long4 {
  type Output = Self;

  #[inline]
  fn add(self, other: Self) -> Self {
    return unsafe { simd_add(self, other) };
  }
}

impl std::ops::Add<i64> for long4 {
  type Output = Self;

  #[inline]
  fn add(self, other: i64) -> Self {
    return unsafe { simd_add(self, long4::broadcast(other)) };
  }
}

impl std::ops::Add<long4> for i64 {
  type Output = long4;

  #[inline]
  fn add(self, other: long4) -> long4 {
    return unsafe { simd_add(long4::broadcast(self), other) };
  }
}

impl std::ops::Sub for long4 {
  type Output = Self;

  #[inline]
  fn sub(self, other: Self) -> Self {
    return unsafe { simd_sub(self, other) };
  }
}

impl std::ops::Sub<i64> for long4 {
  type Output = Self;

  #[inline]
  fn sub(self, other: i64) -> Self {
    return unsafe { simd_sub(self, long4::broadcast(other)) };
  }
}

impl std::ops::Sub<long4> for i64 {
  type Output = long4;

  #[inline]
  fn sub(self, other: long4) -> long4 {
    return unsafe { simd_sub(long4::broadcast(self), other) };
  }
}

impl std::ops::Mul for long4 {
  type Output = Self;

  #[inline]
  fn mul(self, other: Self) -> Self {
    return unsafe { simd_mul(self, other) };
  }
}

impl std::ops::Mul<i64> for long4 {
  type Output = Self;

  #[inline]
  fn mul(self, other: i64) -> Self {
    return unsafe { simd_mul(self, long4::broadcast(other)) };
  }
}

impl std::ops::Mul<long4> for i64 {
  type Output = long4;

  #[inline]
  fn mul(self, other: long4) -> long4 {
    return unsafe { simd_mul(long4::broadcast(self), other) };
  }
}

impl std::ops::Div for long4 {
  type Output = Self;

  #[inline]
  fn div(self, other: Self) -> Self {
    return unsafe { simd_div(self, other) };
  }
}

impl std::ops::Div<i64> for long4 {
  type Output = Self;

  #[inline]
  fn div(self, other: i64) -> Self {
    return unsafe { simd_div(self, long4::broadcast(other)) };
  }
}

impl std::ops::Div<long4> for i64 {
  type Output = long4;

  #[inline]
  fn div(self, other: long4) -> long4 {
    return unsafe { simd_div(long4::broadcast(self), other) };
  }
}

impl std::ops::BitAnd for long4 {
  type Output = Self;

  #[inline]
  fn bitand(self, other: Self) -> Self {
    return unsafe { simd_and(self, other) };
  }
}

impl std::ops::BitAnd<i64> for long4 {
  type Output = Self;

  #[inline]
  fn bitand(self, other: i64) -> Self {
    return unsafe { simd_and(self, long4::broadcast(other)) };
  }
}

impl std::ops::BitAnd<long4> for i64 {
  type Output = long4;

  #[inline]
  fn bitand(self, other: long4) -> long4 {
    return unsafe { simd_and(long4::broadcast(self), other) };
  }
}

impl std::ops::BitOr for long4 {
  type Output = Self;

  #[inline]
  fn bitor(self, other: Self) -> Self {
    return unsafe { simd_or(self, other) };
  }
}

impl std::ops::BitOr<i64> for long4 {
  type Output = Self;

  #[inline]
  fn bitor(self, other: i64) -> Self {
    return unsafe { simd_or(self, long4::broadcast(other)) };
  }
}

impl std::ops::BitOr<long4> for i64 {
  type Output = long4;

  #[inline]
  fn bitor(self, other: long4) -> long4 {
    return unsafe { simd_or(long4::broadcast(self), other) };
  }
}

impl std::ops::BitXor for long4 {
  type Output = Self;

  #[inline]
  fn bitxor(self, other: Self) -> Self {
    return unsafe { simd_xor(self, other) };
  }
}

impl std::ops::BitXor<i64> for long4 {
  type Output = Self;

  #[inline]
  fn bitxor(self, other: i64) -> Self {
    return unsafe { simd_xor(self, long4::broadcast(other)) };
  }
}

impl std::ops::BitXor<long4> for i64 {
  type Output = long4;

  #[inline]
  fn bitxor(self, other: long4) -> long4 {
    return unsafe { simd_xor(long4::broadcast(self), other) };
  }
}

impl std::ops::Shl<long4> for long4 {
  type Output = Self;

  #[inline]
  fn shl(self, other: Self) -> Self {
    return unsafe { simd_shl(self, other) };
  }
}

impl std::ops::Shl<i64> for long4 {
  type Output = Self;

  #[inline]
  fn shl(self, other: i64) -> Self {
    return unsafe { simd_shl(self, long4::broadcast(other)) };
  }
}

impl std::ops::Shl<long4> for i64 {
  type Output = long4;

  #[inline]
  fn shl(self, other: long4) -> long4 {
    return unsafe { simd_shl(long4::broadcast(self), other) };
  }
}

impl std::ops::Shr<long4> for long4 {
  type Output = Self;

  #[inline]
  fn shr(self, other: Self) -> Self {
    return unsafe { simd_shr(self, other) };
  }
}

impl std::ops::Shr<i64> for long4 {
  type Output = Self;

  #[inline]
  fn shr(self, other: i64) -> Self {
    return unsafe { simd_shr(self, long4::broadcast(other)) };
  }
}

impl std::ops::Shr<long4> for i64 {
  type Output = long4;

  #[inline]
  fn shr(self, other: long4) -> long4 {
    return unsafe { simd_shr(long4::broadcast(self), other) };
  }
}

impl std::ops::Not for long4 {
  type Output = Self;

  #[inline]
  fn not(self) -> Self {
    return self ^ -1;
  }
}

impl PartialEq for long4 {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    return long4::all(long4::eq(*self, *other));
  }

  #[inline]
  fn ne(&self, other: &Self) -> bool {
    return long4::all(long4::ne(*self, *other));
  }
}

impl Dot for long4 {
  type Output = i64;

  #[inline]
  fn dot(self, other: long4) -> i64 {
    return long4::reduce_add(self * other);
  }
}

impl long4 {
  #[inline]
  pub fn bitcast<T>(x: T) -> long4 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn broadcast(x: i64) -> long4 {
    return long4(x, x, x, x);
  }

  #[inline]
  pub fn extract(self, i: u32) -> i64 {
    return unsafe { simd_extract(self, i) };
  }

  #[inline]
  pub fn replace(self, i: u32, x: i64) -> long4 {
    return unsafe { simd_insert(self, i, x) };
  }

  #[inline]
  pub fn eq(x: long4, y: long4) -> long4 {
    return unsafe { simd_eq(x, y) };
  }

  #[inline]
  pub fn ne(x: long4, y: long4) -> long4 {
    return unsafe { simd_ne(x, y) };
  }

  #[inline]
  pub fn lt(x: long4, y: long4) -> long4 {
    return unsafe { simd_lt(x, y) };
  }

  #[inline]
  pub fn le(x: long4, y: long4) -> long4 {
    return unsafe { simd_le(x, y) };
  }

  #[inline]
  pub fn gt(x: long4, y: long4) -> long4 {
    return unsafe { simd_gt(x, y) };
  }

  #[inline]
  pub fn ge(x: long4, y: long4) -> long4 {
    return unsafe { simd_ge(x, y) };
  }

  #[inline]
  pub fn madd(x: long4, y: long4, z: long4) -> long4 {
    return x * y + z;
  }

  #[inline]
  pub fn abs(x: long4) -> long4 {
    let mask = x >> 63;
    return (x ^ mask) - mask;
  }

  #[inline]
  pub fn max(x: long4, y: long4) -> long4 {
    return long4::bitselect(x, y, long4::gt(y, x));
  }

  #[inline]
  pub fn min(x: long4, y: long4) -> long4 {
    return long4::bitselect(x, y, long4::lt(y, x));
  }

  #[inline]
  pub fn clamp(x: long4, min: long4, max: long4) -> long4 {
    return long4::min(long4::max(x, min), max);
  }

  #[inline]
  pub fn reduce_add(x: long4) -> i64 {
    return long2::reduce_add(x.lo() + x.hi());
  }

  #[inline]
  pub fn reduce_min(x: long4) -> i64 {
    return long2::reduce_min(long2::min(x.lo(), x.hi()));
  }

  #[inline]
  pub fn reduce_max(x: long4) -> i64 {
    return long2::reduce_max(long2::max(x.lo(), x.hi()));
  }

  #[inline]
  pub fn all(x: long4) -> bool {
    return (x.0 & x.1 & x.2 & x.3) & std::i64::MIN != 0;
  }

  #[inline]
  pub fn any(x: long4) -> bool {
    return (x.0 | x.1 | x.2 | x.3) & std::i64::MIN != 0;
  }

  #[inline]
  pub fn bitselect(x: long4, y: long4, z: long4) -> long4 {
    return (x & !z) | (y & z);
  }

  #[inline]
  pub fn lo(self) -> long2 {
    return long2(self.0, self.1);
  }

  #[inline]
  pub fn hi(self) -> long2 {
    return long2(self.2, self.3);
  }

  #[inline]
  pub fn odd(self) -> long2 {
    return long2(self.1, self.3);
  }

  #[inline]
  pub fn even(self) -> long2 {
    return long2(self.0, self.2);
  }
}
