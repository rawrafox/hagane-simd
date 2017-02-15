use std;
use ::*;

impl Vector for int3 {
  type Scalar = i32;
  type Boolean = int3;

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
  fn abs(self) -> Self {
    let mask = self >> 31;

    return (self ^ mask) - mask;
  }

  #[inline(always)]
  fn max(self, other: Self) -> Self {
    return gt(other, self).bitselect(self, other);
  }

  #[inline(always)]
  fn min(self, other: Self) -> Self {
    return lt(other, self).bitselect(self, other);
  }

  #[inline(always)]
  fn reduce_add(self) -> Self::Scalar {
    return self.0 + self.1 + self.2;
  }

  #[inline(always)]
  fn reduce_min(self) -> Self::Scalar {
    return std::cmp::min(reduce_min(self.lo()), self.2);
  }

  #[inline(always)]
  fn reduce_max(self) -> Self::Scalar {
    return std::cmp::max(reduce_max(self.lo()), self.2);
  }

  #[inline(always)]
  fn to_char_sat(self) -> char3 {
    return int3::to_char(self.clamp(Self::broadcast(std::i8::MIN as i32), Self::broadcast(std::i8::MAX as i32)));
  }

  #[inline(always)]
  fn to_uchar_sat(self) -> uchar3 {
    return int3::to_uchar(self.clamp(Self::broadcast(std::u8::MIN as i32), Self::broadcast(std::u8::MAX as i32)));
  }

  #[inline(always)]
  fn to_short_sat(self) -> short3 {
    return int3::to_short(self.clamp(Self::broadcast(std::i16::MIN as i32), Self::broadcast(std::i16::MAX as i32)));
  }

  #[inline(always)]
  fn to_ushort_sat(self) -> ushort3 {
    return int3::to_ushort(self.clamp(Self::broadcast(std::u16::MIN as i32), Self::broadcast(std::u16::MAX as i32)));
  }

  #[inline(always)]
  fn to_int_sat(self) -> int3 {
    return self;
  }

  #[inline(always)]
  fn to_uint_sat(self) -> uint3 {
    return int3::to_uint(self.max(Self::from(0)));
  }

  #[inline(always)]
  fn to_long(self) -> long3 {
    return long3(self.0 as i64, self.1 as i64, self.2 as i64);
  }

  #[inline(always)]
  fn to_long_sat(self) -> long3 {
    return int3::to_long(self);
  }

  #[inline(always)]
  fn to_ulong(self) -> ulong3 {
    return ulong3(self.0 as u64, self.1 as u64, self.2 as u64);
  }

  #[inline(always)]
  fn to_ulong_sat(self) -> ulong3 {
    return int3::to_ulong(self.max(Self::from(0)));
  }

  #[inline(always)]
  fn to_double(self) -> double3 {
    return double3(self.0 as f64, self.1 as f64, self.2 as f64);
  }
}

impl Dot<int3> for int3 {
  type DotProduct = i32;
  #[inline(always)]
  fn dot(self, other: Self) -> Self::DotProduct {
    return reduce_add(self * other);
  }
}

impl Integer for int3 {
  #[inline(always)]
  fn reduce_and(self) -> Self::Scalar {
    return self.0 & self.1 & self.2
  }

  #[inline(always)]
  fn reduce_or(self) -> Self::Scalar {
    return self.0 | self.1 | self.2
  }

  #[inline(always)]
  fn reduce_xor(self) -> Self::Scalar {
    return self.0 ^ self.1 ^ self.2
  }

  #[inline(always)]
  fn all(self) -> bool {
    return self.reduce_and() & std::i32::MIN != 0;
  }

  #[inline(always)]
  fn any(self) -> bool {
    return self.reduce_or() & std::i32::MIN != 0;
  }
}

impl Select<int3> for int3 {
  #[inline(always)]
  fn select(self, a: int3, b: int3) -> int3 {
    return (self >> 31).bitselect(a, b);
  }

  #[inline(always)]
  fn bitselect(self, a: int3, b: int3) -> int3 {
    return (a & !self) | (b & self);
  }
}

impl Select<uint3> for int3 {
  #[inline(always)]
  fn select(self, a: uint3, b: uint3) -> uint3 {
    return (self >> 31).bitselect(a, b);
  }

  #[inline(always)]
  fn bitselect(self, a: uint3, b: uint3) -> uint3 {
    return uint3::bitcast(self.bitselect(int3::bitcast(a), int3::bitcast(b)));
  }
}

impl Select<float3> for int3 {
  #[inline(always)]
  fn select(self, a: float3, b: float3) -> float3 {
    return (self >> 31).bitselect(a, b);
  }

  #[inline(always)]
  fn bitselect(self, a: float3, b: float3) -> float3 {
    return float3::bitcast(self.bitselect(int3::bitcast(a), int3::bitcast(b)));
  }
}

impl int3 {
  #[inline]
  pub fn bitcast<T>(x: T) -> int3 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn lo(self) -> int2 {
    return int2(self.0, self.1);
  }

  #[inline]
  pub fn hi(self) -> int2 {
    return int2(self.2, 0);
  }

  #[inline]
  pub fn odd(self) -> int2 {
    return int2(self.1, 0);
  }

  #[inline]
  pub fn even(self) -> int2 {
    return int2(self.0, self.2);
  }
}
