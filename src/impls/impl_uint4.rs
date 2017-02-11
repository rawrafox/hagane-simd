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

impl std::ops::Add for uint4 {
  type Output = Self;

  #[inline]
  fn add(self, other: Self) -> Self {
    return unsafe { simd_add(self, other) };
  }
}

impl std::ops::Add<u32> for uint4 {
  type Output = Self;

  #[inline]
  fn add(self, other: u32) -> Self {
    return unsafe { simd_add(self, uint4::broadcast(other)) };
  }
}

impl std::ops::Add<uint4> for u32 {
  type Output = uint4;

  #[inline]
  fn add(self, other: uint4) -> uint4 {
    return unsafe { simd_add(uint4::broadcast(self), other) };
  }
}

impl std::ops::Sub for uint4 {
  type Output = Self;

  #[inline]
  fn sub(self, other: Self) -> Self {
    return unsafe { simd_sub(self, other) };
  }
}

impl std::ops::Sub<u32> for uint4 {
  type Output = Self;

  #[inline]
  fn sub(self, other: u32) -> Self {
    return unsafe { simd_sub(self, uint4::broadcast(other)) };
  }
}

impl std::ops::Sub<uint4> for u32 {
  type Output = uint4;

  #[inline]
  fn sub(self, other: uint4) -> uint4 {
    return unsafe { simd_sub(uint4::broadcast(self), other) };
  }
}

impl std::ops::Mul for uint4 {
  type Output = Self;

  #[inline]
  fn mul(self, other: Self) -> Self {
    return unsafe { simd_mul(self, other) };
  }
}

impl std::ops::Mul<u32> for uint4 {
  type Output = Self;

  #[inline]
  fn mul(self, other: u32) -> Self {
    return unsafe { simd_mul(self, uint4::broadcast(other)) };
  }
}

impl std::ops::Mul<uint4> for u32 {
  type Output = uint4;

  #[inline]
  fn mul(self, other: uint4) -> uint4 {
    return unsafe { simd_mul(uint4::broadcast(self), other) };
  }
}

impl std::ops::Div for uint4 {
  type Output = Self;

  #[inline]
  fn div(self, other: Self) -> Self {
    return unsafe { simd_div(self, other) };
  }
}

impl std::ops::Div<u32> for uint4 {
  type Output = Self;

  #[inline]
  fn div(self, other: u32) -> Self {
    return unsafe { simd_div(self, uint4::broadcast(other)) };
  }
}

impl std::ops::Div<uint4> for u32 {
  type Output = uint4;

  #[inline]
  fn div(self, other: uint4) -> uint4 {
    return unsafe { simd_div(uint4::broadcast(self), other) };
  }
}

impl std::ops::BitAnd for uint4 {
  type Output = Self;

  #[inline]
  fn bitand(self, other: Self) -> Self {
    return unsafe { simd_and(self, other) };
  }
}

impl std::ops::BitAnd<u32> for uint4 {
  type Output = Self;

  #[inline]
  fn bitand(self, other: u32) -> Self {
    return unsafe { simd_and(self, uint4::broadcast(other)) };
  }
}

impl std::ops::BitAnd<uint4> for u32 {
  type Output = uint4;

  #[inline]
  fn bitand(self, other: uint4) -> uint4 {
    return unsafe { simd_and(uint4::broadcast(self), other) };
  }
}

impl std::ops::BitOr for uint4 {
  type Output = Self;

  #[inline]
  fn bitor(self, other: Self) -> Self {
    return unsafe { simd_or(self, other) };
  }
}

impl std::ops::BitOr<u32> for uint4 {
  type Output = Self;

  #[inline]
  fn bitor(self, other: u32) -> Self {
    return unsafe { simd_or(self, uint4::broadcast(other)) };
  }
}

impl std::ops::BitOr<uint4> for u32 {
  type Output = uint4;

  #[inline]
  fn bitor(self, other: uint4) -> uint4 {
    return unsafe { simd_or(uint4::broadcast(self), other) };
  }
}

impl std::ops::BitXor for uint4 {
  type Output = Self;

  #[inline]
  fn bitxor(self, other: Self) -> Self {
    return unsafe { simd_xor(self, other) };
  }
}

impl std::ops::BitXor<u32> for uint4 {
  type Output = Self;

  #[inline]
  fn bitxor(self, other: u32) -> Self {
    return unsafe { simd_xor(self, uint4::broadcast(other)) };
  }
}

impl std::ops::BitXor<uint4> for u32 {
  type Output = uint4;

  #[inline]
  fn bitxor(self, other: uint4) -> uint4 {
    return unsafe { simd_xor(uint4::broadcast(self), other) };
  }
}

impl std::ops::Shl<uint4> for uint4 {
  type Output = Self;

  #[inline]
  fn shl(self, other: Self) -> Self {
    return unsafe { simd_shl(self, other) };
  }
}

impl std::ops::Shl<u32> for uint4 {
  type Output = Self;

  #[inline]
  fn shl(self, other: u32) -> Self {
    return unsafe { simd_shl(self, uint4::broadcast(other)) };
  }
}

impl std::ops::Shl<uint4> for u32 {
  type Output = uint4;

  #[inline]
  fn shl(self, other: uint4) -> uint4 {
    return unsafe { simd_shl(uint4::broadcast(self), other) };
  }
}

impl std::ops::Shr<uint4> for uint4 {
  type Output = Self;

  #[inline]
  fn shr(self, other: Self) -> Self {
    return unsafe { simd_shr(self, other) };
  }
}

impl std::ops::Shr<u32> for uint4 {
  type Output = Self;

  #[inline]
  fn shr(self, other: u32) -> Self {
    return unsafe { simd_shr(self, uint4::broadcast(other)) };
  }
}

impl std::ops::Shr<uint4> for u32 {
  type Output = uint4;

  #[inline]
  fn shr(self, other: uint4) -> uint4 {
    return unsafe { simd_shr(uint4::broadcast(self), other) };
  }
}

impl std::ops::Not for uint4 {
  type Output = Self;

  #[inline]
  fn not(self) -> Self {
    return self ^ std::u32::MAX;
  }
}

impl PartialEq for uint4 {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    return simd::eq(*self, *other).all();
  }

  #[inline]
  fn ne(&self, other: &Self) -> bool {
    return simd::ne(*self, *other).all();
  }
}

impl simd::Vector for uint4 {
  type Scalar = u32;
  type Boolean = int4;

  type CharVector = char4;
  type ShortVector = short4;
  type IntVector = int4;
  type LongVector = long4;

  type UCharVector = uchar4;
  type UShortVector = ushort4;
  type UIntVector = uint4;
  type ULongVector = ulong4;

  type FloatVector = float4;
  type DoubleVector = double4;

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
    return simd::reduce_add(self.lo() + self.hi());
  }

  #[inline(always)]
  fn reduce_min(self) -> Self::Scalar {
    return simd::reduce_min(simd::min(self.lo(), self.hi()));
  }

  #[inline(always)]
  fn reduce_max(self) -> Self::Scalar {
    return simd::reduce_max(simd::max(self.lo(), self.hi()));
  }

  #[inline(always)]
  fn to_char_sat(self) -> char4 {
    return uint4::to_char(simd::min(self, uint4::broadcast(std::i8::MAX as u32)));
  }

  #[inline(always)]
  fn to_uchar_sat(self) -> uchar4 {
    return uint4::to_uchar(simd::min(self, uint4::broadcast(std::u8::MAX as u32)));
  }

  #[inline(always)]
  fn to_short_sat(self) -> short4 {
    return uint4::to_short(simd::min(self, uint4::broadcast(std::i16::MAX as u32)));
  }

  #[inline(always)]
  fn to_ushort_sat(self) -> ushort4 {
    return uint4::to_ushort(simd::min(self, uint4::broadcast(std::u16::MAX as u32)));
  }

  #[inline(always)]
  fn to_int_sat(self) -> int4 {
    return uint4::to_int(simd::min(self, uint4::broadcast(std::i32::MAX as u32)));
  }

  #[inline(always)]
  fn to_uint_sat(self) -> uint4 {
    return self;
  }

  #[inline(always)]
  fn to_long_sat(self) -> long4 {
    return uint4::to_long(simd::min(self, uint4::broadcast(std::i64::MAX as u32)));
  }

  #[inline(always)]
  fn to_ulong_sat(self) -> ulong4 {
    return uint4::to_ulong(self);
  }
}

impl simd::Dot for uint4 {
  type DotProduct = u32;
  #[inline(always)]
  fn dot(self, other: Self) -> Self::DotProduct {
    return simd::reduce_add(self * other);
  }
}

impl simd::Integer for uint4 {
  #[inline(always)]
  fn reduce_and(self) -> Self::Scalar {
    return (self.lo() & self.hi()).reduce_and();
  }

  #[inline(always)]
  fn reduce_or(self) -> Self::Scalar {
    return (self.lo() | self.hi()).reduce_or();
  }

  #[inline(always)]
  fn reduce_xor(self) -> Self::Scalar {
    return (self.lo() ^ self.hi()).reduce_xor();
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

impl uint4 {
  #[inline]
  pub fn bitcast<T>(x: T) -> uint4 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn broadcast(x: u32) -> Self {
    return uint4(x, x, x, x);
  }

  #[inline]
  pub fn lo(self) -> uint2 {
    return uint2(self.0, self.1);
  }

  #[inline]
  pub fn hi(self) -> uint2 {
    return uint2(self.2, self.3);
  }

  #[inline]
  pub fn odd(self) -> uint2 {
    return uint2(self.1, self.3);
  }

  #[inline]
  pub fn even(self) -> uint2 {
    return uint2(self.0, self.2);
  }
}
