use std;
use ::*;

impl Vector for uchar2 {
  type Scalar = u8;
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

  const ZERO: Self = uchar2(0, 0);
  const ONE: Self = uchar2(1, 1);
  const TWO: Self = uchar2(2, 2);
  const THREE: Self = uchar2(3, 3);

  #[inline(always)]
  fn abs(self) -> Self {
    return self;
  }

  #[inline(always)]
  fn max(self, other: Self) -> Self {
    return bitselect(gt(other, self), self, other);
  }

  #[inline(always)]
  fn min(self, other: Self) -> Self {
    return bitselect(lt(other, self), self, other);
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
    return uchar2::to_char(min(self, uchar2::broadcast(std::i8::MAX as u8)));
  }

  #[inline(always)]
  fn to_uchar_sat(self) -> uchar2 {
    return self;
  }

  #[inline(always)]
  fn to_short_sat(self) -> short2 {
    return uchar2::to_short(min(self, uchar2::broadcast(std::i16::MAX as u8)));
  }

  #[inline(always)]
  fn to_ushort_sat(self) -> ushort2 {
    return uchar2::to_ushort(self);
  }

  #[inline(always)]
  fn to_int_sat(self) -> int2 {
    return uchar2::to_int(min(self, uchar2::broadcast(std::i32::MAX as u8)));
  }

  #[inline(always)]
  fn to_uint_sat(self) -> uint2 {
    return uchar2::to_uint(self);
  }

  #[inline(always)]
  fn to_long_sat(self) -> long2 {
    return uchar2::to_long(min(self, uchar2::broadcast(std::i64::MAX as u8)));
  }

  #[inline(always)]
  fn to_ulong_sat(self) -> ulong2 {
    return uchar2::to_ulong(self);
  }
}

impl Dot<uchar2> for uchar2 {
  type DotProduct = u8;
  #[inline(always)]
  fn dot(self, other: Self) -> Self::DotProduct {
    return reduce_add(self * other);
  }
}

impl Integer for uchar2 {
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
    return self.reduce_and() & 0x80 != 0;
  }

  #[inline(always)]
  fn any(self) -> bool {
    return self.reduce_or() & 0x80 != 0;
  }
}

impl uchar2 {
  #[inline]
  pub fn bitcast<T>(x: T) -> uchar2 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn broadcast(x: u8) -> Self {
    return uchar2(x, x);
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
