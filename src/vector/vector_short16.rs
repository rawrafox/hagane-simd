use std;
use ::*;

impl Vector for short16 {
  type Scalar = i16;
  type Boolean = short16;

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
    return short16(f(self.0), f(self.1), f(self.2), f(self.3), f(self.4), f(self.5), f(self.6), f(self.7), f(self.8), f(self.9), f(self.10), f(self.11), f(self.12), f(self.13), f(self.14), f(self.15));
  }

  #[inline(always)]
  fn map_binary(self, other: Self, f: &Fn(Self::Scalar, Self::Scalar) -> Self::Scalar) -> Self {
    return short16(f(self.0, other.0), f(self.1, other.1), f(self.2, other.2), f(self.3, other.3), f(self.4, other.4), f(self.5, other.5), f(self.6, other.6), f(self.7, other.7), f(self.8, other.8), f(self.9, other.9), f(self.10, other.10), f(self.11, other.11), f(self.12, other.12), f(self.13, other.13), f(self.14, other.14), f(self.15, other.15));
  }

  #[inline(always)]
  fn reduce(self, f: &Fn(Self::Scalar, Self::Scalar) -> Self::Scalar) -> Self::Scalar {
    return f(self.15, f(self.14, f(self.13, f(self.12, f(self.11, f(self.10, f(self.9, f(self.8, f(self.7, f(self.6, f(self.5, f(self.4, f(self.3, f(self.2, f(self.1, self.0)))))))))))))));
  }

  #[inline(always)]
  fn abs(self) -> Self {
    let mask = self >> 15;

    return (self ^ mask) - mask;
  }

  #[inline(always)]
  fn to_char_sat(self) -> char16 {
    return short16::to_char(self.clamp(Self::broadcast(std::i8::MIN as i16), Self::broadcast(std::i8::MAX as i16)));
  }

  #[inline(always)]
  fn to_uchar_sat(self) -> uchar16 {
    return short16::to_uchar(self.clamp(Self::broadcast(std::u8::MIN as i16), Self::broadcast(std::u8::MAX as i16)));
  }

  #[inline(always)]
  fn to_short_sat(self) -> short16 {
    return self;
  }

  #[inline(always)]
  fn to_ushort_sat(self) -> ushort16 {
    return short16::to_ushort(self.max(Self::from(0)));
  }

  #[inline(always)]
  fn to_int_sat(self) -> int16 {
    return short16::to_int(self);
  }

  #[inline(always)]
  fn to_uint_sat(self) -> uint16 {
    return short16::to_uint(self.max(Self::from(0)));
  }

  #[inline(always)]
  fn to_long_sat(self) -> long16 {
    return short16::to_long(self);
  }

  #[inline(always)]
  fn to_ulong_sat(self) -> ulong16 {
    return short16::to_ulong(self.max(Self::from(0)));
  }
}

impl Dot<short16> for short16 {
  type DotProduct = i16;
  #[inline(always)]
  fn dot(self, other: Self) -> Self::DotProduct {
    return reduce_add(self * other);
  }
}

impl Integer for short16 {
  type IntegerScalar = i16;

  const SIGN_MASK: i16 = std::i16::MIN;
}

impl Select<short16> for short16 {
  const MASK_SHIFT: i16 = 15;

  #[inline(always)]
  fn bitselect(self, a: short16, b: short16) -> short16 {
    return (a & !self) | (b & self);
  }
}

impl Select<ushort16> for short16 {
  const MASK_SHIFT: i16 = 15;

  #[inline(always)]
  fn bitselect(self, a: ushort16, b: ushort16) -> ushort16 {
    return ushort16::bitcast(self.bitselect(short16::bitcast(a), short16::bitcast(b)));
  }
}

impl short16 {
  #[inline(always)]
  pub fn lo(self) -> short8 {
    return short8(self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7);
  }

  #[inline(always)]
  pub fn hi(self) -> short8 {
    return short8(self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15);
  }

  #[inline(always)]
  pub fn odd(self) -> short8 {
    return short8(self.1, self.3, self.5, self.7, self.9, self.11, self.13, self.15);
  }

  #[inline(always)]
  pub fn even(self) -> short8 {
    return short8(self.0, self.2, self.4, self.6, self.8, self.10, self.12, self.14);
  }
}
