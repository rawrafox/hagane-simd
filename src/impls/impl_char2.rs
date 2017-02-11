use std;
use ::*;
use ::simd::*;

extern "platform-intrinsic" {
  fn simd_shl<T>(x: T, y: T) -> T;
  fn simd_shr<T>(x: T, y: T) -> T;

  fn simd_and<T>(x: T, y: T) -> T;
  fn simd_or<T>(x: T, y: T) -> T;
  fn simd_xor<T>(x: T, y: T) -> T;
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
    return simd::eq(*self, *other).all();
  }

  #[inline]
  fn ne(&self, other: &Self) -> bool {
    return simd::ne(*self, *other).all();
  }
}

impl simd::Vector for char2 {
  type Scalar = i8;
  type Boolean = char2;

  type CharVector = char2;
  type ShortVector = short2;
  type IntVector = int2;
  type LongVector = long2;

  type UCharVector = uchar2;
  type UShortVector = ushort2;
  type UIntVector = uint2;
  type ULongVector = ulong2;

  type FloatVector = float2;
  type DoubleVector = double2;

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

  #[inline(always)]
  fn to_char_sat(self) -> char2 {
    return self;
  }

  #[inline(always)]
  fn to_uchar_sat(self) -> uchar2 {
    return char2::to_uchar(simd::max(self, char2::broadcast(0)));
  }

  #[inline(always)]
  fn to_short_sat(self) -> short2 {
    return char2::to_short(self);
  }

  #[inline(always)]
  fn to_ushort_sat(self) -> ushort2 {
    return char2::to_ushort(simd::max(self, char2::broadcast(0)));
  }

  #[inline(always)]
  fn to_int_sat(self) -> int2 {
    return char2::to_int(self);
  }

  #[inline(always)]
  fn to_uint_sat(self) -> uint2 {
    return char2::to_uint(simd::max(self, char2::broadcast(0)));
  }

  #[inline(always)]
  fn to_long_sat(self) -> long2 {
    return char2::to_long(self);
  }

  #[inline(always)]
  fn to_ulong_sat(self) -> ulong2 {
    return char2::to_ulong(simd::max(self, char2::broadcast(0)));
  }
}

impl simd::Dot for char2 {
  type DotProduct = i8;
  #[inline(always)]
  fn dot(self, other: Self) -> Self::DotProduct {
    return simd::reduce_add(self * other);
  }
}

impl simd::Integer for char2 {
  #[inline(always)]
  fn reduce_and(self) -> Self::Scalar {
    return self.0 & self.1
  }

  #[inline(always)]
  fn reduce_or(self) -> Self::Scalar {
    return self.0 | self.1
  }

  #[inline(always)]
  fn reduce_xor(self) -> Self::Scalar {
    return self.0 ^ self.1
  }

  #[inline(always)]
  fn all(self) -> bool {
    return self.reduce_and() & std::i8::MIN != 0;
  }

  #[inline(always)]
  fn any(self) -> bool {
    return self.reduce_or() & std::i8::MIN != 0;
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
