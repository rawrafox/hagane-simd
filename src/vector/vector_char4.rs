use std;
use ::*;

impl Vector for char4 {
  type Scalar = i8;
  type Boolean = char4;

  type CharVector = char4;
  type ShortVector = short4;
  type IntVector = int4;
  type LongVector = long4;

  type UCharVector = uchar4;
  type UShortVector = ushort4;
  type UIntVector = uint4;
  type ULongVector = ulong4;

  type FloatVector = float4;
  type DoubleVector = double4;

  #[inline(always)]
  fn abs(self) -> Self {
    let mask = self >> 7;

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
  fn to_char_sat(self) -> char4 {
    return self;
  }

  #[inline(always)]
  fn to_uchar_sat(self) -> uchar4 {
    return char4::to_uchar(self.max(Self::from(0)));
  }

  #[inline(always)]
  fn to_short_sat(self) -> short4 {
    return char4::to_short(self);
  }

  #[inline(always)]
  fn to_ushort_sat(self) -> ushort4 {
    return char4::to_ushort(self.max(Self::from(0)));
  }

  #[inline(always)]
  fn to_int_sat(self) -> int4 {
    return char4::to_int(self);
  }

  #[inline(always)]
  fn to_uint_sat(self) -> uint4 {
    return char4::to_uint(self.max(Self::from(0)));
  }

  #[inline(always)]
  fn to_long_sat(self) -> long4 {
    return char4::to_long(self);
  }

  #[inline(always)]
  fn to_ulong_sat(self) -> ulong4 {
    return char4::to_ulong(self.max(Self::from(0)));
  }
}

impl Dot<char4> for char4 {
  type DotProduct = i8;
  #[inline(always)]
  fn dot(self, other: Self) -> Self::DotProduct {
    return reduce_add(self * other);
  }
}

impl Integer for char4 {
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
    return self.reduce_and() & std::i8::MIN != 0;
  }

  #[inline(always)]
  fn any(self) -> bool {
    return self.reduce_or() & std::i8::MIN != 0;
  }
}

impl Select<char4> for char4 {
  #[inline(always)]
  fn select(self, a: char4, b: char4) -> char4 {
    return (self >> 7).bitselect(a, b);
  }

  #[inline(always)]
  fn bitselect(self, a: char4, b: char4) -> char4 {
    return (a & !self) | (b & self);
  }
}

impl Select<uchar4> for char4 {
  #[inline(always)]
  fn select(self, a: uchar4, b: uchar4) -> uchar4 {
    return (self >> 7).bitselect(a, b);
  }

  #[inline(always)]
  fn bitselect(self, a: uchar4, b: uchar4) -> uchar4 {
    return uchar4::bitcast(self.bitselect(char4::bitcast(a), char4::bitcast(b)));
  }
}

impl char4 {
  #[inline]
  pub fn bitcast<T>(x: T) -> char4 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn lo(self) -> char2 {
    return char2(self.0, self.1);
  }

  #[inline]
  pub fn hi(self) -> char2 {
    return char2(self.2, self.3);
  }

  #[inline]
  pub fn odd(self) -> char2 {
    return char2(self.1, self.3);
  }

  #[inline]
  pub fn even(self) -> char2 {
    return char2(self.0, self.2);
  }
}
