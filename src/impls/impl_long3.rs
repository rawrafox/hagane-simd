use std;
use ::*;
use ::simd::*;

extern "platform-intrinsic" {
  fn simd_shl<T>(x: T, y: T) -> T;
  fn simd_shr<T>(x: T, y: T) -> T;
}

impl std::ops::Shl<long3> for long3 {
  type Output = Self;

  #[inline]
  fn shl(self, other: Self) -> Self {
    return unsafe { simd_shl(self, other) };
  }
}

impl std::ops::Shl<i64> for long3 {
  type Output = Self;

  #[inline]
  fn shl(self, other: i64) -> Self {
    return unsafe { simd_shl(self, long3::broadcast(other)) };
  }
}

impl std::ops::Shl<long3> for i64 {
  type Output = long3;

  #[inline]
  fn shl(self, other: long3) -> long3 {
    return unsafe { simd_shl(long3::broadcast(self), other) };
  }
}

impl std::ops::Shr<long3> for long3 {
  type Output = Self;

  #[inline]
  fn shr(self, other: Self) -> Self {
    return unsafe { simd_shr(self, other) };
  }
}

impl std::ops::Shr<i64> for long3 {
  type Output = Self;

  #[inline]
  fn shr(self, other: i64) -> Self {
    return unsafe { simd_shr(self, long3::broadcast(other)) };
  }
}

impl std::ops::Shr<long3> for i64 {
  type Output = long3;

  #[inline]
  fn shr(self, other: long3) -> long3 {
    return unsafe { simd_shr(long3::broadcast(self), other) };
  }
}

impl std::ops::Not for long3 {
  type Output = Self;

  #[inline]
  fn not(self) -> Self {
    return self ^ -1;
  }
}

impl PartialEq for long3 {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    return simd::eq(*self, *other).all();
  }

  #[inline]
  fn ne(&self, other: &Self) -> bool {
    return simd::ne(*self, *other).all();
  }
}

impl simd::Vector for long3 {
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
  fn abs(self) -> Self {
    let mask = self >> 63;
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
    return long3::to_char(simd::clamp(self, long3::broadcast(std::i8::MIN as i64), long3::broadcast(std::i8::MAX as i64)));
  }

  #[inline(always)]
  fn to_uchar_sat(self) -> uchar3 {
    return long3::to_uchar(simd::clamp(self, long3::broadcast(std::u8::MIN as i64), long3::broadcast(std::u8::MAX as i64)));
  }

  #[inline(always)]
  fn to_short_sat(self) -> short3 {
    return long3::to_short(simd::clamp(self, long3::broadcast(std::i16::MIN as i64), long3::broadcast(std::i16::MAX as i64)));
  }

  #[inline(always)]
  fn to_ushort_sat(self) -> ushort3 {
    return long3::to_ushort(simd::clamp(self, long3::broadcast(std::u16::MIN as i64), long3::broadcast(std::u16::MAX as i64)));
  }

  #[inline(always)]
  fn to_int_sat(self) -> int3 {
    return long3::to_int(simd::clamp(self, long3::broadcast(std::i32::MIN as i64), long3::broadcast(std::i32::MAX as i64)));
  }

  #[inline(always)]
  fn to_uint_sat(self) -> uint3 {
    return long3::to_uint(simd::clamp(self, long3::broadcast(std::u32::MIN as i64), long3::broadcast(std::u32::MAX as i64)));
  }

  #[inline(always)]
  fn to_long_sat(self) -> long3 {
    return self;
  }

  #[inline(always)]
  fn to_ulong_sat(self) -> ulong3 {
    return long3::to_ulong(simd::max(self, long3::broadcast(0)));
  }
}

impl simd::Dot for long3 {
  type DotProduct = i64;
  #[inline(always)]
  fn dot(self, other: Self) -> Self::DotProduct {
    return simd::reduce_add(self * other);
  }
}

impl simd::Integer for long3 {
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
    return self.reduce_and() & std::i64::MIN != 0;
  }

  #[inline(always)]
  fn any(self) -> bool {
    return self.reduce_or() & std::i64::MIN != 0;
  }
}

impl simd::Select<long3> for long3 {
  #[inline(always)]
  fn select(self, a: long3, b: long3) -> long3 {
    return (self >> 63).bitselect(a, b);
  }

  #[inline(always)]
  fn bitselect(self, a: long3, b: long3) -> long3 {
    return (a & !self) | (b & self);
  }
}

impl simd::Select<ulong3> for long3 {
  #[inline(always)]
  fn select(self, a: ulong3, b: ulong3) -> ulong3 {
    return (self >> 63).bitselect(a, b);
  }

  #[inline(always)]
  fn bitselect(self, a: ulong3, b: ulong3) -> ulong3 {
    return ulong3::bitcast(self.bitselect(long3::bitcast(a), long3::bitcast(b)));
  }
}

impl simd::Select<double3> for long3 {
  #[inline(always)]
  fn select(self, a: double3, b: double3) -> double3 {
    return (self >> 63).bitselect(a, b);
  }

  #[inline(always)]
  fn bitselect(self, a: double3, b: double3) -> double3 {
    return double3::bitcast(self.bitselect(long3::bitcast(a), long3::bitcast(b)));
  }
}

impl long3 {
  #[inline]
  pub fn bitcast<T>(x: T) -> long3 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn broadcast(x: i64) -> Self {
    return long3(x, x, x);
  }

  #[inline]
  pub fn lo(self) -> long2 {
    return long2(self.0, self.1);
  }

  #[inline]
  pub fn hi(self) -> long2 {
    return long2(self.2, 0);
  }

  #[inline]
  pub fn odd(self) -> long2 {
    return long2(self.1, 0);
  }

  #[inline]
  pub fn even(self) -> long2 {
    return long2(self.0, self.2);
  }
}
