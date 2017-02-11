use std;
use ::*;

#[repr(C)]
#[repr(simd)]
#[derive(Copy, Clone, Debug)]
pub struct char2(pub i8, pub i8);

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

  fn simd_cast<T, U>(x: T) -> U;

  fn simd_insert<T, E>(x: T, i: u32, e: E) -> T;
  fn simd_extract<T, E>(x: T, i: u32) -> E;
}

impl std::ops::Index<u32> for char2 {
  type Output = i8;

  #[inline]
  fn index(&self, index: u32) -> &i8 {
    return unsafe { simd_extract(self, index) };
  }
}

impl std::ops::Add for char2 {
  type Output = Self;

  #[inline]
  fn add(self, other: Self) -> Self {
    return unsafe { simd_add(self, other) };
  }
}

impl std::ops::Add<i8> for char2 {
  type Output = Self;

  #[inline]
  fn add(self, other: i8) -> Self {
    return unsafe { simd_add(self, char2::broadcast(other)) };
  }
}

impl std::ops::Add<char2> for i8 {
  type Output = char2;

  #[inline]
  fn add(self, other: char2) -> char2 {
    return unsafe { simd_add(char2::broadcast(self), other) };
  }
}

impl std::ops::Sub for char2 {
  type Output = Self;

  #[inline]
  fn sub(self, other: Self) -> Self {
    return unsafe { simd_sub(self, other) };
  }
}

impl std::ops::Sub<i8> for char2 {
  type Output = Self;

  #[inline]
  fn sub(self, other: i8) -> Self {
    return unsafe { simd_sub(self, char2::broadcast(other)) };
  }
}

impl std::ops::Sub<char2> for i8 {
  type Output = char2;

  #[inline]
  fn sub(self, other: char2) -> char2 {
    return unsafe { simd_sub(char2::broadcast(self), other) };
  }
}

impl std::ops::Mul for char2 {
  type Output = Self;

  #[inline]
  fn mul(self, other: Self) -> Self {
    return unsafe { simd_mul(self, other) };
  }
}

impl std::ops::Mul<i8> for char2 {
  type Output = Self;

  #[inline]
  fn mul(self, other: i8) -> Self {
    return unsafe { simd_mul(self, char2::broadcast(other)) };
  }
}

impl std::ops::Mul<char2> for i8 {
  type Output = char2;

  #[inline]
  fn mul(self, other: char2) -> char2 {
    return unsafe { simd_mul(char2::broadcast(self), other) };
  }
}

impl std::ops::Div for char2 {
  type Output = Self;

  #[inline]
  fn div(self, other: Self) -> Self {
    return unsafe { simd_div(self, other) };
  }
}

impl std::ops::Div<i8> for char2 {
  type Output = Self;

  #[inline]
  fn div(self, other: i8) -> Self {
    return unsafe { simd_div(self, char2::broadcast(other)) };
  }
}

impl std::ops::Div<char2> for i8 {
  type Output = char2;

  #[inline]
  fn div(self, other: char2) -> char2 {
    return unsafe { simd_div(char2::broadcast(self), other) };
  }
}

impl std::ops::BitAnd for char2 {
  type Output = Self;

  #[inline]
  fn bitand(self, other: Self) -> Self {
    return unsafe { simd_and(self, other) };
  }
}

impl std::ops::BitAnd<i8> for char2 {
  type Output = Self;

  #[inline]
  fn bitand(self, other: i8) -> Self {
    return unsafe { simd_and(self, char2::broadcast(other)) };
  }
}

impl std::ops::BitAnd<char2> for i8 {
  type Output = char2;

  #[inline]
  fn bitand(self, other: char2) -> char2 {
    return unsafe { simd_and(char2::broadcast(self), other) };
  }
}

impl std::ops::BitOr for char2 {
  type Output = Self;

  #[inline]
  fn bitor(self, other: Self) -> Self {
    return unsafe { simd_or(self, other) };
  }
}

impl std::ops::BitOr<i8> for char2 {
  type Output = Self;

  #[inline]
  fn bitor(self, other: i8) -> Self {
    return unsafe { simd_or(self, char2::broadcast(other)) };
  }
}

impl std::ops::BitOr<char2> for i8 {
  type Output = char2;

  #[inline]
  fn bitor(self, other: char2) -> char2 {
    return unsafe { simd_or(char2::broadcast(self), other) };
  }
}

impl std::ops::BitXor for char2 {
  type Output = Self;

  #[inline]
  fn bitxor(self, other: Self) -> Self {
    return unsafe { simd_xor(self, other) };
  }
}

impl std::ops::BitXor<i8> for char2 {
  type Output = Self;

  #[inline]
  fn bitxor(self, other: i8) -> Self {
    return unsafe { simd_xor(self, char2::broadcast(other)) };
  }
}

impl std::ops::BitXor<char2> for i8 {
  type Output = char2;

  #[inline]
  fn bitxor(self, other: char2) -> char2 {
    return unsafe { simd_xor(char2::broadcast(self), other) };
  }
}

impl std::ops::Shl<char2> for char2 {
  type Output = Self;

  #[inline]
  fn shl(self, other: Self) -> Self {
    return unsafe { simd_shl(self, other) };
  }
}

impl std::ops::Shl<i8> for char2 {
  type Output = Self;

  #[inline]
  fn shl(self, other: i8) -> Self {
    return unsafe { simd_shl(self, char2::broadcast(other)) };
  }
}

impl std::ops::Shl<char2> for i8 {
  type Output = char2;

  #[inline]
  fn shl(self, other: char2) -> char2 {
    return unsafe { simd_shl(char2::broadcast(self), other) };
  }
}

impl std::ops::Shr<char2> for char2 {
  type Output = Self;

  #[inline]
  fn shr(self, other: Self) -> Self {
    return unsafe { simd_shr(self, other) };
  }
}

impl std::ops::Shr<i8> for char2 {
  type Output = Self;

  #[inline]
  fn shr(self, other: i8) -> Self {
    return unsafe { simd_shr(self, char2::broadcast(other)) };
  }
}

impl std::ops::Shr<char2> for i8 {
  type Output = char2;

  #[inline]
  fn shr(self, other: char2) -> char2 {
    return unsafe { simd_shr(char2::broadcast(self), other) };
  }
}

impl std::ops::Not for char2 {
  type Output = Self;

  #[inline]
  fn not(self) -> Self {
    return self ^ -1;
  }
}

impl PartialEq for char2 {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    return simd::all(simd::eq(*self, *other));
  }

  #[inline]
  fn ne(&self, other: &Self) -> bool {
    return simd::all(simd::ne(*self, *other));
  }
}

impl simd::Vector for char2 {
  type Scalar = i8;
  type Boolean = char2;

  #[inline(always)]
  fn extract(self, i: u32) -> Self::Scalar {
    return unsafe { simd_extract(self, i) };
  }

  #[inline(always)]
  fn replace(self, i: u32, x: Self::Scalar) -> Self {
    return unsafe { simd_insert(self, i, x) };
  }

  #[inline(always)]
  fn eq(self, other: Self) -> Self::Boolean {
    return unsafe { simd_eq(self, other) };
  }

  #[inline(always)]
  fn ne(self, other: Self) -> Self::Boolean {
    return unsafe { simd_ne(self, other) };
  }

  #[inline(always)]
  fn lt(self, other: Self) -> Self::Boolean {
    return unsafe { simd_lt(self, other) };
  }

  #[inline(always)]
  fn le(self, other: Self) -> Self::Boolean {
    return unsafe { simd_le(self, other) };
  }

  #[inline(always)]
  fn gt(self, other: Self) -> Self::Boolean {
    return unsafe { simd_gt(self, other) };
  }

  #[inline(always)]
  fn ge(self, other: Self) -> Self::Boolean {
    return unsafe { simd_ge(self, other) };
  }

  #[inline(always)]
  fn abs(self) -> Self {
    let mask = self >> 7;
    return (self ^ mask) - mask;
  }

  #[inline(always)]
  fn max(self, other: Self) -> Self {
    return simd::bitselect(simd::gt(other, self), self, other);
  }

  #[inline(always)]
  fn min(self, other: Self) -> Self {
    return simd::bitselect(simd::lt(other, self), self, other);
  }
}

impl simd::Dot for char2 {
  type Output = i8;

  #[inline(always)]
  fn dot(self, other: Self) -> Self::Output {
    return simd::reduce_add(self * other);
  }
}

impl simd::Logic for char2 {
  #[inline(always)]
  fn all(self) -> bool {
    return (self.0 & self.1) & std::i8::MIN != 0;
  }

  #[inline(always)]
  fn any(self) -> bool {
    return (self.0 | self.1) & std::i8::MIN != 0;
  }
}

impl simd::Reduce for char2 {
  #[inline(always)]
  fn reduce_add(self) -> Self::Scalar {
    return self.0 + self.1;
  }

  #[inline(always)]
  fn reduce_min(self) -> Self::Scalar {
    return std::cmp::min(self.0, self.1);
  }

  #[inline(always)]
  fn reduce_max(self) -> Self::Scalar {
    return std::cmp::max(self.0, self.1);
  }
}

impl simd::Select<char2> for char2 {
  #[inline(always)]
  fn select(self, a: char2, b: char2) -> char2 {
    return (self >> 7).bitselect(a, b);
  }

  #[inline(always)]
  fn bitselect(self, a: char2, b: char2) -> char2 {
    return (a & !self) | (b & self);
  }
}

impl simd::Select<uchar2> for char2 {
  #[inline(always)]
  fn select(self, a: uchar2, b: uchar2) -> uchar2 {
    return (self >> 7).bitselect(a, b);
  }

  #[inline(always)]
  fn bitselect(self, a: uchar2, b: uchar2) -> uchar2 {
    return uchar2::bitcast(self.bitselect(char2::bitcast(a), char2::bitcast(b)));
  }
}

impl char2 {
  #[inline]
  pub fn bitcast<T>(x: T) -> char2 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn broadcast(x: i8) -> Self {
    return char2(x, x);
  }

  #[inline]
  pub fn madd(x: char2, y: char2, z: char2) -> char2 {
    return x * y + z;
  }

  #[inline]
  pub fn to_char(x: char2) -> char2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_char_sat(x: char2) -> char2 {
    return x;
  }

  #[inline]
  pub fn to_uchar(x: char2) -> uchar2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_uchar_sat(x: char2) -> uchar2 {
    return char2::to_uchar(simd::max(x, char2::broadcast(0)));
  }

  #[inline]
  pub fn to_short(x: char2) -> short2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_short_sat(x: char2) -> short2 {
    return char2::to_short(x);
  }

  #[inline]
  pub fn to_ushort(x: char2) -> ushort2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_ushort_sat(x: char2) -> ushort2 {
    return char2::to_ushort(simd::max(x, char2::broadcast(0)));
  }

  #[inline]
  pub fn to_int(x: char2) -> int2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_int_sat(x: char2) -> int2 {
    return char2::to_int(x);
  }

  #[inline]
  pub fn to_uint(x: char2) -> uint2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_uint_sat(x: char2) -> uint2 {
    return char2::to_uint(simd::max(x, char2::broadcast(0)));
  }

  #[inline]
  pub fn to_float(x: char2) -> float2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_long(x: char2) -> long2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_long_sat(x: char2) -> long2 {
    return char2::to_long(x);
  }

  #[inline]
  pub fn to_ulong(x: char2) -> ulong2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_ulong_sat(x: char2) -> ulong2 {
    return char2::to_ulong(simd::max(x, char2::broadcast(0)));
  }

  #[inline]
  pub fn to_double(x: char2) -> double2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn lo(self) -> i8 {
    return self.0;
  }

  #[inline]
  pub fn hi(self) -> i8 {
    return self.1;
  }

  #[inline]
  pub fn odd(self) -> i8 {
    return self.1;
  }

  #[inline]
  pub fn even(self) -> i8 {
    return self.0;
  }
}
