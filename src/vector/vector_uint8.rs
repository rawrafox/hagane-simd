use std;
use ::*;

impl Vector for uint8 {
  type Scalar = u32;
  type Boolean = int8;

  type CharVector = char8;
  type ShortVector = short8;
  type IntVector = int8;
  type LongVector = long8;

  type UCharVector = uchar8;
  type UShortVector = ushort8;
  type UIntVector = uint8;
  type ULongVector = ulong8;

  type FloatVector = float8;
  type DoubleVector = double8;

  const ZERO: Self = uint8(0, 0, 0, 0, 0, 0, 0, 0);
  const ONE: Self = uint8(1, 1, 1, 1, 1, 1, 1, 1);
  const TWO: Self = uint8(2, 2, 2, 2, 2, 2, 2, 2);
  const THREE: Self = uint8(3, 3, 3, 3, 3, 3, 3, 3);

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
    return reduce_add(self.lo() + self.hi());
  }

  #[inline(always)]
  fn reduce_min(self) -> Self::Scalar {
    return reduce_min(min(self.lo(), self.hi()));
  }

  #[inline(always)]
  fn reduce_max(self) -> Self::Scalar {
    return reduce_max(max(self.lo(), self.hi()));
  }

  #[inline(always)]
  fn to_char_sat(self) -> char8 {
    return uint8::to_char(min(self, broadcast(std::i8::MAX as u32)));
  }

  #[inline(always)]
  fn to_uchar_sat(self) -> uchar8 {
    return uint8::to_uchar(min(self, broadcast(std::u8::MAX as u32)));
  }

  #[inline(always)]
  fn to_short_sat(self) -> short8 {
    return uint8::to_short(min(self, broadcast(std::i16::MAX as u32)));
  }

  #[inline(always)]
  fn to_ushort_sat(self) -> ushort8 {
    return uint8::to_ushort(min(self, broadcast(std::u16::MAX as u32)));
  }

  #[inline(always)]
  fn to_int_sat(self) -> int8 {
    return uint8::to_int(min(self, broadcast(std::i32::MAX as u32)));
  }

  #[inline(always)]
  fn to_uint_sat(self) -> uint8 {
    return self;
  }

  #[inline(always)]
  fn to_long_sat(self) -> long8 {
    return uint8::to_long(min(self, broadcast(std::i64::MAX as u32)));
  }

  #[inline(always)]
  fn to_ulong_sat(self) -> ulong8 {
    return uint8::to_ulong(self);
  }
}

impl Dot<uint8> for uint8 {
  type DotProduct = u32;
  #[inline(always)]
  fn dot(self, other: Self) -> Self::DotProduct {
    return reduce_add(self * other);
  }
}

impl Integer for uint8 {
  #[inline(always)]
  fn reduce_and(self) -> Self::Scalar {
    return (self.lo() & self.hi()).reduce_and();
  }

  #[inline(always)]
  fn reduce_or(self) -> Self::Scalar {
    return (self.lo() | self.hi()).reduce_or();
  }

  #[inline(always)]
  fn reduce_xor(self) -> Self::Scalar {
    return (self.lo() ^ self.hi()).reduce_xor();
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

impl uint8 {
  #[inline]
  pub fn bitcast<T>(x: T) -> uint8 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn lo(self) -> uint4 {
    return uint4(self.0, self.1, self.2, self.3);
  }

  #[inline]
  pub fn hi(self) -> uint4 {
    return uint4(self.4, self.5, self.6, self.7);
  }

  #[inline]
  pub fn odd(self) -> uint4 {
    return uint4(self.1, self.3, self.5, self.7);
  }

  #[inline]
  pub fn even(self) -> uint4 {
    return uint4(self.0, self.2, self.4, self.6);
  }
}
