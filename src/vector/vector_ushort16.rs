use std;
use ::*;

impl Vector for ushort16 {
  type Scalar = u16;
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
    return ushort16(f(self.0), f(self.1), f(self.2), f(self.3), f(self.4), f(self.5), f(self.6), f(self.7), f(self.8), f(self.9), f(self.10), f(self.11), f(self.12), f(self.13), f(self.14), f(self.15));
  }

  #[inline(always)]
  fn map_binary(self, other: Self, f: &Fn(Self::Scalar, Self::Scalar) -> Self::Scalar) -> Self {
    return ushort16(f(self.0, other.0), f(self.1, other.1), f(self.2, other.2), f(self.3, other.3), f(self.4, other.4), f(self.5, other.5), f(self.6, other.6), f(self.7, other.7), f(self.8, other.8), f(self.9, other.9), f(self.10, other.10), f(self.11, other.11), f(self.12, other.12), f(self.13, other.13), f(self.14, other.14), f(self.15, other.15));
  }

  #[inline(always)]
  fn reduce(self, f: &Fn(Self::Scalar, Self::Scalar) -> Self::Scalar) -> Self::Scalar {
    return f(self.15, f(self.14, f(self.13, f(self.12, f(self.11, f(self.10, f(self.9, f(self.8, f(self.7, f(self.6, f(self.5, f(self.4, f(self.3, f(self.2, f(self.1, self.0)))))))))))))));
  }

  #[inline(always)]
  fn abs(self) -> Self {
    return self;
  }

  #[inline(always)]
  fn to_char_sat(self) -> char16 {
    return ushort16::to_char(self.min(Self::broadcast(std::i8::MAX as u16)));
  }

  #[inline(always)]
  fn to_uchar_sat(self) -> uchar16 {
    return ushort16::to_uchar(self.min(Self::broadcast(std::u8::MAX as u16)));
  }

  #[inline(always)]
  fn to_short_sat(self) -> short16 {
    return ushort16::to_short(self.min(Self::broadcast(std::i16::MAX as u16)));
  }

  #[inline(always)]
  fn to_ushort_sat(self) -> ushort16 {
    return self;
  }

  #[inline(always)]
  fn to_int_sat(self) -> int16 {
    return ushort16::to_int(self.min(Self::broadcast(std::i32::MAX as u16)));
  }

  #[inline(always)]
  fn to_uint_sat(self) -> uint16 {
    return ushort16::to_uint(self);
  }

  #[inline(always)]
  fn to_long_sat(self) -> long16 {
    return ushort16::to_long(self.min(Self::broadcast(std::i64::MAX as u16)));
  }

  #[inline(always)]
  fn to_ulong_sat(self) -> ulong16 {
    return ushort16::to_ulong(self);
  }
}

impl Dot<ushort16> for ushort16 {
  type DotProduct = u16;
  #[inline(always)]
  fn dot(self, other: Self) -> Self::DotProduct {
    return reduce_add(self * other);
  }
}

impl Integer for ushort16 {
  type IntegerScalar = u16;

  const SIGN_MASK: u16 = 0x8000;
}

impl ushort16 {
  #[inline(always)]
  pub fn bitcast<T>(x: T) -> ushort16 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline(always)]
  pub fn lo(self) -> ushort8 {
    return ushort8(self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7);
  }

  #[inline(always)]
  pub fn hi(self) -> ushort8 {
    return ushort8(self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15);
  }

  #[inline(always)]
  pub fn odd(self) -> ushort8 {
    return ushort8(self.1, self.3, self.5, self.7, self.9, self.11, self.13, self.15);
  }

  #[inline(always)]
  pub fn even(self) -> ushort8 {
    return ushort8(self.0, self.2, self.4, self.6, self.8, self.10, self.12, self.14);
  }
}
