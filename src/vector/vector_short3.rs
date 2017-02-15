use std;
use ::*;

impl Vector for short3 {
  type Scalar = i16;
  type Boolean = short3;

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
    return short3(f(self.0), f(self.1), f(self.2));
  }

  #[inline(always)]
  fn map_binary(self, other: Self, f: &Fn(Self::Scalar, Self::Scalar) -> Self::Scalar) -> Self {
    return short3(f(self.0, other.0), f(self.1, other.1), f(self.2, other.2));
  }

  #[inline(always)]
  fn reduce(self, f: &Fn(Self::Scalar, Self::Scalar) -> Self::Scalar) -> Self::Scalar {
    return f(self.2, f(self.1, self.0));
  }

  #[inline(always)]
  fn abs(self) -> Self {
    let mask = self >> 15;

    return (self ^ mask) - mask;
  }

  #[inline(always)]
  fn to_char_sat(self) -> char3 {
    return short3::to_char(self.clamp(Self::broadcast(std::i8::MIN as i16), Self::broadcast(std::i8::MAX as i16)));
  }

  #[inline(always)]
  fn to_uchar_sat(self) -> uchar3 {
    return short3::to_uchar(self.clamp(Self::broadcast(std::u8::MIN as i16), Self::broadcast(std::u8::MAX as i16)));
  }

  #[inline(always)]
  fn to_short_sat(self) -> short3 {
    return self;
  }

  #[inline(always)]
  fn to_ushort_sat(self) -> ushort3 {
    return short3::to_ushort(self.max(Self::from(0)));
  }

  #[inline(always)]
  fn to_int(self) -> int3 {
    return int3(self.0 as i32, self.1 as i32, self.2 as i32);
  }

  #[inline(always)]
  fn to_int_sat(self) -> int3 {
    return short3::to_int(self);
  }

  #[inline(always)]
  fn to_uint(self) -> uint3 {
    return uint3(self.0 as u32, self.1 as u32, self.2 as u32);
  }

  #[inline(always)]
  fn to_uint_sat(self) -> uint3 {
    return short3::to_uint(self.max(Self::from(0)));
  }

  #[inline(always)]
  fn to_float(self) -> float3 {
    return float3(self.0 as f32, self.1 as f32, self.2 as f32);
  }

  #[inline(always)]
  fn to_long(self) -> long3 {
    return long3(self.0 as i64, self.1 as i64, self.2 as i64);
  }

  #[inline(always)]
  fn to_long_sat(self) -> long3 {
    return short3::to_long(self);
  }

  #[inline(always)]
  fn to_ulong(self) -> ulong3 {
    return ulong3(self.0 as u64, self.1 as u64, self.2 as u64);
  }

  #[inline(always)]
  fn to_ulong_sat(self) -> ulong3 {
    return short3::to_ulong(self.max(Self::from(0)));
  }

  #[inline(always)]
  fn to_double(self) -> double3 {
    return double3(self.0 as f64, self.1 as f64, self.2 as f64);
  }
}

impl Dot<short3> for short3 {
  type DotProduct = i16;
  #[inline(always)]
  fn dot(self, other: Self) -> Self::DotProduct {
    return reduce_add(self * other);
  }
}

impl Integer for short3 {
  type IntegerScalar = i16;

  const SIGN_MASK: i16 = std::i16::MIN;
}

impl Select<short3> for short3 {
  const MASK_SHIFT: i16 = 15;

  #[inline(always)]
  fn bitselect(self, a: short3, b: short3) -> short3 {
    return (a & !self) | (b & self);
  }
}

impl Select<ushort3> for short3 {
  const MASK_SHIFT: i16 = 15;

  #[inline(always)]
  fn bitselect(self, a: ushort3, b: ushort3) -> ushort3 {
    return ushort3::bitcast(self.bitselect(short3::bitcast(a), short3::bitcast(b)));
  }
}

impl short3 {
  #[inline(always)]
  pub fn lo(self) -> short2 {
    return short2(self.0, self.1);
  }

  #[inline(always)]
  pub fn hi(self) -> short2 {
    return short2(self.2, 0);
  }

  #[inline(always)]
  pub fn odd(self) -> short2 {
    return short2(self.1, 0);
  }

  #[inline(always)]
  pub fn even(self) -> short2 {
    return short2(self.0, self.2);
  }
}
