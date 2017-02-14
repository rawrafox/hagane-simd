use std;
use ::*;

impl Vector for short4 {
  type Scalar = i16;
  type Boolean = short4;

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
    let mask = self >> 15;

    return (self ^ mask) - mask;
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
  fn to_char_sat(self) -> char4 {
    return short4::to_char(clamp(self, broadcast(std::i8::MIN as i16), broadcast(std::i8::MAX as i16)));
  }

  #[inline(always)]
  fn to_uchar_sat(self) -> uchar4 {
    return short4::to_uchar(clamp(self, broadcast(std::u8::MIN as i16), broadcast(std::u8::MAX as i16)));
  }

  #[inline(always)]
  fn to_short_sat(self) -> short4 {
    return self;
  }

  #[inline(always)]
  fn to_ushort_sat(self) -> ushort4 {
    return short4::to_ushort(max(self, broadcast::<isize, Self>(0isize)));
  }

  #[inline(always)]
  fn to_int_sat(self) -> int4 {
    return short4::to_int(self);
  }

  #[inline(always)]
  fn to_uint_sat(self) -> uint4 {
    return short4::to_uint(max(self, broadcast::<isize, Self>(0isize)));
  }

  #[inline(always)]
  fn to_long_sat(self) -> long4 {
    return short4::to_long(self);
  }

  #[inline(always)]
  fn to_ulong_sat(self) -> ulong4 {
    return short4::to_ulong(max(self, broadcast::<isize, Self>(0isize)));
  }
}

impl Dot<short4> for short4 {
  type DotProduct = i16;
  #[inline(always)]
  fn dot(self, other: Self) -> Self::DotProduct {
    return reduce_add(self * other);
  }
}

impl Integer for short4 {
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
    return self.reduce_and() & std::i16::MIN != 0;
  }

  #[inline(always)]
  fn any(self) -> bool {
    return self.reduce_or() & std::i16::MIN != 0;
  }
}

impl Select<short4> for short4 {
  #[inline(always)]
  fn select(self, a: short4, b: short4) -> short4 {
    return (self >> 15).bitselect(a, b);
  }

  #[inline(always)]
  fn bitselect(self, a: short4, b: short4) -> short4 {
    return (a & !self) | (b & self);
  }
}

impl Select<ushort4> for short4 {
  #[inline(always)]
  fn select(self, a: ushort4, b: ushort4) -> ushort4 {
    return (self >> 15).bitselect(a, b);
  }

  #[inline(always)]
  fn bitselect(self, a: ushort4, b: ushort4) -> ushort4 {
    return ushort4::bitcast(self.bitselect(short4::bitcast(a), short4::bitcast(b)));
  }
}

impl short4 {
  #[inline]
  pub fn bitcast<T>(x: T) -> short4 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn lo(self) -> short2 {
    return short2(self.0, self.1);
  }

  #[inline]
  pub fn hi(self) -> short2 {
    return short2(self.2, self.3);
  }

  #[inline]
  pub fn odd(self) -> short2 {
    return short2(self.1, self.3);
  }

  #[inline]
  pub fn even(self) -> short2 {
    return short2(self.0, self.2);
  }
}