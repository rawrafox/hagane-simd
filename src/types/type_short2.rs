use std;
use ::*;

#[repr(C)]
#[repr(simd)]
#[derive(Copy, Clone, Debug)]
pub struct short2(pub i16, pub i16);
pub type vector_short2 = short2;

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

impl std::ops::Index<u32> for short2 {
  type Output = i16;

  #[inline]
  fn index(&self, index: u32) -> &i16 {
    return unsafe { simd_extract(self, index) };
  }
}

impl std::ops::Add for short2 {
  type Output = Self;

  #[inline]
  fn add(self, other: Self) -> Self {
    return unsafe { simd_add(self, other) };
  }
}

impl std::ops::Add<i16> for short2 {
  type Output = Self;

  #[inline]
  fn add(self, other: i16) -> Self {
    return unsafe { simd_add(self, short2::broadcast(other)) };
  }
}

impl std::ops::Add<short2> for i16 {
  type Output = short2;

  #[inline]
  fn add(self, other: short2) -> short2 {
    return unsafe { simd_add(short2::broadcast(self), other) };
  }
}

impl std::ops::Sub for short2 {
  type Output = Self;

  #[inline]
  fn sub(self, other: Self) -> Self {
    return unsafe { simd_sub(self, other) };
  }
}

impl std::ops::Sub<i16> for short2 {
  type Output = Self;

  #[inline]
  fn sub(self, other: i16) -> Self {
    return unsafe { simd_sub(self, short2::broadcast(other)) };
  }
}

impl std::ops::Sub<short2> for i16 {
  type Output = short2;

  #[inline]
  fn sub(self, other: short2) -> short2 {
    return unsafe { simd_sub(short2::broadcast(self), other) };
  }
}

impl std::ops::Mul for short2 {
  type Output = Self;

  #[inline]
  fn mul(self, other: Self) -> Self {
    return unsafe { simd_mul(self, other) };
  }
}

impl std::ops::Mul<i16> for short2 {
  type Output = Self;

  #[inline]
  fn mul(self, other: i16) -> Self {
    return unsafe { simd_mul(self, short2::broadcast(other)) };
  }
}

impl std::ops::Mul<short2> for i16 {
  type Output = short2;

  #[inline]
  fn mul(self, other: short2) -> short2 {
    return unsafe { simd_mul(short2::broadcast(self), other) };
  }
}

impl std::ops::Div for short2 {
  type Output = Self;

  #[inline]
  fn div(self, other: Self) -> Self {
    return unsafe { simd_div(self, other) };
  }
}

impl std::ops::Div<i16> for short2 {
  type Output = Self;

  #[inline]
  fn div(self, other: i16) -> Self {
    return unsafe { simd_div(self, short2::broadcast(other)) };
  }
}

impl std::ops::Div<short2> for i16 {
  type Output = short2;

  #[inline]
  fn div(self, other: short2) -> short2 {
    return unsafe { simd_div(short2::broadcast(self), other) };
  }
}

impl std::ops::BitAnd for short2 {
  type Output = Self;

  #[inline]
  fn bitand(self, other: Self) -> Self {
    return unsafe { simd_and(self, other) };
  }
}

impl std::ops::BitAnd<i16> for short2 {
  type Output = Self;

  #[inline]
  fn bitand(self, other: i16) -> Self {
    return unsafe { simd_and(self, short2::broadcast(other)) };
  }
}

impl std::ops::BitAnd<short2> for i16 {
  type Output = short2;

  #[inline]
  fn bitand(self, other: short2) -> short2 {
    return unsafe { simd_and(short2::broadcast(self), other) };
  }
}

impl std::ops::BitOr for short2 {
  type Output = Self;

  #[inline]
  fn bitor(self, other: Self) -> Self {
    return unsafe { simd_or(self, other) };
  }
}

impl std::ops::BitOr<i16> for short2 {
  type Output = Self;

  #[inline]
  fn bitor(self, other: i16) -> Self {
    return unsafe { simd_or(self, short2::broadcast(other)) };
  }
}

impl std::ops::BitOr<short2> for i16 {
  type Output = short2;

  #[inline]
  fn bitor(self, other: short2) -> short2 {
    return unsafe { simd_or(short2::broadcast(self), other) };
  }
}

impl std::ops::BitXor for short2 {
  type Output = Self;

  #[inline]
  fn bitxor(self, other: Self) -> Self {
    return unsafe { simd_xor(self, other) };
  }
}

impl std::ops::BitXor<i16> for short2 {
  type Output = Self;

  #[inline]
  fn bitxor(self, other: i16) -> Self {
    return unsafe { simd_xor(self, short2::broadcast(other)) };
  }
}

impl std::ops::BitXor<short2> for i16 {
  type Output = short2;

  #[inline]
  fn bitxor(self, other: short2) -> short2 {
    return unsafe { simd_xor(short2::broadcast(self), other) };
  }
}

impl std::ops::Shl<short2> for short2 {
  type Output = Self;

  #[inline]
  fn shl(self, other: Self) -> Self {
    return unsafe { simd_shl(self, other) };
  }
}

impl std::ops::Shl<i16> for short2 {
  type Output = Self;

  #[inline]
  fn shl(self, other: i16) -> Self {
    return unsafe { simd_shl(self, short2::broadcast(other)) };
  }
}

impl std::ops::Shl<short2> for i16 {
  type Output = short2;

  #[inline]
  fn shl(self, other: short2) -> short2 {
    return unsafe { simd_shl(short2::broadcast(self), other) };
  }
}

impl std::ops::Shr<short2> for short2 {
  type Output = Self;

  #[inline]
  fn shr(self, other: Self) -> Self {
    return unsafe { simd_shr(self, other) };
  }
}

impl std::ops::Shr<i16> for short2 {
  type Output = Self;

  #[inline]
  fn shr(self, other: i16) -> Self {
    return unsafe { simd_shr(self, short2::broadcast(other)) };
  }
}

impl std::ops::Shr<short2> for i16 {
  type Output = short2;

  #[inline]
  fn shr(self, other: short2) -> short2 {
    return unsafe { simd_shr(short2::broadcast(self), other) };
  }
}

impl std::ops::Not for short2 {
  type Output = Self;

  #[inline]
  fn not(self) -> Self {
    return self ^ short2::broadcast(-1);
  }
}

impl PartialEq for short2 {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    return short2::all(short2::eq(*self, *other));
  }

  #[inline]
  fn ne(&self, other: &Self) -> bool {
    return short2::all(short2::ne(*self, *other));
  }
}

impl Dot for short2 {
  type Output = i16;

  #[inline]
  fn dot(self, other: short2) -> i16 {
    return short2::reduce_add(self * other);
  }
}

impl short2 {
  #[inline]
  pub fn bitcast<T>(x: T) -> short2 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn broadcast(x: i16) -> short2 {
    return short2(x, x);
  }

  #[inline]
  pub fn extract(self, i: u32) -> i16 {
    return unsafe { simd_extract(self, i) };
  }

  #[inline]
  pub fn replace(self, i: u32, x: i16) -> short2 {
    return unsafe { simd_insert(self, i, x) };
  }

  #[inline]
  pub fn eq(x: short2, y: short2) -> short2 {
    return unsafe { simd_eq(x, y) };
  }

  #[inline]
  pub fn ne(x: short2, y: short2) -> short2 {
    return unsafe { simd_ne(x, y) };
  }

  #[inline]
  pub fn lt(x: short2, y: short2) -> short2 {
    return unsafe { simd_lt(x, y) };
  }

  #[inline]
  pub fn le(x: short2, y: short2) -> short2 {
    return unsafe { simd_le(x, y) };
  }

  #[inline]
  pub fn gt(x: short2, y: short2) -> short2 {
    return unsafe { simd_gt(x, y) };
  }

  #[inline]
  pub fn ge(x: short2, y: short2) -> short2 {
    return unsafe { simd_ge(x, y) };
  }

  #[inline]
  pub fn abs(x: short2) -> short2 {
    let mask = x >> 15;
    return (x ^ mask) - mask;
  }

  #[inline]
  pub fn max(x: short2, y: short2) -> short2 {
    return short2(std::cmp::max(x.0, y.0), std::cmp::max(x.1, y.1));
  }

  #[inline]
  pub fn min(x: short2, y: short2) -> short2 {
    return short2(std::cmp::min(x.0, y.0), std::cmp::min(x.1, y.1));
  }

  #[inline]
  pub fn clamp(x: short2, min: short2, max: short2) -> short2 {
    return short2::min(short2::max(x, min), max);
  }

  #[inline]
  pub fn reduce_add(x: short2) -> i16 {
    return x.0 + x.1;
  }

  #[inline]
  pub fn reduce_min(x: short2) -> i16 {
    return std::cmp::min(x.0, x.1);
  }

  #[inline]
  pub fn reduce_max(x: short2) -> i16 {
    return std::cmp::max(x.0, x.1);
  }

  #[inline]
  pub fn all(x: short2) -> bool {
    return (x.0 & x.1) & std::i16::MIN != 0;
  }

  #[inline]
  pub fn any(x: short2) -> bool {
    return (x.0 | x.1) & std::i16::MIN != 0;
  }

  #[inline]
  pub fn bitselect(x: short2, y: short2, z: short2) -> short2 {
    return (x & !z) | (y & z);
  }

  #[inline]
  pub fn lo(self) -> short1 {
    return self.0;
  }

  #[inline]
  pub fn hi(self) -> short1 {
    return self.1;
  }
}
