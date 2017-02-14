use std;
use ::*;

impl Vector for long4 {
  type Scalar = i64;
  type Boolean = long4;

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
    let mask = self >> 63;

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
    return long4::to_char(clamp(self, broadcast(std::i8::MIN as i64), broadcast(std::i8::MAX as i64)));
  }

  #[inline(always)]
  fn to_uchar_sat(self) -> uchar4 {
    return long4::to_uchar(clamp(self, broadcast(std::u8::MIN as i64), broadcast(std::u8::MAX as i64)));
  }

  #[inline(always)]
  fn to_short_sat(self) -> short4 {
    return long4::to_short(clamp(self, broadcast(std::i16::MIN as i64), broadcast(std::i16::MAX as i64)));
  }

  #[inline(always)]
  fn to_ushort_sat(self) -> ushort4 {
    return long4::to_ushort(clamp(self, broadcast(std::u16::MIN as i64), broadcast(std::u16::MAX as i64)));
  }

  #[inline(always)]
  fn to_int_sat(self) -> int4 {
    return long4::to_int(clamp(self, broadcast(std::i32::MIN as i64), broadcast(std::i32::MAX as i64)));
  }

  #[inline(always)]
  fn to_uint_sat(self) -> uint4 {
    return long4::to_uint(clamp(self, broadcast(std::u32::MIN as i64), broadcast(std::u32::MAX as i64)));
  }

  #[inline(always)]
  fn to_long_sat(self) -> long4 {
    return self;
  }

  #[inline(always)]
  fn to_ulong_sat(self) -> ulong4 {
    return long4::to_ulong(max(self, broadcast::<isize, Self>(0isize)));
  }
}

impl Dot<long4> for long4 {
  type DotProduct = i64;
  #[inline(always)]
  fn dot(self, other: Self) -> Self::DotProduct {
    return reduce_add(self * other);
  }
}

impl Integer for long4 {
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
    return self.reduce_and() & std::i64::MIN != 0;
  }

  #[inline(always)]
  fn any(self) -> bool {
    return self.reduce_or() & std::i64::MIN != 0;
  }
}

impl Select<long4> for long4 {
  #[inline(always)]
  fn select(self, a: long4, b: long4) -> long4 {
    return (self >> 63).bitselect(a, b);
  }

  #[inline(always)]
  fn bitselect(self, a: long4, b: long4) -> long4 {
    return (a & !self) | (b & self);
  }
}

impl Select<ulong4> for long4 {
  #[inline(always)]
  fn select(self, a: ulong4, b: ulong4) -> ulong4 {
    return (self >> 63).bitselect(a, b);
  }

  #[inline(always)]
  fn bitselect(self, a: ulong4, b: ulong4) -> ulong4 {
    return ulong4::bitcast(self.bitselect(long4::bitcast(a), long4::bitcast(b)));
  }
}

impl Select<double4> for long4 {
  #[inline(always)]
  fn select(self, a: double4, b: double4) -> double4 {
    return (self >> 63).bitselect(a, b);
  }

  #[inline(always)]
  fn bitselect(self, a: double4, b: double4) -> double4 {
    return double4::bitcast(self.bitselect(long4::bitcast(a), long4::bitcast(b)));
  }
}

impl long4 {
  #[inline]
  pub fn bitcast<T>(x: T) -> long4 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn lo(self) -> long2 {
    return long2(self.0, self.1);
  }

  #[inline]
  pub fn hi(self) -> long2 {
    return long2(self.2, self.3);
  }

  #[inline]
  pub fn odd(self) -> long2 {
    return long2(self.1, self.3);
  }

  #[inline]
  pub fn even(self) -> long2 {
    return long2(self.0, self.2);
  }
}
