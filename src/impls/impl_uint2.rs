use std;
use ::*;
use ::simd::*;

extern "platform-intrinsic" {
  fn simd_add<T>(x: T, y: T) -> T;
  fn simd_sub<T>(x: T, y: T) -> T;
  fn simd_mul<T>(x: T, y: T) -> T;
  fn simd_div<T>(x: T, y: T) -> T;

  fn simd_shl<T>(x: T, y: T) -> T;
  fn simd_shr<T>(x: T, y: T) -> T;

  fn simd_and<T>(x: T, y: T) -> T;
  fn simd_or<T>(x: T, y: T) -> T;
  fn simd_xor<T>(x: T, y: T) -> T;
}

impl std::ops::Add for uint2 {
  type Output = Self;

  #[inline]
  fn add(self, other: Self) -> Self {
    return unsafe { simd_add(self, other) };
  }
}

impl std::ops::Add<u32> for uint2 {
  type Output = Self;

  #[inline]
  fn add(self, other: u32) -> Self {
    return unsafe { simd_add(self, uint2::broadcast(other)) };
  }
}

impl std::ops::Add<uint2> for u32 {
  type Output = uint2;

  #[inline]
  fn add(self, other: uint2) -> uint2 {
    return unsafe { simd_add(uint2::broadcast(self), other) };
  }
}

impl std::ops::Sub for uint2 {
  type Output = Self;

  #[inline]
  fn sub(self, other: Self) -> Self {
    return unsafe { simd_sub(self, other) };
  }
}

impl std::ops::Sub<u32> for uint2 {
  type Output = Self;

  #[inline]
  fn sub(self, other: u32) -> Self {
    return unsafe { simd_sub(self, uint2::broadcast(other)) };
  }
}

impl std::ops::Sub<uint2> for u32 {
  type Output = uint2;

  #[inline]
  fn sub(self, other: uint2) -> uint2 {
    return unsafe { simd_sub(uint2::broadcast(self), other) };
  }
}

impl std::ops::Mul for uint2 {
  type Output = Self;

  #[inline]
  fn mul(self, other: Self) -> Self {
    return unsafe { simd_mul(self, other) };
  }
}

impl std::ops::Mul<u32> for uint2 {
  type Output = Self;

  #[inline]
  fn mul(self, other: u32) -> Self {
    return unsafe { simd_mul(self, uint2::broadcast(other)) };
  }
}

impl std::ops::Mul<uint2> for u32 {
  type Output = uint2;

  #[inline]
  fn mul(self, other: uint2) -> uint2 {
    return unsafe { simd_mul(uint2::broadcast(self), other) };
  }
}

impl std::ops::Div for uint2 {
  type Output = Self;

  #[inline]
  fn div(self, other: Self) -> Self {
    return unsafe { simd_div(self, other) };
  }
}

impl std::ops::Div<u32> for uint2 {
  type Output = Self;

  #[inline]
  fn div(self, other: u32) -> Self {
    return unsafe { simd_div(self, uint2::broadcast(other)) };
  }
}

impl std::ops::Div<uint2> for u32 {
  type Output = uint2;

  #[inline]
  fn div(self, other: uint2) -> uint2 {
    return unsafe { simd_div(uint2::broadcast(self), other) };
  }
}

impl std::ops::BitAnd for uint2 {
  type Output = Self;

  #[inline]
  fn bitand(self, other: Self) -> Self {
    return unsafe { simd_and(self, other) };
  }
}

impl std::ops::BitAnd<u32> for uint2 {
  type Output = Self;

  #[inline]
  fn bitand(self, other: u32) -> Self {
    return unsafe { simd_and(self, uint2::broadcast(other)) };
  }
}

impl std::ops::BitAnd<uint2> for u32 {
  type Output = uint2;

  #[inline]
  fn bitand(self, other: uint2) -> uint2 {
    return unsafe { simd_and(uint2::broadcast(self), other) };
  }
}

impl std::ops::BitOr for uint2 {
  type Output = Self;

  #[inline]
  fn bitor(self, other: Self) -> Self {
    return unsafe { simd_or(self, other) };
  }
}

impl std::ops::BitOr<u32> for uint2 {
  type Output = Self;

  #[inline]
  fn bitor(self, other: u32) -> Self {
    return unsafe { simd_or(self, uint2::broadcast(other)) };
  }
}

impl std::ops::BitOr<uint2> for u32 {
  type Output = uint2;

  #[inline]
  fn bitor(self, other: uint2) -> uint2 {
    return unsafe { simd_or(uint2::broadcast(self), other) };
  }
}

impl std::ops::BitXor for uint2 {
  type Output = Self;

  #[inline]
  fn bitxor(self, other: Self) -> Self {
    return unsafe { simd_xor(self, other) };
  }
}

impl std::ops::BitXor<u32> for uint2 {
  type Output = Self;

  #[inline]
  fn bitxor(self, other: u32) -> Self {
    return unsafe { simd_xor(self, uint2::broadcast(other)) };
  }
}

impl std::ops::BitXor<uint2> for u32 {
  type Output = uint2;

  #[inline]
  fn bitxor(self, other: uint2) -> uint2 {
    return unsafe { simd_xor(uint2::broadcast(self), other) };
  }
}

impl std::ops::Shl<uint2> for uint2 {
  type Output = Self;

  #[inline]
  fn shl(self, other: Self) -> Self {
    return unsafe { simd_shl(self, other) };
  }
}

impl std::ops::Shl<u32> for uint2 {
  type Output = Self;

  #[inline]
  fn shl(self, other: u32) -> Self {
    return unsafe { simd_shl(self, uint2::broadcast(other)) };
  }
}

impl std::ops::Shl<uint2> for u32 {
  type Output = uint2;

  #[inline]
  fn shl(self, other: uint2) -> uint2 {
    return unsafe { simd_shl(uint2::broadcast(self), other) };
  }
}

impl std::ops::Shr<uint2> for uint2 {
  type Output = Self;

  #[inline]
  fn shr(self, other: Self) -> Self {
    return unsafe { simd_shr(self, other) };
  }
}

impl std::ops::Shr<u32> for uint2 {
  type Output = Self;

  #[inline]
  fn shr(self, other: u32) -> Self {
    return unsafe { simd_shr(self, uint2::broadcast(other)) };
  }
}

impl std::ops::Shr<uint2> for u32 {
  type Output = uint2;

  #[inline]
  fn shr(self, other: uint2) -> uint2 {
    return unsafe { simd_shr(uint2::broadcast(self), other) };
  }
}

impl std::ops::Not for uint2 {
  type Output = Self;

  #[inline]
  fn not(self) -> Self {
    return self ^ std::u32::MAX;
  }
}

impl PartialEq for uint2 {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    return simd::eq(*self, *other).all();
  }

  #[inline]
  fn ne(&self, other: &Self) -> bool {
    return simd::ne(*self, *other).all();
  }
}

impl simd::Vector for uint2 {
  type Scalar = u32;
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
    return uint2::to_char(simd::min(self, uint2::broadcast(std::i8::MAX as u32)));
  }

  #[inline(always)]
  fn to_uchar_sat(self) -> uchar2 {
    return uint2::to_uchar(simd::min(self, uint2::broadcast(std::u8::MAX as u32)));
  }

  #[inline(always)]
  fn to_short_sat(self) -> short2 {
    return uint2::to_short(simd::min(self, uint2::broadcast(std::i16::MAX as u32)));
  }

  #[inline(always)]
  fn to_ushort_sat(self) -> ushort2 {
    return uint2::to_ushort(simd::min(self, uint2::broadcast(std::u16::MAX as u32)));
  }

  #[inline(always)]
  fn to_int_sat(self) -> int2 {
    return uint2::to_int(simd::min(self, uint2::broadcast(std::i32::MAX as u32)));
  }

  #[inline(always)]
  fn to_uint_sat(self) -> uint2 {
    return self;
  }

  #[inline(always)]
  fn to_long_sat(self) -> long2 {
    return uint2::to_long(simd::min(self, uint2::broadcast(std::i64::MAX as u32)));
  }

  #[inline(always)]
  fn to_ulong_sat(self) -> ulong2 {
    return uint2::to_ulong(self);
  }
}

impl simd::Dot for uint2 {
  type DotProduct = u32;
  #[inline(always)]
  fn dot(self, other: Self) -> Self::DotProduct {
    return simd::reduce_add(self * other);
  }
}

impl simd::Integer for uint2 {
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
    return self.reduce_and() & 0x80000000 != 0;
  }

  #[inline(always)]
  fn any(self) -> bool {
    return self.reduce_or() & 0x80000000 != 0;
  }
}

impl uint2 {
  #[inline]
  pub fn bitcast<T>(x: T) -> uint2 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn broadcast(x: u32) -> Self {
    return uint2(x, x);
  }

  #[inline]
  pub fn lo(self) -> u32 {
    return self.0;
  }

  #[inline]
  pub fn hi(self) -> u32 {
    return self.1;
  }

  #[inline]
  pub fn odd(self) -> u32 {
    return self.1;
  }

  #[inline]
  pub fn even(self) -> u32 {
    return self.0;
  }
}
