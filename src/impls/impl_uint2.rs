use std;
use ::*;

impl simd::Vector for uint2 {
  type Scalar = u32;
  type Boolean = int2;

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
    return uint2::to_char(simd::min(self, uint2::broadcast(std::i8::MAX as u32)));
  }

  #[inline(always)]
  fn to_uchar_sat(self) -> uchar2 {
    return uint2::to_uchar(simd::min(self, uint2::broadcast(std::u8::MAX as u32)));
  }

  #[inline(always)]
  fn to_short_sat(self) -> short2 {
    return uint2::to_short(simd::min(self, uint2::broadcast(std::i16::MAX as u32)));
  }

  #[inline(always)]
  fn to_ushort_sat(self) -> ushort2 {
    return uint2::to_ushort(simd::min(self, uint2::broadcast(std::u16::MAX as u32)));
  }

  #[inline(always)]
  fn to_int_sat(self) -> int2 {
    return uint2::to_int(simd::min(self, uint2::broadcast(std::i32::MAX as u32)));
  }

  #[inline(always)]
  fn to_uint_sat(self) -> uint2 {
    return self;
  }

  #[inline(always)]
  fn to_long_sat(self) -> long2 {
    return uint2::to_long(simd::min(self, uint2::broadcast(std::i64::MAX as u32)));
  }

  #[inline(always)]
  fn to_ulong_sat(self) -> ulong2 {
    return uint2::to_ulong(self);
  }
}

impl simd::Dot for uint2 {
  type DotProduct = u32;
  #[inline(always)]
  fn dot(self, other: Self) -> Self::DotProduct {
    return simd::reduce_add(self * other);
  }
}

impl simd::Integer for uint2 {
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
    return self.reduce_and() & 0x80000000 != 0;
  }

  #[inline(always)]
  fn any(self) -> bool {
    return self.reduce_or() & 0x80000000 != 0;
  }
}

impl uint2 {
  #[inline]
  pub fn bitcast<T>(x: T) -> uint2 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn broadcast(x: u32) -> Self {
    return uint2(x, x);
  }

  #[inline]
  pub fn lo(self) -> u32 {
    return self.0;
  }

  #[inline]
  pub fn hi(self) -> u32 {
    return self.1;
  }

  #[inline]
  pub fn odd(self) -> u32 {
    return self.1;
  }

  #[inline]
  pub fn even(self) -> u32 {
    return self.0;
  }
}
