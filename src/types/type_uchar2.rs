use std;
use ::*;

#[repr(C)]
#[repr(simd)]
#[derive(Copy, Clone, Debug)]
pub struct uchar2(pub u8, pub u8);
pub type vector_uchar2 = uchar2;

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

impl std::ops::Index<u32> for uchar2 {
  type Output = u8;

  #[inline]
  fn index(&self, index: u32) -> &u8 {
    return unsafe { simd_extract(self, index) };
  }
}

impl std::ops::Add for uchar2 {
  type Output = Self;

  #[inline]
  fn add(self, other: Self) -> Self {
    return unsafe { simd_add(self, other) };
  }
}

impl std::ops::Add<u8> for uchar2 {
  type Output = Self;

  #[inline]
  fn add(self, other: u8) -> Self {
    return unsafe { simd_add(self, uchar2::broadcast(other)) };
  }
}

impl std::ops::Add<uchar2> for u8 {
  type Output = uchar2;

  #[inline]
  fn add(self, other: uchar2) -> uchar2 {
    return unsafe { simd_add(uchar2::broadcast(self), other) };
  }
}

impl std::ops::Sub for uchar2 {
  type Output = Self;

  #[inline]
  fn sub(self, other: Self) -> Self {
    return unsafe { simd_sub(self, other) };
  }
}

impl std::ops::Sub<u8> for uchar2 {
  type Output = Self;

  #[inline]
  fn sub(self, other: u8) -> Self {
    return unsafe { simd_sub(self, uchar2::broadcast(other)) };
  }
}

impl std::ops::Sub<uchar2> for u8 {
  type Output = uchar2;

  #[inline]
  fn sub(self, other: uchar2) -> uchar2 {
    return unsafe { simd_sub(uchar2::broadcast(self), other) };
  }
}

impl std::ops::Mul for uchar2 {
  type Output = Self;

  #[inline]
  fn mul(self, other: Self) -> Self {
    return unsafe { simd_mul(self, other) };
  }
}

impl std::ops::Mul<u8> for uchar2 {
  type Output = Self;

  #[inline]
  fn mul(self, other: u8) -> Self {
    return unsafe { simd_mul(self, uchar2::broadcast(other)) };
  }
}

impl std::ops::Mul<uchar2> for u8 {
  type Output = uchar2;

  #[inline]
  fn mul(self, other: uchar2) -> uchar2 {
    return unsafe { simd_mul(uchar2::broadcast(self), other) };
  }
}

impl std::ops::Div for uchar2 {
  type Output = Self;

  #[inline]
  fn div(self, other: Self) -> Self {
    return unsafe { simd_div(self, other) };
  }
}

impl std::ops::Div<u8> for uchar2 {
  type Output = Self;

  #[inline]
  fn div(self, other: u8) -> Self {
    return unsafe { simd_div(self, uchar2::broadcast(other)) };
  }
}

impl std::ops::Div<uchar2> for u8 {
  type Output = uchar2;

  #[inline]
  fn div(self, other: uchar2) -> uchar2 {
    return unsafe { simd_div(uchar2::broadcast(self), other) };
  }
}

impl std::ops::BitAnd for uchar2 {
  type Output = Self;

  #[inline]
  fn bitand(self, other: Self) -> Self {
    return unsafe { simd_and(self, other) };
  }
}

impl std::ops::BitAnd<u8> for uchar2 {
  type Output = Self;

  #[inline]
  fn bitand(self, other: u8) -> Self {
    return unsafe { simd_and(self, uchar2::broadcast(other)) };
  }
}

impl std::ops::BitAnd<uchar2> for u8 {
  type Output = uchar2;

  #[inline]
  fn bitand(self, other: uchar2) -> uchar2 {
    return unsafe { simd_and(uchar2::broadcast(self), other) };
  }
}

impl std::ops::BitOr for uchar2 {
  type Output = Self;

  #[inline]
  fn bitor(self, other: Self) -> Self {
    return unsafe { simd_or(self, other) };
  }
}

impl std::ops::BitOr<u8> for uchar2 {
  type Output = Self;

  #[inline]
  fn bitor(self, other: u8) -> Self {
    return unsafe { simd_or(self, uchar2::broadcast(other)) };
  }
}

impl std::ops::BitOr<uchar2> for u8 {
  type Output = uchar2;

  #[inline]
  fn bitor(self, other: uchar2) -> uchar2 {
    return unsafe { simd_or(uchar2::broadcast(self), other) };
  }
}

impl std::ops::BitXor for uchar2 {
  type Output = Self;

  #[inline]
  fn bitxor(self, other: Self) -> Self {
    return unsafe { simd_xor(self, other) };
  }
}

impl std::ops::BitXor<u8> for uchar2 {
  type Output = Self;

  #[inline]
  fn bitxor(self, other: u8) -> Self {
    return unsafe { simd_xor(self, uchar2::broadcast(other)) };
  }
}

impl std::ops::BitXor<uchar2> for u8 {
  type Output = uchar2;

  #[inline]
  fn bitxor(self, other: uchar2) -> uchar2 {
    return unsafe { simd_xor(uchar2::broadcast(self), other) };
  }
}

impl std::ops::Shl<uchar2> for uchar2 {
  type Output = Self;

  #[inline]
  fn shl(self, other: Self) -> Self {
    return unsafe { simd_shl(self, other) };
  }
}

impl std::ops::Shl<u8> for uchar2 {
  type Output = Self;

  #[inline]
  fn shl(self, other: u8) -> Self {
    return unsafe { simd_shl(self, uchar2::broadcast(other)) };
  }
}

impl std::ops::Shl<uchar2> for u8 {
  type Output = uchar2;

  #[inline]
  fn shl(self, other: uchar2) -> uchar2 {
    return unsafe { simd_shl(uchar2::broadcast(self), other) };
  }
}

impl std::ops::Shr<uchar2> for uchar2 {
  type Output = Self;

  #[inline]
  fn shr(self, other: Self) -> Self {
    return unsafe { simd_shr(self, other) };
  }
}

impl std::ops::Shr<u8> for uchar2 {
  type Output = Self;

  #[inline]
  fn shr(self, other: u8) -> Self {
    return unsafe { simd_shr(self, uchar2::broadcast(other)) };
  }
}

impl std::ops::Shr<uchar2> for u8 {
  type Output = uchar2;

  #[inline]
  fn shr(self, other: uchar2) -> uchar2 {
    return unsafe { simd_shr(uchar2::broadcast(self), other) };
  }
}

impl std::ops::Not for uchar2 {
  type Output = Self;

  #[inline]
  fn not(self) -> Self {
    return self ^ std::u8::MAX;
  }
}

impl PartialEq for uchar2 {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    return char2::all(uchar2::eq(*self, *other));
  }

  #[inline]
  fn ne(&self, other: &Self) -> bool {
    return char2::all(uchar2::ne(*self, *other));
  }
}

impl Dot for uchar2 {
  type Output = u8;

  #[inline]
  fn dot(self, other: uchar2) -> u8 {
    return uchar2::reduce_add(self * other);
  }
}

impl uchar2 {
  #[inline]
  pub fn bitcast<T>(x: T) -> uchar2 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn broadcast(x: u8) -> uchar2 {
    return uchar2(x, x);
  }

  #[inline]
  pub fn extract(self, i: u32) -> u8 {
    return unsafe { simd_extract(self, i) };
  }

  #[inline]
  pub fn replace(self, i: u32, x: u8) -> uchar2 {
    return unsafe { simd_insert(self, i, x) };
  }

  #[inline]
  pub fn eq(x: uchar2, y: uchar2) -> char2 {
    return unsafe { simd_eq(x, y) };
  }

  #[inline]
  pub fn ne(x: uchar2, y: uchar2) -> char2 {
    return unsafe { simd_ne(x, y) };
  }

  #[inline]
  pub fn lt(x: uchar2, y: uchar2) -> char2 {
    return unsafe { simd_lt(x, y) };
  }

  #[inline]
  pub fn le(x: uchar2, y: uchar2) -> char2 {
    return unsafe { simd_le(x, y) };
  }

  #[inline]
  pub fn gt(x: uchar2, y: uchar2) -> char2 {
    return unsafe { simd_gt(x, y) };
  }

  #[inline]
  pub fn ge(x: uchar2, y: uchar2) -> char2 {
    return unsafe { simd_ge(x, y) };
  }

  #[inline]
  pub fn madd(x: uchar2, y: uchar2, z: uchar2) -> uchar2 {
    return x * y + z;
  }

  #[inline]
  pub fn abs(x: uchar2) -> uchar2 {
    return x;
  }

  #[inline]
  pub fn max(x: uchar2, y: uchar2) -> uchar2 {
    return uchar2::bitselect(x, y, uchar2::gt(y, x));
  }

  #[inline]
  pub fn min(x: uchar2, y: uchar2) -> uchar2 {
    return uchar2::bitselect(x, y, uchar2::lt(y, x));
  }

  #[inline]
  pub fn clamp(x: uchar2, min: uchar2, max: uchar2) -> uchar2 {
    return uchar2::min(uchar2::max(x, min), max);
  }

  #[inline]
  pub fn reduce_add(x: uchar2) -> u8 {
    return x.0 + x.1;
  }

  #[inline]
  pub fn reduce_min(x: uchar2) -> u8 {
    return std::cmp::min(x.0, x.1);
  }

  #[inline]
  pub fn reduce_max(x: uchar2) -> u8 {
    return std::cmp::max(x.0, x.1);
  }

  #[inline]
  pub fn all(x: uchar2) -> bool {
    return (x.0 & x.1) & 0x80 != 0;
  }

  #[inline]
  pub fn any(x: uchar2) -> bool {
    return (x.0 | x.1) & 0x80 != 0;
  }

  #[inline]
  pub fn bitselect(x: uchar2, y: uchar2, z: char2) -> uchar2 {
    return uchar2::bitcast(char2::bitselect(char2::bitcast(x), char2::bitcast(y), z));
  }

  #[inline]
  pub fn lo(self) -> u8 {
    return self.0;
  }

  #[inline]
  pub fn hi(self) -> u8 {
    return self.1;
  }

  #[inline]
  pub fn odd(self) -> u8 {
    return self.1;
  }

  #[inline]
  pub fn even(self) -> u8 {
    return self.0;
  }
}
