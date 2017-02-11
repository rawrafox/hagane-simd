use std;
use ::*;

impl Vector for int16 {
  type Scalar = i32;
  type Boolean = int16;

  type CharVector = char16;
  type ShortVector = short16;
  type IntVector = int16;
  type LongVector = long16;

  type UCharVector = uchar16;
  type UShortVector = ushort16;
  type UIntVector = uint16;
  type ULongVector = ulong16;

  type FloatVector = float16;
  type DoubleVector = double16;

  const ZERO: Self = int16(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0);
  const ONE: Self = int16(1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1);
  const TWO: Self = int16(2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2);
  const THREE: Self = int16(3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3);

  #[inline(always)]
  fn abs(self) -> Self {
    let mask = self >> 31;

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
  fn to_char_sat(self) -> char16 {
    return int16::to_char(clamp(self, broadcast(std::i8::MIN as i32), broadcast(std::i8::MAX as i32)));
  }

  #[inline(always)]
  fn to_uchar_sat(self) -> uchar16 {
    return int16::to_uchar(clamp(self, broadcast(std::u8::MIN as i32), broadcast(std::u8::MAX as i32)));
  }

  #[inline(always)]
  fn to_short_sat(self) -> short16 {
    return int16::to_short(clamp(self, broadcast(std::i16::MIN as i32), broadcast(std::i16::MAX as i32)));
  }

  #[inline(always)]
  fn to_ushort_sat(self) -> ushort16 {
    return int16::to_ushort(clamp(self, broadcast(std::u16::MIN as i32), broadcast(std::u16::MAX as i32)));
  }

  #[inline(always)]
  fn to_int_sat(self) -> int16 {
    return self;
  }

  #[inline(always)]
  fn to_uint_sat(self) -> uint16 {
    return int16::to_uint(max(self, Self::ZERO));
  }

  #[inline(always)]
  fn to_long_sat(self) -> long16 {
    return int16::to_long(self);
  }

  #[inline(always)]
  fn to_ulong_sat(self) -> ulong16 {
    return int16::to_ulong(max(self, Self::ZERO));
  }
}

impl Dot<int16> for int16 {
  type DotProduct = i32;
  #[inline(always)]
  fn dot(self, other: Self) -> Self::DotProduct {
    return reduce_add(self * other);
  }
}

impl Integer for int16 {
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

impl Select<int16> for int16 {
  #[inline(always)]
  fn select(self, a: int16, b: int16) -> int16 {
    return (self >> 31).bitselect(a, b);
  }

  #[inline(always)]
  fn bitselect(self, a: int16, b: int16) -> int16 {
    return (a & !self) | (b & self);
  }
}

impl Select<uint16> for int16 {
  #[inline(always)]
  fn select(self, a: uint16, b: uint16) -> uint16 {
    return (self >> 31).bitselect(a, b);
  }

  #[inline(always)]
  fn bitselect(self, a: uint16, b: uint16) -> uint16 {
    return uint16::bitcast(self.bitselect(int16::bitcast(a), int16::bitcast(b)));
  }
}

impl Select<float16> for int16 {
  #[inline(always)]
  fn select(self, a: float16, b: float16) -> float16 {
    return (self >> 31).bitselect(a, b);
  }

  #[inline(always)]
  fn bitselect(self, a: float16, b: float16) -> float16 {
    return float16::bitcast(self.bitselect(int16::bitcast(a), int16::bitcast(b)));
  }
}

impl int16 {
  #[inline]
  pub fn bitcast<T>(x: T) -> int16 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn lo(self) -> int8 {
    return int8(self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7);
  }

  #[inline]
  pub fn hi(self) -> int8 {
    return int8(self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15);
  }

  #[inline]
  pub fn odd(self) -> int8 {
    return int8(self.1, self.3, self.5, self.7, self.9, self.11, self.13, self.15);
  }

  #[inline]
  pub fn even(self) -> int8 {
    return int8(self.0, self.2, self.4, self.6, self.8, self.10, self.12, self.14);
  }
}
