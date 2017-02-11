use std;
use ::*;
use ::simd::*;

extern "platform-intrinsic" {
  fn simd_shl<T>(x: T, y: T) -> T;
  fn simd_shr<T>(x: T, y: T) -> T;
}

impl std::ops::Shl<ushort2> for ushort2 {
  type Output = Self;

  #[inline]
  fn shl(self, other: Self) -> Self {
    return unsafe { simd_shl(self, other) };
  }
}

impl std::ops::Shl<u16> for ushort2 {
  type Output = Self;

  #[inline]
  fn shl(self, other: u16) -> Self {
    return unsafe { simd_shl(self, ushort2::broadcast(other)) };
  }
}

impl std::ops::Shl<ushort2> for u16 {
  type Output = ushort2;

  #[inline]
  fn shl(self, other: ushort2) -> ushort2 {
    return unsafe { simd_shl(ushort2::broadcast(self), other) };
  }
}

impl std::ops::Shr<ushort2> for ushort2 {
  type Output = Self;

  #[inline]
  fn shr(self, other: Self) -> Self {
    return unsafe { simd_shr(self, other) };
  }
}

impl std::ops::Shr<u16> for ushort2 {
  type Output = Self;

  #[inline]
  fn shr(self, other: u16) -> Self {
    return unsafe { simd_shr(self, ushort2::broadcast(other)) };
  }
}

impl std::ops::Shr<ushort2> for u16 {
  type Output = ushort2;

  #[inline]
  fn shr(self, other: ushort2) -> ushort2 {
    return unsafe { simd_shr(ushort2::broadcast(self), other) };
  }
}

impl std::ops::Not for ushort2 {
  type Output = Self;

  #[inline]
  fn not(self) -> Self {
    return self ^ std::u16::MAX;
  }
}

impl PartialEq for ushort2 {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    return simd::eq(*self, *other).all();
  }

  #[inline]
  fn ne(&self, other: &Self) -> bool {
    return simd::ne(*self, *other).all();
  }
}

impl simd::Vector for ushort2 {
  type Scalar = u16;
  type Boolean = short2;

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
    return self;
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
    return ushort2::to_char(simd::min(self, ushort2::broadcast(std::i8::MAX as u16)));
  }

  #[inline(always)]
  fn to_uchar_sat(self) -> uchar2 {
    return ushort2::to_uchar(simd::min(self, ushort2::broadcast(std::u8::MAX as u16)));
  }

  #[inline(always)]
  fn to_short_sat(self) -> short2 {
    return ushort2::to_short(simd::min(self, ushort2::broadcast(std::i16::MAX as u16)));
  }

  #[inline(always)]
  fn to_ushort_sat(self) -> ushort2 {
    return self;
  }

  #[inline(always)]
  fn to_int_sat(self) -> int2 {
    return ushort2::to_int(simd::min(self, ushort2::broadcast(std::i32::MAX as u16)));
  }

  #[inline(always)]
  fn to_uint_sat(self) -> uint2 {
    return ushort2::to_uint(self);
  }

  #[inline(always)]
  fn to_long_sat(self) -> long2 {
    return ushort2::to_long(simd::min(self, ushort2::broadcast(std::i64::MAX as u16)));
  }

  #[inline(always)]
  fn to_ulong_sat(self) -> ulong2 {
    return ushort2::to_ulong(self);
  }
}

impl simd::Dot for ushort2 {
  type DotProduct = u16;
  #[inline(always)]
  fn dot(self, other: Self) -> Self::DotProduct {
    return simd::reduce_add(self * other);
  }
}

impl simd::Integer for ushort2 {
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
    return self.reduce_and() & 0x8000 != 0;
  }

  #[inline(always)]
  fn any(self) -> bool {
    return self.reduce_or() & 0x8000 != 0;
  }
}

impl ushort2 {
  #[inline]
  pub fn bitcast<T>(x: T) -> ushort2 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn broadcast(x: u16) -> Self {
    return ushort2(x, x);
  }

  #[inline]
  pub fn lo(self) -> u16 {
    return self.0;
  }

  #[inline]
  pub fn hi(self) -> u16 {
    return self.1;
  }

  #[inline]
  pub fn odd(self) -> u16 {
    return self.1;
  }

  #[inline]
  pub fn even(self) -> u16 {
    return self.0;
  }
}
