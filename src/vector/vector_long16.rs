use std;
use ::*;

impl Vector for long16 {
  type Scalar = i64;
  type Boolean = long16;

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

  #[inline(always)]
  fn map_unary(self, f: &Fn(Self::Scalar) -> Self::Scalar) -> Self {
    return long16(f(self.0), f(self.1), f(self.2), f(self.3), f(self.4), f(self.5), f(self.6), f(self.7), f(self.8), f(self.9), f(self.10), f(self.11), f(self.12), f(self.13), f(self.14), f(self.15));
  }

  #[inline(always)]
  fn map_binary(self, other: Self, f: &Fn(Self::Scalar, Self::Scalar) -> Self::Scalar) -> Self {
    return long16(f(self.0, other.0), f(self.1, other.1), f(self.2, other.2), f(self.3, other.3), f(self.4, other.4), f(self.5, other.5), f(self.6, other.6), f(self.7, other.7), f(self.8, other.8), f(self.9, other.9), f(self.10, other.10), f(self.11, other.11), f(self.12, other.12), f(self.13, other.13), f(self.14, other.14), f(self.15, other.15));
  }

  #[inline(always)]
  fn reduce(self, f: &Fn(Self::Scalar, Self::Scalar) -> Self::Scalar) -> Self::Scalar {
    return f(self.15, f(self.14, f(self.13, f(self.12, f(self.11, f(self.10, f(self.9, f(self.8, f(self.7, f(self.6, f(self.5, f(self.4, f(self.3, f(self.2, f(self.1, self.0)))))))))))))));
  }

  #[inline(always)]
  fn abs(self) -> Self {
    let mask = self >> 63;

    return (self ^ mask) - mask;
  }

  #[inline(always)]
  fn to_char_sat(self) -> char16 {
    return long16::to_char(self.clamp(Self::broadcast(std::i8::MIN as i64), Self::broadcast(std::i8::MAX as i64)));
  }

  #[inline(always)]
  fn to_uchar_sat(self) -> uchar16 {
    return long16::to_uchar(self.clamp(Self::broadcast(std::u8::MIN as i64), Self::broadcast(std::u8::MAX as i64)));
  }

  #[inline(always)]
  fn to_short_sat(self) -> short16 {
    return long16::to_short(self.clamp(Self::broadcast(std::i16::MIN as i64), Self::broadcast(std::i16::MAX as i64)));
  }

  #[inline(always)]
  fn to_ushort_sat(self) -> ushort16 {
    return long16::to_ushort(self.clamp(Self::broadcast(std::u16::MIN as i64), Self::broadcast(std::u16::MAX as i64)));
  }

  #[inline(always)]
  fn to_int_sat(self) -> int16 {
    return long16::to_int(self.clamp(Self::broadcast(std::i32::MIN as i64), Self::broadcast(std::i32::MAX as i64)));
  }

  #[inline(always)]
  fn to_uint_sat(self) -> uint16 {
    return long16::to_uint(self.clamp(Self::broadcast(std::u32::MIN as i64), Self::broadcast(std::u32::MAX as i64)));
  }

  #[inline(always)]
  fn to_long_sat(self) -> long16 {
    return self;
  }

  #[inline(always)]
  fn to_ulong_sat(self) -> ulong16 {
    return long16::to_ulong(self.max(Self::from(0)));
  }
}

impl Dot<long16> for long16 {
  type DotProduct = i64;
  #[inline(always)]
  fn dot(self, other: Self) -> Self::DotProduct {
    return reduce_add(self * other);
  }
}

impl Integer for long16 {
  type IntegerScalar = i64;

  const SIGN_MASK: i64 = std::i64::MIN;
}

impl Select<long16> for long16 {
  const MASK_SHIFT: i64 = 63;

  #[inline(always)]
  fn bitselect(self, a: long16, b: long16) -> long16 {
    return (a & !self) | (b & self);
  }
}

impl Select<ulong16> for long16 {
  const MASK_SHIFT: i64 = 63;

  #[inline(always)]
  fn bitselect(self, a: ulong16, b: ulong16) -> ulong16 {
    return ulong16::bitcast(self.bitselect(long16::bitcast(a), long16::bitcast(b)));
  }
}

impl Select<double16> for long16 {
  const MASK_SHIFT: i64 = 63;

  #[inline(always)]
  fn bitselect(self, a: double16, b: double16) -> double16 {
    return double16::bitcast(self.bitselect(long16::bitcast(a), long16::bitcast(b)));
  }
}

impl long16 {
  #[inline(always)]
  pub fn lo(self) -> long8 {
    return long8(self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7);
  }

  #[inline(always)]
  pub fn hi(self) -> long8 {
    return long8(self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15);
  }

  #[inline(always)]
  pub fn odd(self) -> long8 {
    return long8(self.1, self.3, self.5, self.7, self.9, self.11, self.13, self.15);
  }

  #[inline(always)]
  pub fn even(self) -> long8 {
    return long8(self.0, self.2, self.4, self.6, self.8, self.10, self.12, self.14);
  }
}
