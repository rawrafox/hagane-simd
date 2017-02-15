use std;
use ::*;

impl Vector for long8 {
  type Scalar = i64;
  type Boolean = long8;

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
  fn map_unary(self, f: &Fn(Self::Scalar) -> Self::Scalar) -> Self {
    return long8(f(self.0), f(self.1), f(self.2), f(self.3), f(self.4), f(self.5), f(self.6), f(self.7));
  }

  #[inline(always)]
  fn map_binary(self, other: Self, f: &Fn(Self::Scalar, Self::Scalar) -> Self::Scalar) -> Self {
    return long8(f(self.0, other.0), f(self.1, other.1), f(self.2, other.2), f(self.3, other.3), f(self.4, other.4), f(self.5, other.5), f(self.6, other.6), f(self.7, other.7));
  }

  #[inline(always)]
  fn reduce(self, f: &Fn(Self::Scalar, Self::Scalar) -> Self::Scalar) -> Self::Scalar {
    return f(self.7, f(self.6, f(self.5, f(self.4, f(self.3, f(self.2, f(self.1, self.0)))))));
  }

  #[inline(always)]
  fn abs(self) -> Self {
    let mask = self >> 63;

    return (self ^ mask) - mask;
  }

  #[inline(always)]
  fn to_char_sat(self) -> char8 {
    return long8::to_char(self.clamp(Self::broadcast(std::i8::MIN as i64), Self::broadcast(std::i8::MAX as i64)));
  }

  #[inline(always)]
  fn to_uchar_sat(self) -> uchar8 {
    return long8::to_uchar(self.clamp(Self::broadcast(std::u8::MIN as i64), Self::broadcast(std::u8::MAX as i64)));
  }

  #[inline(always)]
  fn to_short_sat(self) -> short8 {
    return long8::to_short(self.clamp(Self::broadcast(std::i16::MIN as i64), Self::broadcast(std::i16::MAX as i64)));
  }

  #[inline(always)]
  fn to_ushort_sat(self) -> ushort8 {
    return long8::to_ushort(self.clamp(Self::broadcast(std::u16::MIN as i64), Self::broadcast(std::u16::MAX as i64)));
  }

  #[inline(always)]
  fn to_int_sat(self) -> int8 {
    return long8::to_int(self.clamp(Self::broadcast(std::i32::MIN as i64), Self::broadcast(std::i32::MAX as i64)));
  }

  #[inline(always)]
  fn to_uint_sat(self) -> uint8 {
    return long8::to_uint(self.clamp(Self::broadcast(std::u32::MIN as i64), Self::broadcast(std::u32::MAX as i64)));
  }

  #[inline(always)]
  fn to_long_sat(self) -> long8 {
    return self;
  }

  #[inline(always)]
  fn to_ulong_sat(self) -> ulong8 {
    return long8::to_ulong(self.max(Self::from(0)));
  }
}

impl Dot<long8> for long8 {
  type DotProduct = i64;
  #[inline(always)]
  fn dot(self, other: Self) -> Self::DotProduct {
    return reduce_add(self * other);
  }
}

impl Integer for long8 {
  type IntegerScalar = i64;

  const SIGN_MASK: i64 = std::i64::MIN;
}

impl Select<long8> for long8 {
  const MASK_SHIFT: i64 = 63;

  #[inline(always)]
  fn bitselect(self, a: long8, b: long8) -> long8 {
    return (a & !self) | (b & self);
  }
}

impl Select<ulong8> for long8 {
  const MASK_SHIFT: i64 = 63;

  #[inline(always)]
  fn bitselect(self, a: ulong8, b: ulong8) -> ulong8 {
    return ulong8::bitcast(self.bitselect(long8::bitcast(a), long8::bitcast(b)));
  }
}

impl Select<double8> for long8 {
  const MASK_SHIFT: i64 = 63;

  #[inline(always)]
  fn bitselect(self, a: double8, b: double8) -> double8 {
    return double8::bitcast(self.bitselect(long8::bitcast(a), long8::bitcast(b)));
  }
}

impl long8 {
  #[inline(always)]
  pub fn lo(self) -> long4 {
    return long4(self.0, self.1, self.2, self.3);
  }

  #[inline(always)]
  pub fn hi(self) -> long4 {
    return long4(self.4, self.5, self.6, self.7);
  }

  #[inline(always)]
  pub fn odd(self) -> long4 {
    return long4(self.1, self.3, self.5, self.7);
  }

  #[inline(always)]
  pub fn even(self) -> long4 {
    return long4(self.0, self.2, self.4, self.6);
  }
}
