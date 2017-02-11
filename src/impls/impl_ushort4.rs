use std;
use ::*;
use ::simd::*;

extern "platform-intrinsic" {
  fn simd_shl<T>(x: T, y: T) -> T;
  fn simd_shr<T>(x: T, y: T) -> T;
}

impl std::ops::Shl<ushort4> for ushort4 {
  type Output = Self;

  #[inline]
  fn shl(self, other: Self) -> Self {
    return unsafe { simd_shl(self, other) };
  }
}

impl std::ops::Shl<u16> for ushort4 {
  type Output = Self;

  #[inline]
  fn shl(self, other: u16) -> Self {
    return unsafe { simd_shl(self, ushort4::broadcast(other)) };
  }
}

impl std::ops::Shl<ushort4> for u16 {
  type Output = ushort4;

  #[inline]
  fn shl(self, other: ushort4) -> ushort4 {
    return unsafe { simd_shl(ushort4::broadcast(self), other) };
  }
}

impl std::ops::Shr<ushort4> for ushort4 {
  type Output = Self;

  #[inline]
  fn shr(self, other: Self) -> Self {
    return unsafe { simd_shr(self, other) };
  }
}

impl std::ops::Shr<u16> for ushort4 {
  type Output = Self;

  #[inline]
  fn shr(self, other: u16) -> Self {
    return unsafe { simd_shr(self, ushort4::broadcast(other)) };
  }
}

impl std::ops::Shr<ushort4> for u16 {
  type Output = ushort4;

  #[inline]
  fn shr(self, other: ushort4) -> ushort4 {
    return unsafe { simd_shr(ushort4::broadcast(self), other) };
  }
}

impl std::ops::Not for ushort4 {
  type Output = Self;

  #[inline]
  fn not(self) -> Self {
    return self ^ std::u16::MAX;
  }
}

impl PartialEq for ushort4 {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    return simd::eq(*self, *other).all();
  }

  #[inline]
  fn ne(&self, other: &Self) -> bool {
    return simd::ne(*self, *other).all();
  }
}

impl simd::Vector for ushort4 {
  type Scalar = u16;
  type Boolean = short4;

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
    return ushort4::to_char(simd::min(self, ushort4::broadcast(std::i8::MAX as u16)));
  }

  #[inline(always)]
  fn to_uchar_sat(self) -> uchar4 {
    return ushort4::to_uchar(simd::min(self, ushort4::broadcast(std::u8::MAX as u16)));
  }

  #[inline(always)]
  fn to_short_sat(self) -> short4 {
    return ushort4::to_short(simd::min(self, ushort4::broadcast(std::i16::MAX as u16)));
  }

  #[inline(always)]
  fn to_ushort_sat(self) -> ushort4 {
    return self;
  }

  #[inline(always)]
  fn to_int_sat(self) -> int4 {
    return ushort4::to_int(simd::min(self, ushort4::broadcast(std::i32::MAX as u16)));
  }

  #[inline(always)]
  fn to_uint_sat(self) -> uint4 {
    return ushort4::to_uint(self);
  }

  #[inline(always)]
  fn to_long_sat(self) -> long4 {
    return ushort4::to_long(simd::min(self, ushort4::broadcast(std::i64::MAX as u16)));
  }

  #[inline(always)]
  fn to_ulong_sat(self) -> ulong4 {
    return ushort4::to_ulong(self);
  }
}

impl simd::Dot for ushort4 {
  type DotProduct = u16;
  #[inline(always)]
  fn dot(self, other: Self) -> Self::DotProduct {
    return simd::reduce_add(self * other);
  }
}

impl simd::Integer for ushort4 {
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
    return self.reduce_and() & 0x8000 != 0;
  }

  #[inline(always)]
  fn any(self) -> bool {
    return self.reduce_or() & 0x8000 != 0;
  }
}

impl ushort4 {
  #[inline]
  pub fn bitcast<T>(x: T) -> ushort4 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn broadcast(x: u16) -> Self {
    return ushort4(x, x, x, x);
  }

  #[inline]
  pub fn lo(self) -> ushort2 {
    return ushort2(self.0, self.1);
  }

  #[inline]
  pub fn hi(self) -> ushort2 {
    return ushort2(self.2, self.3);
  }

  #[inline]
  pub fn odd(self) -> ushort2 {
    return ushort2(self.1, self.3);
  }

  #[inline]
  pub fn even(self) -> ushort2 {
    return ushort2(self.0, self.2);
  }
}
