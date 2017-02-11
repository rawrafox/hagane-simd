use std;
use ::*;
use ::simd::*;

extern "platform-intrinsic" {
  fn simd_shl<T>(x: T, y: T) -> T;
  fn simd_shr<T>(x: T, y: T) -> T;
}

impl std::ops::Shl<uint3> for uint3 {
  type Output = Self;

  #[inline]
  fn shl(self, other: Self) -> Self {
    return unsafe { simd_shl(self, other) };
  }
}

impl std::ops::Shl<u32> for uint3 {
  type Output = Self;

  #[inline]
  fn shl(self, other: u32) -> Self {
    return unsafe { simd_shl(self, uint3::broadcast(other)) };
  }
}

impl std::ops::Shl<uint3> for u32 {
  type Output = uint3;

  #[inline]
  fn shl(self, other: uint3) -> uint3 {
    return unsafe { simd_shl(uint3::broadcast(self), other) };
  }
}

impl std::ops::Shr<uint3> for uint3 {
  type Output = Self;

  #[inline]
  fn shr(self, other: Self) -> Self {
    return unsafe { simd_shr(self, other) };
  }
}

impl std::ops::Shr<u32> for uint3 {
  type Output = Self;

  #[inline]
  fn shr(self, other: u32) -> Self {
    return unsafe { simd_shr(self, uint3::broadcast(other)) };
  }
}

impl std::ops::Shr<uint3> for u32 {
  type Output = uint3;

  #[inline]
  fn shr(self, other: uint3) -> uint3 {
    return unsafe { simd_shr(uint3::broadcast(self), other) };
  }
}

impl std::ops::Not for uint3 {
  type Output = Self;

  #[inline]
  fn not(self) -> Self {
    return self ^ std::u32::MAX;
  }
}

impl PartialEq for uint3 {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    return simd::eq(*self, *other).all();
  }

  #[inline]
  fn ne(&self, other: &Self) -> bool {
    return simd::ne(*self, *other).all();
  }
}

impl simd::Vector for uint3 {
  type Scalar = u32;
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
    return self;
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
    return uint3::to_char(simd::min(self, uint3::broadcast(std::i8::MAX as u32)));
  }

  #[inline(always)]
  fn to_uchar_sat(self) -> uchar3 {
    return uint3::to_uchar(simd::min(self, uint3::broadcast(std::u8::MAX as u32)));
  }

  #[inline(always)]
  fn to_short_sat(self) -> short3 {
    return uint3::to_short(simd::min(self, uint3::broadcast(std::i16::MAX as u32)));
  }

  #[inline(always)]
  fn to_ushort_sat(self) -> ushort3 {
    return uint3::to_ushort(simd::min(self, uint3::broadcast(std::u16::MAX as u32)));
  }

  #[inline(always)]
  fn to_int_sat(self) -> int3 {
    return uint3::to_int(simd::min(self, uint3::broadcast(std::i32::MAX as u32)));
  }

  #[inline(always)]
  fn to_uint_sat(self) -> uint3 {
    return self;
  }

  #[inline(always)]
  fn to_long(self) -> long3 {
    return long3(self.0 as i64, self.1 as i64, self.2 as i64);
  }

  #[inline(always)]
  fn to_long_sat(self) -> long3 {
    return uint3::to_long(simd::min(self, uint3::broadcast(std::i64::MAX as u32)));
  }

  #[inline(always)]
  fn to_ulong(self) -> ulong3 {
    return ulong3(self.0 as u64, self.1 as u64, self.2 as u64);
  }

  #[inline(always)]
  fn to_ulong_sat(self) -> ulong3 {
    return uint3::to_ulong(self);
  }

  #[inline(always)]
  fn to_double(self) -> double3 {
    return double3(self.0 as f64, self.1 as f64, self.2 as f64);
  }
}

impl simd::Dot for uint3 {
  type DotProduct = u32;
  #[inline(always)]
  fn dot(self, other: Self) -> Self::DotProduct {
    return simd::reduce_add(self * other);
  }
}

impl simd::Integer for uint3 {
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
    return self.reduce_and() & 0x80000000 != 0;
  }

  #[inline(always)]
  fn any(self) -> bool {
    return self.reduce_or() & 0x80000000 != 0;
  }
}

impl uint3 {
  #[inline]
  pub fn bitcast<T>(x: T) -> uint3 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn broadcast(x: u32) -> Self {
    return uint3(x, x, x);
  }

  #[inline]
  pub fn lo(self) -> uint2 {
    return uint2(self.0, self.1);
  }

  #[inline]
  pub fn hi(self) -> uint2 {
    return uint2(self.2, 0);
  }

  #[inline]
  pub fn odd(self) -> uint2 {
    return uint2(self.1, 0);
  }

  #[inline]
  pub fn even(self) -> uint2 {
    return uint2(self.0, self.2);
  }
}
