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
  fn map_unary(self, f: &Fn(Self::Scalar) -> Self::Scalar) -> Self {
    return short8(f(self.0), f(self.1), f(self.2), f(self.3), f(self.4), f(self.5), f(self.6), f(self.7));
  }

  #[inline(always)]
  fn map_binary(self, other: Self, f: &Fn(Self::Scalar, Self::Scalar) -> Self::Scalar) -> Self {
    return short8(f(self.0, other.0), f(self.1, other.1), f(self.2, other.2), f(self.3, other.3), f(self.4, other.4), f(self.5, other.5), f(self.6, other.6), f(self.7, other.7));
  }

  #[inline(always)]
  fn reduce(self, f: &Fn(Self::Scalar, Self::Scalar) -> Self::Scalar) -> Self::Scalar {
    return f(self.7, f(self.6, f(self.5, f(self.4, f(self.3, f(self.2, f(self.1, self.0)))))));
  }

  #[inline(always)]
  fn abs(self) -> Self {
    let mask = self >> 15;

    return (self ^ mask) - mask;
  }

  #[inline(always)]
  fn to_char_sat(self) -> char8 {
    return short8::to_char(self.clamp(Self::broadcast(std::i8::MIN as i16), Self::broadcast(std::i8::MAX as i16)));
  }

  #[inline(always)]
  fn to_uchar_sat(self) -> uchar8 {
    return short8::to_uchar(self.clamp(Self::broadcast(std::u8::MIN as i16), Self::broadcast(std::u8::MAX as i16)));
  }

  #[inline(always)]
  fn to_short_sat(self) -> short8 {
    return self;
  }

  #[inline(always)]
  fn to_ushort_sat(self) -> ushort8 {
    return short8::to_ushort(self.max(Self::from(0)));
  }

  #[inline(always)]
  fn to_int_sat(self) -> int8 {
    return short8::to_int(self);
  }

  #[inline(always)]
  fn to_uint_sat(self) -> uint8 {
    return short8::to_uint(self.max(Self::from(0)));
  }

  #[inline(always)]
  fn to_long_sat(self) -> long8 {
    return short8::to_long(self);
  }

  #[inline(always)]
  fn to_ulong_sat(self) -> ulong8 {
    return short8::to_ulong(self.max(Self::from(0)));
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
  type IntegerScalar = i16;

  const SIGN_MASK: i16 = std::i16::MIN;
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
  #[inline(always)]
  pub fn bitcast<T>(x: T) -> short8 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline(always)]
  pub fn lo(self) -> short4 {
    return short4(self.0, self.1, self.2, self.3);
  }

  #[inline(always)]
  pub fn hi(self) -> short4 {
    return short4(self.4, self.5, self.6, self.7);
  }

  #[inline(always)]
  pub fn odd(self) -> short4 {
    return short4(self.1, self.3, self.5, self.7);
  }

  #[inline(always)]
  pub fn even(self) -> short4 {
    return short4(self.0, self.2, self.4, self.6);
  }
}
