use std;
use ::*;

impl simd::Vector for short3 {
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
  fn abs(self) -> Self {
    let mask = self >> 15;
    return (self ^ mask) - mask;
  }

  #[inline(always)]
  fn max(self, other: Self) -> Self {
    return simd::bitselect(simd::gt(other, self), self, other);
  }

  #[inline(always)]
  fn min(self, other: Self) -> Self {
    return simd::bitselect(simd::lt(other, self), self, other);
  }

  #[inline(always)]
  fn reduce_add(self) -> Self::Scalar {
    return self.0 + self.1 + self.2;
  }

  #[inline(always)]
  fn reduce_min(self) -> Self::Scalar {
    return std::cmp::min(simd::reduce_min(self.lo()), self.2);
  }

  #[inline(always)]
  fn reduce_max(self) -> Self::Scalar {
    return std::cmp::max(simd::reduce_max(self.lo()), self.2);
  }

  #[inline(always)]
  fn to_char_sat(self) -> char3 {
    return short3::to_char(simd::clamp(self, short3::broadcast(std::i8::MIN as i16), short3::broadcast(std::i8::MAX as i16)));
  }

  #[inline(always)]
  fn to_uchar_sat(self) -> uchar3 {
    return short3::to_uchar(simd::clamp(self, short3::broadcast(std::u8::MIN as i16), short3::broadcast(std::u8::MAX as i16)));
  }

  #[inline(always)]
  fn to_short_sat(self) -> short3 {
    return self;
  }

  #[inline(always)]
  fn to_ushort_sat(self) -> ushort3 {
    return short3::to_ushort(simd::max(self, short3::broadcast(0)));
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
    return short3::to_uint(simd::max(self, short3::broadcast(0)));
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
    return short3::to_ulong(simd::max(self, short3::broadcast(0)));
  }

  #[inline(always)]
  fn to_double(self) -> double3 {
    return double3(self.0 as f64, self.1 as f64, self.2 as f64);
  }
}

impl simd::Dot for short3 {
  type DotProduct = i16;
  #[inline(always)]
  fn dot(self, other: Self) -> Self::DotProduct {
    return simd::reduce_add(self * other);
  }
}

impl simd::Integer for short3 {
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
    return self.reduce_and() & std::i16::MIN != 0;
  }

  #[inline(always)]
  fn any(self) -> bool {
    return self.reduce_or() & std::i16::MIN != 0;
  }
}

impl simd::Select<short3> for short3 {
  #[inline(always)]
  fn select(self, a: short3, b: short3) -> short3 {
    return (self >> 15).bitselect(a, b);
  }

  #[inline(always)]
  fn bitselect(self, a: short3, b: short3) -> short3 {
    return (a & !self) | (b & self);
  }
}

impl simd::Select<ushort3> for short3 {
  #[inline(always)]
  fn select(self, a: ushort3, b: ushort3) -> ushort3 {
    return (self >> 15).bitselect(a, b);
  }

  #[inline(always)]
  fn bitselect(self, a: ushort3, b: ushort3) -> ushort3 {
    return ushort3::bitcast(self.bitselect(short3::bitcast(a), short3::bitcast(b)));
  }
}

impl short3 {
  #[inline]
  pub fn bitcast<T>(x: T) -> short3 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn broadcast(x: i16) -> Self {
    return short3(x, x, x);
  }

  #[inline]
  pub fn lo(self) -> short2 {
    return short2(self.0, self.1);
  }

  #[inline]
  pub fn hi(self) -> short2 {
    return short2(self.2, 0);
  }

  #[inline]
  pub fn odd(self) -> short2 {
    return short2(self.1, 0);
  }

  #[inline]
  pub fn even(self) -> short2 {
    return short2(self.0, self.2);
  }
}
