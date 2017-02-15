use std;
use ::*;

impl Vector for long2 {
  type Scalar = i64;
  type Boolean = long2;

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
    let mask = self >> 63;

    return (self ^ mask) - mask;
  }

  #[inline(always)]
  fn max(self, other: Self) -> Self {
    return gt(other, self).bitselect(self, other);
  }

  #[inline(always)]
  fn min(self, other: Self) -> Self {
    return lt(other, self).bitselect(self, other);
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
    return long2::to_char(self.clamp(Self::broadcast(std::i8::MIN as i64), Self::broadcast(std::i8::MAX as i64)));
  }

  #[inline(always)]
  fn to_uchar_sat(self) -> uchar2 {
    return long2::to_uchar(self.clamp(Self::broadcast(std::u8::MIN as i64), Self::broadcast(std::u8::MAX as i64)));
  }

  #[inline(always)]
  fn to_short_sat(self) -> short2 {
    return long2::to_short(self.clamp(Self::broadcast(std::i16::MIN as i64), Self::broadcast(std::i16::MAX as i64)));
  }

  #[inline(always)]
  fn to_ushort_sat(self) -> ushort2 {
    return long2::to_ushort(self.clamp(Self::broadcast(std::u16::MIN as i64), Self::broadcast(std::u16::MAX as i64)));
  }

  #[inline(always)]
  fn to_int_sat(self) -> int2 {
    return long2::to_int(self.clamp(Self::broadcast(std::i32::MIN as i64), Self::broadcast(std::i32::MAX as i64)));
  }

  #[inline(always)]
  fn to_uint_sat(self) -> uint2 {
    return long2::to_uint(self.clamp(Self::broadcast(std::u32::MIN as i64), Self::broadcast(std::u32::MAX as i64)));
  }

  #[inline(always)]
  fn to_long_sat(self) -> long2 {
    return self;
  }

  #[inline(always)]
  fn to_ulong_sat(self) -> ulong2 {
    return long2::to_ulong(self.max(Self::from(0)));
  }
}

impl Dot<long2> for long2 {
  type DotProduct = i64;
  #[inline(always)]
  fn dot(self, other: Self) -> Self::DotProduct {
    return reduce_add(self * other);
  }
}

impl Integer for long2 {
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
    return self.reduce_and() & std::i64::MIN != 0;
  }

  #[inline(always)]
  fn any(self) -> bool {
    return self.reduce_or() & std::i64::MIN != 0;
  }
}

impl Select<long2> for long2 {
  #[inline(always)]
  fn select(self, a: long2, b: long2) -> long2 {
    return (self >> 63).bitselect(a, b);
  }

  #[inline(always)]
  fn bitselect(self, a: long2, b: long2) -> long2 {
    return (a & !self) | (b & self);
  }
}

impl Select<ulong2> for long2 {
  #[inline(always)]
  fn select(self, a: ulong2, b: ulong2) -> ulong2 {
    return (self >> 63).bitselect(a, b);
  }

  #[inline(always)]
  fn bitselect(self, a: ulong2, b: ulong2) -> ulong2 {
    return ulong2::bitcast(self.bitselect(long2::bitcast(a), long2::bitcast(b)));
  }
}

impl Select<double2> for long2 {
  #[inline(always)]
  fn select(self, a: double2, b: double2) -> double2 {
    return (self >> 63).bitselect(a, b);
  }

  #[inline(always)]
  fn bitselect(self, a: double2, b: double2) -> double2 {
    return double2::bitcast(self.bitselect(long2::bitcast(a), long2::bitcast(b)));
  }
}

impl long2 {
  #[inline]
  pub fn bitcast<T>(x: T) -> long2 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn lo(self) -> i64 {
    return self.0;
  }

  #[inline]
  pub fn hi(self) -> i64 {
    return self.1;
  }

  #[inline]
  pub fn odd(self) -> i64 {
    return self.1;
  }

  #[inline]
  pub fn even(self) -> i64 {
    return self.0;
  }
}
