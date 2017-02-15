use std;
use ::*;

impl Vector for int8 {
  type Scalar = i32;
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

  #[inline(always)]
  fn abs(self) -> Self {
    let mask = self >> 31;

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
  fn to_char_sat(self) -> char8 {
    return int8::to_char(self.clamp(Self::broadcast(std::i8::MIN as i32), Self::broadcast(std::i8::MAX as i32)));
  }

  #[inline(always)]
  fn to_uchar_sat(self) -> uchar8 {
    return int8::to_uchar(self.clamp(Self::broadcast(std::u8::MIN as i32), Self::broadcast(std::u8::MAX as i32)));
  }

  #[inline(always)]
  fn to_short_sat(self) -> short8 {
    return int8::to_short(self.clamp(Self::broadcast(std::i16::MIN as i32), Self::broadcast(std::i16::MAX as i32)));
  }

  #[inline(always)]
  fn to_ushort_sat(self) -> ushort8 {
    return int8::to_ushort(self.clamp(Self::broadcast(std::u16::MIN as i32), Self::broadcast(std::u16::MAX as i32)));
  }

  #[inline(always)]
  fn to_int_sat(self) -> int8 {
    return self;
  }

  #[inline(always)]
  fn to_uint_sat(self) -> uint8 {
    return int8::to_uint(self.max(Self::from(0)));
  }

  #[inline(always)]
  fn to_long_sat(self) -> long8 {
    return int8::to_long(self);
  }

  #[inline(always)]
  fn to_ulong_sat(self) -> ulong8 {
    return int8::to_ulong(self.max(Self::from(0)));
  }
}

impl Dot<int8> for int8 {
  type DotProduct = i32;
  #[inline(always)]
  fn dot(self, other: Self) -> Self::DotProduct {
    return reduce_add(self * other);
  }
}

impl Integer for int8 {
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
    return self.reduce_and() & std::i32::MIN != 0;
  }

  #[inline(always)]
  fn any(self) -> bool {
    return self.reduce_or() & std::i32::MIN != 0;
  }
}

impl Select<int8> for int8 {
  #[inline(always)]
  fn select(self, a: int8, b: int8) -> int8 {
    return (self >> 31).bitselect(a, b);
  }

  #[inline(always)]
  fn bitselect(self, a: int8, b: int8) -> int8 {
    return (a & !self) | (b & self);
  }
}

impl Select<uint8> for int8 {
  #[inline(always)]
  fn select(self, a: uint8, b: uint8) -> uint8 {
    return (self >> 31).bitselect(a, b);
  }

  #[inline(always)]
  fn bitselect(self, a: uint8, b: uint8) -> uint8 {
    return uint8::bitcast(self.bitselect(int8::bitcast(a), int8::bitcast(b)));
  }
}

impl Select<float8> for int8 {
  #[inline(always)]
  fn select(self, a: float8, b: float8) -> float8 {
    return (self >> 31).bitselect(a, b);
  }

  #[inline(always)]
  fn bitselect(self, a: float8, b: float8) -> float8 {
    return float8::bitcast(self.bitselect(int8::bitcast(a), int8::bitcast(b)));
  }
}

impl int8 {
  #[inline]
  pub fn bitcast<T>(x: T) -> int8 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn lo(self) -> int4 {
    return int4(self.0, self.1, self.2, self.3);
  }

  #[inline]
  pub fn hi(self) -> int4 {
    return int4(self.4, self.5, self.6, self.7);
  }

  #[inline]
  pub fn odd(self) -> int4 {
    return int4(self.1, self.3, self.5, self.7);
  }

  #[inline]
  pub fn even(self) -> int4 {
    return int4(self.0, self.2, self.4, self.6);
  }
}
