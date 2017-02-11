use std;
use ::*;

extern "platform-intrinsic" {
  fn simd_shl<T>(x: T, y: T) -> T;
  fn simd_shr<T>(x: T, y: T) -> T;
}

impl std::ops::Shl<int2> for int2 {
  type Output = Self;

  #[inline]
  fn shl(self, other: Self) -> Self {
    return unsafe { simd_shl(self, other) };
  }
}

impl std::ops::Shl<i32> for int2 {
  type Output = Self;

  #[inline]
  fn shl(self, other: i32) -> Self {
    return unsafe { simd_shl(self, int2::broadcast(other)) };
  }
}

impl std::ops::Shl<int2> for i32 {
  type Output = int2;

  #[inline]
  fn shl(self, other: int2) -> int2 {
    return unsafe { simd_shl(int2::broadcast(self), other) };
  }
}

impl std::ops::Shr<int2> for int2 {
  type Output = Self;

  #[inline]
  fn shr(self, other: Self) -> Self {
    return unsafe { simd_shr(self, other) };
  }
}

impl std::ops::Shr<i32> for int2 {
  type Output = Self;

  #[inline]
  fn shr(self, other: i32) -> Self {
    return unsafe { simd_shr(self, int2::broadcast(other)) };
  }
}

impl std::ops::Shr<int2> for i32 {
  type Output = int2;

  #[inline]
  fn shr(self, other: int2) -> int2 {
    return unsafe { simd_shr(int2::broadcast(self), other) };
  }
}

impl simd::Vector for int2 {
  type Scalar = i32;
  type Boolean = int2;

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
  fn abs(self) -> Self {
    let mask = self >> 31;
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
    return self.0 + self.1;
  }

  #[inline(always)]
  fn reduce_min(self) -> Self::Scalar {
    return std::cmp::min(self.0, self.1);
  }

  #[inline(always)]
  fn reduce_max(self) -> Self::Scalar {
    return std::cmp::max(self.0, self.1);
  }

  #[inline(always)]
  fn to_char_sat(self) -> char2 {
    return int2::to_char(simd::clamp(self, int2::broadcast(std::i8::MIN as i32), int2::broadcast(std::i8::MAX as i32)));
  }

  #[inline(always)]
  fn to_uchar_sat(self) -> uchar2 {
    return int2::to_uchar(simd::clamp(self, int2::broadcast(std::u8::MIN as i32), int2::broadcast(std::u8::MAX as i32)));
  }

  #[inline(always)]
  fn to_short_sat(self) -> short2 {
    return int2::to_short(simd::clamp(self, int2::broadcast(std::i16::MIN as i32), int2::broadcast(std::i16::MAX as i32)));
  }

  #[inline(always)]
  fn to_ushort_sat(self) -> ushort2 {
    return int2::to_ushort(simd::clamp(self, int2::broadcast(std::u16::MIN as i32), int2::broadcast(std::u16::MAX as i32)));
  }

  #[inline(always)]
  fn to_int_sat(self) -> int2 {
    return self;
  }

  #[inline(always)]
  fn to_uint_sat(self) -> uint2 {
    return int2::to_uint(simd::max(self, int2::broadcast(0)));
  }

  #[inline(always)]
  fn to_long_sat(self) -> long2 {
    return int2::to_long(self);
  }

  #[inline(always)]
  fn to_ulong_sat(self) -> ulong2 {
    return int2::to_ulong(simd::max(self, int2::broadcast(0)));
  }
}

impl simd::Dot for int2 {
  type DotProduct = i32;
  #[inline(always)]
  fn dot(self, other: Self) -> Self::DotProduct {
    return simd::reduce_add(self * other);
  }
}

impl simd::Integer for int2 {
  #[inline(always)]
  fn reduce_and(self) -> Self::Scalar {
    return self.0 & self.1
  }

  #[inline(always)]
  fn reduce_or(self) -> Self::Scalar {
    return self.0 | self.1
  }

  #[inline(always)]
  fn reduce_xor(self) -> Self::Scalar {
    return self.0 ^ self.1
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

impl simd::Select<int2> for int2 {
  #[inline(always)]
  fn select(self, a: int2, b: int2) -> int2 {
    return (self >> 31).bitselect(a, b);
  }

  #[inline(always)]
  fn bitselect(self, a: int2, b: int2) -> int2 {
    return (a & !self) | (b & self);
  }
}

impl simd::Select<uint2> for int2 {
  #[inline(always)]
  fn select(self, a: uint2, b: uint2) -> uint2 {
    return (self >> 31).bitselect(a, b);
  }

  #[inline(always)]
  fn bitselect(self, a: uint2, b: uint2) -> uint2 {
    return uint2::bitcast(self.bitselect(int2::bitcast(a), int2::bitcast(b)));
  }
}

impl simd::Select<float2> for int2 {
  #[inline(always)]
  fn select(self, a: float2, b: float2) -> float2 {
    return (self >> 31).bitselect(a, b);
  }

  #[inline(always)]
  fn bitselect(self, a: float2, b: float2) -> float2 {
    return float2::bitcast(self.bitselect(int2::bitcast(a), int2::bitcast(b)));
  }
}

impl int2 {
  #[inline]
  pub fn bitcast<T>(x: T) -> int2 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn broadcast(x: i32) -> Self {
    return int2(x, x);
  }

  #[inline]
  pub fn lo(self) -> i32 {
    return self.0;
  }

  #[inline]
  pub fn hi(self) -> i32 {
    return self.1;
  }

  #[inline]
  pub fn odd(self) -> i32 {
    return self.1;
  }

  #[inline]
  pub fn even(self) -> i32 {
    return self.0;
  }
}
