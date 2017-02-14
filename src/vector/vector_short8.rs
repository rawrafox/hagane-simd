use std;
use ::*;

impl Vector for short8 {
  type Scalar = i16;
  type Boolean = short8;

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
  fn to_char_sat(self) -> char8 {
    return short8::to_char(clamp(self, broadcast(std::i8::MIN as i16), broadcast(std::i8::MAX as i16)));
  }

  #[inline(always)]
  fn to_uchar_sat(self) -> uchar8 {
    return short8::to_uchar(clamp(self, broadcast(std::u8::MIN as i16), broadcast(std::u8::MAX as i16)));
  }

  #[inline(always)]
  fn to_short_sat(self) -> short8 {
    return self;
  }

  #[inline(always)]
  fn to_ushort_sat(self) -> ushort8 {
    return short8::to_ushort(max(self, broadcast::<isize, Self>(0isize)));
  }

  #[inline(always)]
  fn to_int_sat(self) -> int8 {
    return short8::to_int(self);
  }

  #[inline(always)]
  fn to_uint_sat(self) -> uint8 {
    return short8::to_uint(max(self, broadcast::<isize, Self>(0isize)));
  }

  #[inline(always)]
  fn to_long_sat(self) -> long8 {
    return short8::to_long(self);
  }

  #[inline(always)]
  fn to_ulong_sat(self) -> ulong8 {
    return short8::to_ulong(max(self, broadcast::<isize, Self>(0isize)));
  }
}

impl Dot<short8> for short8 {
  type DotProduct = i16;
  #[inline(always)]
  fn dot(self, other: Self) -> Self::DotProduct {
    return reduce_add(self * other);
  }
}

impl Integer for short8 {
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

impl Select<short8> for short8 {
  #[inline(always)]
  fn select(self, a: short8, b: short8) -> short8 {
    return (self >> 15).bitselect(a, b);
  }

  #[inline(always)]
  fn bitselect(self, a: short8, b: short8) -> short8 {
    return (a & !self) | (b & self);
  }
}

impl Select<ushort8> for short8 {
  #[inline(always)]
  fn select(self, a: ushort8, b: ushort8) -> ushort8 {
    return (self >> 15).bitselect(a, b);
  }

  #[inline(always)]
  fn bitselect(self, a: ushort8, b: ushort8) -> ushort8 {
    return ushort8::bitcast(self.bitselect(short8::bitcast(a), short8::bitcast(b)));
  }
}

impl short8 {
  #[inline]
  pub fn bitcast<T>(x: T) -> short8 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn lo(self) -> short4 {
    return short4(self.0, self.1, self.2, self.3);
  }

  #[inline]
  pub fn hi(self) -> short4 {
    return short4(self.4, self.5, self.6, self.7);
  }

  #[inline]
  pub fn odd(self) -> short4 {
    return short4(self.1, self.3, self.5, self.7);
  }

  #[inline]
  pub fn even(self) -> short4 {
    return short4(self.0, self.2, self.4, self.6);
  }
}