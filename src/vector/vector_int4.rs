use std;
use ::*;

impl Vector for int4 {
  type Scalar = i32;
  type Boolean = int4;

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
  fn to_char_sat(self) -> char4 {
    return int4::to_char(self.clamp(Self::broadcast(std::i8::MIN as i32), Self::broadcast(std::i8::MAX as i32)));
  }

  #[inline(always)]
  fn to_uchar_sat(self) -> uchar4 {
    return int4::to_uchar(self.clamp(Self::broadcast(std::u8::MIN as i32), Self::broadcast(std::u8::MAX as i32)));
  }

  #[inline(always)]
  fn to_short_sat(self) -> short4 {
    return int4::to_short(self.clamp(Self::broadcast(std::i16::MIN as i32), Self::broadcast(std::i16::MAX as i32)));
  }

  #[inline(always)]
  fn to_ushort_sat(self) -> ushort4 {
    return int4::to_ushort(self.clamp(Self::broadcast(std::u16::MIN as i32), Self::broadcast(std::u16::MAX as i32)));
  }

  #[inline(always)]
  fn to_int_sat(self) -> int4 {
    return self;
  }

  #[inline(always)]
  fn to_uint_sat(self) -> uint4 {
    return int4::to_uint(self.max(Self::from(0)));
  }

  #[inline(always)]
  fn to_long_sat(self) -> long4 {
    return int4::to_long(self);
  }

  #[inline(always)]
  fn to_ulong_sat(self) -> ulong4 {
    return int4::to_ulong(self.max(Self::from(0)));
  }
}

impl Dot<int4> for int4 {
  type DotProduct = i32;
  #[inline(always)]
  fn dot(self, other: Self) -> Self::DotProduct {
    return reduce_add(self * other);
  }
}

impl Integer for int4 {
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

impl Select<int4> for int4 {
  #[inline(always)]
  fn select(self, a: int4, b: int4) -> int4 {
    return (self >> 31).bitselect(a, b);
  }

  #[inline(always)]
  fn bitselect(self, a: int4, b: int4) -> int4 {
    return (a & !self) | (b & self);
  }
}

impl Select<uint4> for int4 {
  #[inline(always)]
  fn select(self, a: uint4, b: uint4) -> uint4 {
    return (self >> 31).bitselect(a, b);
  }

  #[inline(always)]
  fn bitselect(self, a: uint4, b: uint4) -> uint4 {
    return uint4::bitcast(self.bitselect(int4::bitcast(a), int4::bitcast(b)));
  }
}

impl Select<float4> for int4 {
  #[inline(always)]
  fn select(self, a: float4, b: float4) -> float4 {
    return (self >> 31).bitselect(a, b);
  }

  #[inline(always)]
  fn bitselect(self, a: float4, b: float4) -> float4 {
    return float4::bitcast(self.bitselect(int4::bitcast(a), int4::bitcast(b)));
  }
}

impl int4 {
  #[inline]
  pub fn bitcast<T>(x: T) -> int4 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn lo(self) -> int2 {
    return int2(self.0, self.1);
  }

  #[inline]
  pub fn hi(self) -> int2 {
    return int2(self.2, self.3);
  }

  #[inline]
  pub fn odd(self) -> int2 {
    return int2(self.1, self.3);
  }

  #[inline]
  pub fn even(self) -> int2 {
    return int2(self.0, self.2);
  }
}
