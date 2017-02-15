use std;
use ::*;

impl Vector for ushort4 {
  type Scalar = u16;
  type Boolean = short4;

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
  fn map_unary(self, f: &Fn(Self::Scalar) -> Self::Scalar) -> Self {
    return ushort4(f(self.0), f(self.1), f(self.2), f(self.3));
  }

  #[inline(always)]
  fn map_binary(self, other: Self, f: &Fn(Self::Scalar, Self::Scalar) -> Self::Scalar) -> Self {
    return ushort4(f(self.0, other.0), f(self.1, other.1), f(self.2, other.2), f(self.3, other.3));
  }

  #[inline(always)]
  fn reduce(self, f: &Fn(Self::Scalar, Self::Scalar) -> Self::Scalar) -> Self::Scalar {
    return f(self.3, f(self.2, f(self.1, self.0)));
  }

  #[inline(always)]
  fn abs(self) -> Self {
    return self;
  }

  #[inline(always)]
  fn to_char_sat(self) -> char4 {
    return ushort4::to_char(self.min(Self::broadcast(std::i8::MAX as u16)));
  }

  #[inline(always)]
  fn to_uchar_sat(self) -> uchar4 {
    return ushort4::to_uchar(self.min(Self::broadcast(std::u8::MAX as u16)));
  }

  #[inline(always)]
  fn to_short_sat(self) -> short4 {
    return ushort4::to_short(self.min(Self::broadcast(std::i16::MAX as u16)));
  }

  #[inline(always)]
  fn to_ushort_sat(self) -> ushort4 {
    return self;
  }

  #[inline(always)]
  fn to_int_sat(self) -> int4 {
    return ushort4::to_int(self.min(Self::broadcast(std::i32::MAX as u16)));
  }

  #[inline(always)]
  fn to_uint_sat(self) -> uint4 {
    return ushort4::to_uint(self);
  }

  #[inline(always)]
  fn to_long_sat(self) -> long4 {
    return ushort4::to_long(self.min(Self::broadcast(std::i64::MAX as u16)));
  }

  #[inline(always)]
  fn to_ulong_sat(self) -> ulong4 {
    return ushort4::to_ulong(self);
  }
}

impl Dot<ushort4> for ushort4 {
  type DotProduct = u16;
  #[inline(always)]
  fn dot(self, other: Self) -> Self::DotProduct {
    return reduce_add(self * other);
  }
}

impl Integer for ushort4 {
  type IntegerScalar = u16;

  const SIGN_MASK: u16 = 0x8000;
}

impl ushort4 {
  #[inline(always)]
  pub fn lo(self) -> ushort2 {
    return ushort2(self.0, self.1);
  }

  #[inline(always)]
  pub fn hi(self) -> ushort2 {
    return ushort2(self.2, self.3);
  }

  #[inline(always)]
  pub fn odd(self) -> ushort2 {
    return ushort2(self.1, self.3);
  }

  #[inline(always)]
  pub fn even(self) -> ushort2 {
    return ushort2(self.0, self.2);
  }
}
