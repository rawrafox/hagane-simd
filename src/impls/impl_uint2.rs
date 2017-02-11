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

  fn simd_cast<T, U>(x: T) -> U;
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
}

impl simd::Dot for uint2 {
  type DotProduct = u32;
  #[inline(always)]
  fn dot(self, other: Self) -> Self::DotProduct {
    return simd::reduce_add(self * other);
  }
}

impl simd::Logic for uint2 {
  #[inline(always)]
  fn all(self) -> bool {
    return (self.0 & self.1) & 0x80000000 != 0;
  }

  #[inline(always)]
  fn any(self) -> bool {
    return (self.0 | self.1) & 0x80000000 != 0;
  }
}

impl simd::Reduce for uint2 {
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
  pub fn madd(x: uint2, y: uint2, z: uint2) -> uint2 {
    return x * y + z;
  }

  #[inline]
  pub fn to_char(x: uint2) -> char2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_char_sat(x: uint2) -> char2 {
    return uint2::to_char(simd::min(x, uint2::broadcast(std::i8::MAX as u32)));
  }

  #[inline]
  pub fn to_uchar(x: uint2) -> uchar2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_uchar_sat(x: uint2) -> uchar2 {
    return uint2::to_uchar(simd::min(x, uint2::broadcast(std::u8::MAX as u32)));
  }

  #[inline]
  pub fn to_short(x: uint2) -> short2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_short_sat(x: uint2) -> short2 {
    return uint2::to_short(simd::min(x, uint2::broadcast(std::i16::MAX as u32)));
  }

  #[inline]
  pub fn to_ushort(x: uint2) -> ushort2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_ushort_sat(x: uint2) -> ushort2 {
    return uint2::to_ushort(simd::min(x, uint2::broadcast(std::u16::MAX as u32)));
  }

  #[inline]
  pub fn to_int(x: uint2) -> int2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_int_sat(x: uint2) -> int2 {
    return uint2::to_int(simd::min(x, uint2::broadcast(std::i32::MAX as u32)));
  }

  #[inline]
  pub fn to_uint(x: uint2) -> uint2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_uint_sat(x: uint2) -> uint2 {
    return x;
  }

  #[inline]
  pub fn to_float(x: uint2) -> float2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_long(x: uint2) -> long2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_long_sat(x: uint2) -> long2 {
    return uint2::to_long(simd::min(x, uint2::broadcast(std::i64::MAX as u32)));
  }

  #[inline]
  pub fn to_ulong(x: uint2) -> ulong2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_ulong_sat(x: uint2) -> ulong2 {
    return uint2::to_ulong(x);
  }

  #[inline]
  pub fn to_double(x: uint2) -> double2 {
    return unsafe { simd_cast(x) };
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
