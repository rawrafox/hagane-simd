use std;
use ::*;

impl Vector for uchar3 {
  type Scalar = u8;
  type Boolean = char3;

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
    return uchar3(f(self.0), f(self.1), f(self.2));
  }

  #[inline(always)]
  fn map_binary(self, other: Self, f: &Fn(Self::Scalar, Self::Scalar) -> Self::Scalar) -> Self {
    return uchar3(f(self.0, other.0), f(self.1, other.1), f(self.2, other.2));
  }

  #[inline(always)]
  fn reduce(self, f: &Fn(Self::Scalar, Self::Scalar) -> Self::Scalar) -> Self::Scalar {
    return f(self.2, f(self.1, self.0));
  }

  #[inline(always)]
  fn abs(self) -> Self {
    return self;
  }

  #[inline(always)]
  fn to_char_sat(self) -> char3 {
    return uchar3::to_char(self.min(Self::broadcast(std::i8::MAX as u8)));
  }

  #[inline(always)]
  fn to_uchar_sat(self) -> uchar3 {
    return self;
  }

  #[inline(always)]
  fn to_short(self) -> short3 {
    return short3(self.0 as i16, self.1 as i16, self.2 as i16);
  }

  #[inline(always)]
  fn to_short_sat(self) -> short3 {
    return uchar3::to_short(self.min(Self::broadcast(std::i16::MAX as u8)));
  }

  #[inline(always)]
  fn to_ushort(self) -> ushort3 {
    return ushort3(self.0 as u16, self.1 as u16, self.2 as u16);
  }

  #[inline(always)]
  fn to_ushort_sat(self) -> ushort3 {
    return uchar3::to_ushort(self);
  }

  #[inline(always)]
  fn to_int(self) -> int3 {
    return int3(self.0 as i32, self.1 as i32, self.2 as i32);
  }

  #[inline(always)]
  fn to_int_sat(self) -> int3 {
    return uchar3::to_int(self.min(Self::broadcast(std::i32::MAX as u8)));
  }

  #[inline(always)]
  fn to_uint(self) -> uint3 {
    return uint3(self.0 as u32, self.1 as u32, self.2 as u32);
  }

  #[inline(always)]
  fn to_uint_sat(self) -> uint3 {
    return uchar3::to_uint(self);
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
    return uchar3::to_long(self.min(Self::broadcast(std::i64::MAX as u8)));
  }

  #[inline(always)]
  fn to_ulong(self) -> ulong3 {
    return ulong3(self.0 as u64, self.1 as u64, self.2 as u64);
  }

  #[inline(always)]
  fn to_ulong_sat(self) -> ulong3 {
    return uchar3::to_ulong(self);
  }

  #[inline(always)]
  fn to_double(self) -> double3 {
    return double3(self.0 as f64, self.1 as f64, self.2 as f64);
  }
}

impl Dot<uchar3> for uchar3 {
  type DotProduct = u8;
  #[inline(always)]
  fn dot(self, other: Self) -> Self::DotProduct {
    return reduce_add(self * other);
  }
}

impl Integer for uchar3 {
  type IntegerScalar = u8;

  const SIGN_MASK: u8 = 0x80;
}

impl uchar3 {
  #[inline(always)]
  pub fn lo(self) -> uchar2 {
    return uchar2(self.0, self.1);
  }

  #[inline(always)]
  pub fn hi(self) -> uchar2 {
    return uchar2(self.2, 0);
  }

  #[inline(always)]
  pub fn odd(self) -> uchar2 {
    return uchar2(self.1, 0);
  }

  #[inline(always)]
  pub fn even(self) -> uchar2 {
    return uchar2(self.0, self.2);
  }
}
