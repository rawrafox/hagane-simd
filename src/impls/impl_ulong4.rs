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

impl std::ops::Add for ulong4 {
  type Output = Self;

  #[inline]
  fn add(self, other: Self) -> Self {
    return unsafe { simd_add(self, other) };
  }
}

impl std::ops::Add<u64> for ulong4 {
  type Output = Self;

  #[inline]
  fn add(self, other: u64) -> Self {
    return unsafe { simd_add(self, ulong4::broadcast(other)) };
  }
}

impl std::ops::Add<ulong4> for u64 {
  type Output = ulong4;

  #[inline]
  fn add(self, other: ulong4) -> ulong4 {
    return unsafe { simd_add(ulong4::broadcast(self), other) };
  }
}

impl std::ops::Sub for ulong4 {
  type Output = Self;

  #[inline]
  fn sub(self, other: Self) -> Self {
    return unsafe { simd_sub(self, other) };
  }
}

impl std::ops::Sub<u64> for ulong4 {
  type Output = Self;

  #[inline]
  fn sub(self, other: u64) -> Self {
    return unsafe { simd_sub(self, ulong4::broadcast(other)) };
  }
}

impl std::ops::Sub<ulong4> for u64 {
  type Output = ulong4;

  #[inline]
  fn sub(self, other: ulong4) -> ulong4 {
    return unsafe { simd_sub(ulong4::broadcast(self), other) };
  }
}

impl std::ops::Mul for ulong4 {
  type Output = Self;

  #[inline]
  fn mul(self, other: Self) -> Self {
    return unsafe { simd_mul(self, other) };
  }
}

impl std::ops::Mul<u64> for ulong4 {
  type Output = Self;

  #[inline]
  fn mul(self, other: u64) -> Self {
    return unsafe { simd_mul(self, ulong4::broadcast(other)) };
  }
}

impl std::ops::Mul<ulong4> for u64 {
  type Output = ulong4;

  #[inline]
  fn mul(self, other: ulong4) -> ulong4 {
    return unsafe { simd_mul(ulong4::broadcast(self), other) };
  }
}

impl std::ops::Div for ulong4 {
  type Output = Self;

  #[inline]
  fn div(self, other: Self) -> Self {
    return unsafe { simd_div(self, other) };
  }
}

impl std::ops::Div<u64> for ulong4 {
  type Output = Self;

  #[inline]
  fn div(self, other: u64) -> Self {
    return unsafe { simd_div(self, ulong4::broadcast(other)) };
  }
}

impl std::ops::Div<ulong4> for u64 {
  type Output = ulong4;

  #[inline]
  fn div(self, other: ulong4) -> ulong4 {
    return unsafe { simd_div(ulong4::broadcast(self), other) };
  }
}

impl std::ops::BitAnd for ulong4 {
  type Output = Self;

  #[inline]
  fn bitand(self, other: Self) -> Self {
    return unsafe { simd_and(self, other) };
  }
}

impl std::ops::BitAnd<u64> for ulong4 {
  type Output = Self;

  #[inline]
  fn bitand(self, other: u64) -> Self {
    return unsafe { simd_and(self, ulong4::broadcast(other)) };
  }
}

impl std::ops::BitAnd<ulong4> for u64 {
  type Output = ulong4;

  #[inline]
  fn bitand(self, other: ulong4) -> ulong4 {
    return unsafe { simd_and(ulong4::broadcast(self), other) };
  }
}

impl std::ops::BitOr for ulong4 {
  type Output = Self;

  #[inline]
  fn bitor(self, other: Self) -> Self {
    return unsafe { simd_or(self, other) };
  }
}

impl std::ops::BitOr<u64> for ulong4 {
  type Output = Self;

  #[inline]
  fn bitor(self, other: u64) -> Self {
    return unsafe { simd_or(self, ulong4::broadcast(other)) };
  }
}

impl std::ops::BitOr<ulong4> for u64 {
  type Output = ulong4;

  #[inline]
  fn bitor(self, other: ulong4) -> ulong4 {
    return unsafe { simd_or(ulong4::broadcast(self), other) };
  }
}

impl std::ops::BitXor for ulong4 {
  type Output = Self;

  #[inline]
  fn bitxor(self, other: Self) -> Self {
    return unsafe { simd_xor(self, other) };
  }
}

impl std::ops::BitXor<u64> for ulong4 {
  type Output = Self;

  #[inline]
  fn bitxor(self, other: u64) -> Self {
    return unsafe { simd_xor(self, ulong4::broadcast(other)) };
  }
}

impl std::ops::BitXor<ulong4> for u64 {
  type Output = ulong4;

  #[inline]
  fn bitxor(self, other: ulong4) -> ulong4 {
    return unsafe { simd_xor(ulong4::broadcast(self), other) };
  }
}

impl std::ops::Shl<ulong4> for ulong4 {
  type Output = Self;

  #[inline]
  fn shl(self, other: Self) -> Self {
    return unsafe { simd_shl(self, other) };
  }
}

impl std::ops::Shl<u64> for ulong4 {
  type Output = Self;

  #[inline]
  fn shl(self, other: u64) -> Self {
    return unsafe { simd_shl(self, ulong4::broadcast(other)) };
  }
}

impl std::ops::Shl<ulong4> for u64 {
  type Output = ulong4;

  #[inline]
  fn shl(self, other: ulong4) -> ulong4 {
    return unsafe { simd_shl(ulong4::broadcast(self), other) };
  }
}

impl std::ops::Shr<ulong4> for ulong4 {
  type Output = Self;

  #[inline]
  fn shr(self, other: Self) -> Self {
    return unsafe { simd_shr(self, other) };
  }
}

impl std::ops::Shr<u64> for ulong4 {
  type Output = Self;

  #[inline]
  fn shr(self, other: u64) -> Self {
    return unsafe { simd_shr(self, ulong4::broadcast(other)) };
  }
}

impl std::ops::Shr<ulong4> for u64 {
  type Output = ulong4;

  #[inline]
  fn shr(self, other: ulong4) -> ulong4 {
    return unsafe { simd_shr(ulong4::broadcast(self), other) };
  }
}

impl std::ops::Not for ulong4 {
  type Output = Self;

  #[inline]
  fn not(self) -> Self {
    return self ^ std::u64::MAX;
  }
}

impl PartialEq for ulong4 {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    return simd::eq(*self, *other).all();
  }

  #[inline]
  fn ne(&self, other: &Self) -> bool {
    return simd::ne(*self, *other).all();
  }
}

impl simd::Vector for ulong4 {
  type Scalar = u64;
  type Boolean = long4;

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
    return ulong4::to_char(simd::min(self, ulong4::broadcast(std::i8::MAX as u64)));
  }

  #[inline(always)]
  fn to_uchar_sat(self) -> uchar4 {
    return ulong4::to_uchar(simd::min(self, ulong4::broadcast(std::u8::MAX as u64)));
  }

  #[inline(always)]
  fn to_short_sat(self) -> short4 {
    return ulong4::to_short(simd::min(self, ulong4::broadcast(std::i16::MAX as u64)));
  }

  #[inline(always)]
  fn to_ushort_sat(self) -> ushort4 {
    return ulong4::to_ushort(simd::min(self, ulong4::broadcast(std::u16::MAX as u64)));
  }

  #[inline(always)]
  fn to_int_sat(self) -> int4 {
    return ulong4::to_int(simd::min(self, ulong4::broadcast(std::i32::MAX as u64)));
  }

  #[inline(always)]
  fn to_uint_sat(self) -> uint4 {
    return ulong4::to_uint(simd::min(self, ulong4::broadcast(std::u32::MAX as u64)));
  }

  #[inline(always)]
  fn to_long_sat(self) -> long4 {
    return ulong4::to_long(simd::min(self, ulong4::broadcast(std::i64::MAX as u64)));
  }

  #[inline(always)]
  fn to_ulong_sat(self) -> ulong4 {
    return self;
  }
}

impl simd::Dot for ulong4 {
  type DotProduct = u64;
  #[inline(always)]
  fn dot(self, other: Self) -> Self::DotProduct {
    return simd::reduce_add(self * other);
  }
}

impl simd::Integer for ulong4 {
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
    return self.reduce_and() & 0x8000000000000000 != 0;
  }

  #[inline(always)]
  fn any(self) -> bool {
    return self.reduce_or() & 0x8000000000000000 != 0;
  }
}

impl ulong4 {
  #[inline]
  pub fn bitcast<T>(x: T) -> ulong4 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn broadcast(x: u64) -> Self {
    return ulong4(x, x, x, x);
  }

  #[inline]
  pub fn lo(self) -> ulong2 {
    return ulong2(self.0, self.1);
  }

  #[inline]
  pub fn hi(self) -> ulong2 {
    return ulong2(self.2, self.3);
  }

  #[inline]
  pub fn odd(self) -> ulong2 {
    return ulong2(self.1, self.3);
  }

  #[inline]
  pub fn even(self) -> ulong2 {
    return ulong2(self.0, self.2);
  }
}
