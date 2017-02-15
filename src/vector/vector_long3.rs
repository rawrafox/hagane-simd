use std;
use ::*;

impl Vector for long3 {
  type Scalar = i64;
  type Boolean = long3;

  type CharVector = char3;
  type ShortVector = short3;
  type IntVector = int3;
  type LongVector = long3;

  type UCharVector = uchar3;
  type UShortVector = ushort3;
  type UIntVector = uint3;
  type ULongVector = ulong3;

  type FloatVector = float3;
  type DoubleVector = double3;

  #[inline(always)]
  fn map_unary(self, f: &Fn(Self::Scalar) -> Self::Scalar) -> Self {
    return long3(f(self.0), f(self.1), f(self.2));
  }

  #[inline(always)]
  fn map_binary(self, other: Self, f: &Fn(Self::Scalar, Self::Scalar) -> Self::Scalar) -> Self {
    return long3(f(self.0, other.0), f(self.1, other.1), f(self.2, other.2));
  }

  #[inline(always)]
  fn reduce(self, f: &Fn(Self::Scalar, Self::Scalar) -> Self::Scalar) -> Self::Scalar {
    return f(self.2, f(self.1, self.0));
  }

  #[inline(always)]
  fn abs(self) -> Self {
    let mask = self >> 63;

    return (self ^ mask) - mask;
  }

  #[inline(always)]
  fn to_char_sat(self) -> char3 {
    return long3::to_char(self.clamp(Self::broadcast(std::i8::MIN as i64), Self::broadcast(std::i8::MAX as i64)));
  }

  #[inline(always)]
  fn to_uchar_sat(self) -> uchar3 {
    return long3::to_uchar(self.clamp(Self::broadcast(std::u8::MIN as i64), Self::broadcast(std::u8::MAX as i64)));
  }

  #[inline(always)]
  fn to_short_sat(self) -> short3 {
    return long3::to_short(self.clamp(Self::broadcast(std::i16::MIN as i64), Self::broadcast(std::i16::MAX as i64)));
  }

  #[inline(always)]
  fn to_ushort_sat(self) -> ushort3 {
    return long3::to_ushort(self.clamp(Self::broadcast(std::u16::MIN as i64), Self::broadcast(std::u16::MAX as i64)));
  }

  #[inline(always)]
  fn to_int_sat(self) -> int3 {
    return long3::to_int(self.clamp(Self::broadcast(std::i32::MIN as i64), Self::broadcast(std::i32::MAX as i64)));
  }

  #[inline(always)]
  fn to_uint_sat(self) -> uint3 {
    return long3::to_uint(self.clamp(Self::broadcast(std::u32::MIN as i64), Self::broadcast(std::u32::MAX as i64)));
  }

  #[inline(always)]
  fn to_long_sat(self) -> long3 {
    return self;
  }

  #[inline(always)]
  fn to_ulong_sat(self) -> ulong3 {
    return long3::to_ulong(self.max(Self::from(0)));
  }
}

impl Dot<long3> for long3 {
  type DotProduct = i64;
  #[inline(always)]
  fn dot(self, other: Self) -> Self::DotProduct {
    return reduce_add(self * other);
  }
}

impl Integer for long3 {
  type IntegerScalar = i64;

  const SIGN_MASK: i64 = std::i64::MIN;
}

impl Select<long3> for long3 {
  const MASK_SHIFT: i64 = 63;

  #[inline(always)]
  fn bitselect(self, a: long3, b: long3) -> long3 {
    return (a & !self) | (b & self);
  }
}

impl Select<ulong3> for long3 {
  const MASK_SHIFT: i64 = 63;

  #[inline(always)]
  fn bitselect(self, a: ulong3, b: ulong3) -> ulong3 {
    return ulong3::bitcast(self.bitselect(long3::bitcast(a), long3::bitcast(b)));
  }
}

impl Select<double3> for long3 {
  const MASK_SHIFT: i64 = 63;

  #[inline(always)]
  fn bitselect(self, a: double3, b: double3) -> double3 {
    return double3::bitcast(self.bitselect(long3::bitcast(a), long3::bitcast(b)));
  }
}

impl long3 {
  #[inline(always)]
  pub fn lo(self) -> long2 {
    return long2(self.0, self.1);
  }

  #[inline(always)]
  pub fn hi(self) -> long2 {
    return long2(self.2, 0);
  }

  #[inline(always)]
  pub fn odd(self) -> long2 {
    return long2(self.1, 0);
  }

  #[inline(always)]
  pub fn even(self) -> long2 {
    return long2(self.0, self.2);
  }
}
