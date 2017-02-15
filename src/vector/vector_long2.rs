use std;
use ::*;

impl Vector for long2 {
  type Scalar = i64;
  type Boolean = long2;

  type CharVector = char2;
  type ShortVector = short2;
  type IntVector = int2;
  type LongVector = long2;

  type UCharVector = uchar2;
  type UShortVector = ushort2;
  type UIntVector = uint2;
  type ULongVector = ulong2;

  type FloatVector = float2;
  type DoubleVector = double2;

  #[inline(always)]
  fn map_unary(self, f: &Fn(Self::Scalar) -> Self::Scalar) -> Self {
    return long2(f(self.0), f(self.1));
  }

  #[inline(always)]
  fn map_binary(self, other: Self, f: &Fn(Self::Scalar, Self::Scalar) -> Self::Scalar) -> Self {
    return long2(f(self.0, other.0), f(self.1, other.1));
  }

  #[inline(always)]
  fn reduce(self, f: &Fn(Self::Scalar, Self::Scalar) -> Self::Scalar) -> Self::Scalar {
    return f(self.1, self.0);
  }

  #[inline(always)]
  fn abs(self) -> Self {
    let mask = self >> 63;

    return (self ^ mask) - mask;
  }

  #[inline(always)]
  fn to_char_sat(self) -> char2 {
    return long2::to_char(self.clamp(Self::broadcast(std::i8::MIN as i64), Self::broadcast(std::i8::MAX as i64)));
  }

  #[inline(always)]
  fn to_uchar_sat(self) -> uchar2 {
    return long2::to_uchar(self.clamp(Self::broadcast(std::u8::MIN as i64), Self::broadcast(std::u8::MAX as i64)));
  }

  #[inline(always)]
  fn to_short_sat(self) -> short2 {
    return long2::to_short(self.clamp(Self::broadcast(std::i16::MIN as i64), Self::broadcast(std::i16::MAX as i64)));
  }

  #[inline(always)]
  fn to_ushort_sat(self) -> ushort2 {
    return long2::to_ushort(self.clamp(Self::broadcast(std::u16::MIN as i64), Self::broadcast(std::u16::MAX as i64)));
  }

  #[inline(always)]
  fn to_int_sat(self) -> int2 {
    return long2::to_int(self.clamp(Self::broadcast(std::i32::MIN as i64), Self::broadcast(std::i32::MAX as i64)));
  }

  #[inline(always)]
  fn to_uint_sat(self) -> uint2 {
    return long2::to_uint(self.clamp(Self::broadcast(std::u32::MIN as i64), Self::broadcast(std::u32::MAX as i64)));
  }

  #[inline(always)]
  fn to_long_sat(self) -> long2 {
    return self;
  }

  #[inline(always)]
  fn to_ulong_sat(self) -> ulong2 {
    return long2::to_ulong(self.max(Self::from(0)));
  }
}

impl Dot<long2> for long2 {
  type DotProduct = i64;
  #[inline(always)]
  fn dot(self, other: Self) -> Self::DotProduct {
    return reduce_add(self * other);
  }
}

impl Integer for long2 {
  type IntegerScalar = i64;

  const SIGN_MASK: i64 = std::i64::MIN;
}

impl Select<long2> for long2 {
  const MASK_SHIFT: i64 = 63;

  #[inline(always)]
  fn bitselect(self, a: long2, b: long2) -> long2 {
    return (a & !self) | (b & self);
  }
}

impl Select<ulong2> for long2 {
  const MASK_SHIFT: i64 = 63;

  #[inline(always)]
  fn bitselect(self, a: ulong2, b: ulong2) -> ulong2 {
    return ulong2::bitcast(self.bitselect(long2::bitcast(a), long2::bitcast(b)));
  }
}

impl Select<double2> for long2 {
  const MASK_SHIFT: i64 = 63;

  #[inline(always)]
  fn bitselect(self, a: double2, b: double2) -> double2 {
    return double2::bitcast(self.bitselect(long2::bitcast(a), long2::bitcast(b)));
  }
}

impl long2 {
  #[inline(always)]
  pub fn lo(self) -> i64 {
    return self.0;
  }

  #[inline(always)]
  pub fn hi(self) -> i64 {
    return self.1;
  }

  #[inline(always)]
  pub fn odd(self) -> i64 {
    return self.1;
  }

  #[inline(always)]
  pub fn even(self) -> i64 {
    return self.0;
  }
}
